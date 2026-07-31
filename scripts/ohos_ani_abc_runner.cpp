// Copyright (c) 2026
//
// A small OpenHarmony-side runner for ArkTS 1.2 ABC files. It creates an ANI
// VM from the system Ark runtime, loads the requested ABC through
// std.core.AbcRuntimeLinker. A tiny ArkTS launcher ABC performs the reflective
// call to the target ABC's no-argument static entry method.

#include <ani.h>
#include <dlfcn.h>

#include <cstdlib>
#include <cstdio>
#include <cstring>
#include <fstream>
#include <limits>
#include <string>
#include <vector>

namespace {

using AniCreateVm = ani_status (*)(const ani_options *, uint32_t, ani_vm **);
using AniDestructor = ani_status (*)(ani_vm *);

size_t ParsePositiveEnv(const char *name, size_t fallback)
{
    const char *value = std::getenv(name);
    if (value == nullptr || *value == '\0') {
        return fallback;
    }

    char *end = nullptr;
    unsigned long long parsed = std::strtoull(value, &end, 10);
    if (end == value || *end != '\0' || parsed == 0 ||
        parsed > std::numeric_limits<size_t>::max()) {
        std::fprintf(
            stderr, "invalid %s=%s; expected a positive integer\n", name,
            value);
        std::exit(2);
    }
    return static_cast<size_t>(parsed);
}

bool EnvFlagEnabled(const char *name)
{
    const char *value = std::getenv(name);
    return value != nullptr && *value != '\0' && std::strcmp(value, "0") != 0;
}

long ReadMemoryKb(const char *name)
{
    std::ifstream input("/proc/self/smaps_rollup");
    std::string line;
    while (std::getline(input, line)) {
        if (line.rfind(name, 0) == 0) {
            long value = -1;
            if (std::sscanf(line.c_str() + std::strlen(name), "%ld", &value) ==
                1) {
                return value;
            }
        }
    }
    return -1;
}

void PrintMemorySample(size_t iteration)
{
    std::printf(
        "ANI_MEMORY_SAMPLE iteration=%zu rss_kb=%ld pss_kb=%ld\n",
        iteration, ReadMemoryKb("Rss:"), ReadMemoryKb("Pss:"));
    std::fflush(stdout);
}

int Fail(const char *operation, ani_status status)
{
    std::fprintf(stderr, "%s failed: ani_status=%d\n", operation, static_cast<int>(status));
    return 1;
}

void *OpenArkRuntime()
{
    constexpr const char *CANDIDATES[] = {
        "libarkruntime.so",
        "/system/lib64/libarkruntime.so",
        "/system/lib/libarkruntime.so",
    };

    for (const char *candidate : CANDIDATES) {
        if (void *handle = dlopen(candidate, RTLD_NOW | RTLD_GLOBAL); handle != nullptr) {
            return handle;
        }
    }

    std::fprintf(stderr, "unable to load libarkruntime.so: %s\n", dlerror());
    return nullptr;
}

ani_status InvokeAniDestructor(ani_vm *vm, const char *library_path)
{
    void *module = nullptr;
#ifdef RTLD_NOLOAD
    module = dlopen(library_path, RTLD_NOW | RTLD_NOLOAD);
#endif
    if (module == nullptr) {
        module = dlopen(library_path, RTLD_NOW | RTLD_LOCAL);
    }
    if (module == nullptr) {
        std::fprintf(
            stderr, "unable to open ANI module for destructor check: %s\n",
            dlerror());
        return ANI_ERROR;
    }

    dlerror();
    auto destructor = reinterpret_cast<AniDestructor>(
        dlsym(module, "ANI_Destructor"));
    const char *error = dlerror();
    if (error != nullptr) {
        std::fprintf(stderr, "unable to resolve ANI_Destructor: %s\n", error);
        dlclose(module);
        return ANI_ERROR;
    }
    ani_status status = destructor(vm);
    dlclose(module);
    return status;
}

ani_status LoadApplicationClassObject(
    ani_env *env, const char *abc_path, const char *class_descriptor,
    ani_object *result)
{
#define RETURN_ON_ANI_ERROR(operation)                                           \
    do {                                                                         \
        if (status != ANI_OK) {                                                   \
            std::fprintf(                                                        \
                stderr, "%s failed while loading app ABC: ani_status=%d\n",      \
                operation, static_cast<int>(status));                             \
            return status;                                                       \
        }                                                                        \
    } while (false)

    ani_ref undefined = nullptr;
    ani_status status = env->GetUndefined(&undefined);
    RETURN_ON_ANI_ERROR("GetUndefined");

    ani_class linker_class = nullptr;
    status = env->FindClass("std.core.AbcRuntimeLinker", &linker_class);
    RETURN_ON_ANI_ERROR("FindClass(AbcRuntimeLinker)");

    ani_string abc_path_string = nullptr;
    status = env->String_NewUTF8(abc_path, std::strlen(abc_path), &abc_path_string);
    RETURN_ON_ANI_ERROR("String_NewUTF8(abc_path)");

    ani_array abc_files = nullptr;
    status = env->Array_New(1, abc_path_string, &abc_files);
    RETURN_ON_ANI_ERROR("Array_New(abc_files)");

    ani_method linker_constructor = nullptr;
    status = env->Class_FindMethod(
        linker_class, "<ctor>", "C{std.core.RuntimeLinker}C{std.core.Array}:",
        &linker_constructor);
    RETURN_ON_ANI_ERROR("Class_FindMethod(AbcRuntimeLinker.<ctor>)");

    ani_object linker = nullptr;
    status = env->Object_New(
        linker_class, linker_constructor, &linker, undefined, abc_files);
    RETURN_ON_ANI_ERROR("Object_New(AbcRuntimeLinker)");

    ani_method load_class = nullptr;
    status = env->Class_FindMethod(
        linker_class, "loadClass",
        "C{std.core.String}C{std.core.Boolean}:C{std.core.Class}", &load_class);
    RETURN_ON_ANI_ERROR("Class_FindMethod(AbcRuntimeLinker.loadClass)");

    ani_string class_name = nullptr;
    status = env->String_NewUTF8(
        class_descriptor, std::strlen(class_descriptor), &class_name);
    RETURN_ON_ANI_ERROR("String_NewUTF8(class_descriptor)");

    ani_class boolean_class = nullptr;
    status = env->FindClass("std.core.Boolean", &boolean_class);
    RETURN_ON_ANI_ERROR("FindClass(Boolean)");

    ani_method boolean_constructor = nullptr;
    status = env->Class_FindMethod(
        boolean_class, "<ctor>", "z:", &boolean_constructor);
    RETURN_ON_ANI_ERROR("Class_FindMethod(Boolean.<ctor>)");

    ani_object initialize_class = nullptr;
    status = env->Object_New(
        boolean_class, boolean_constructor, &initialize_class, ANI_FALSE);
    RETURN_ON_ANI_ERROR("Object_New(Boolean)");

    ani_ref class_object = nullptr;
    status = env->Object_CallMethod_Ref(
        linker, load_class, &class_object, class_name, initialize_class);
    RETURN_ON_ANI_ERROR("Object_CallMethod(AbcRuntimeLinker.loadClass)");

    *result = static_cast<ani_object>(class_object);
#undef RETURN_ON_ANI_ERROR
    return ANI_OK;
}

ani_status CreateApplicationInstance(
    ani_env *env, ani_object class_object, ani_class *result_class,
    ani_object *result_instance)
{
#define RETURN_ON_ANI_ERROR(operation)                                           \
    do {                                                                         \
        if (status != ANI_OK) {                                                   \
            std::fprintf(                                                        \
                stderr, "%s failed while creating launcher: ani_status=%d\n",    \
                operation, static_cast<int>(status));                             \
            return status;                                                       \
        }                                                                        \
    } while (false)

    ani_class reflection_class = nullptr;
    ani_status status = env->FindClass("std.core.Class", &reflection_class);
    RETURN_ON_ANI_ERROR("FindClass(Class)");

    ani_method create_instance = nullptr;
    status = env->Class_FindMethod(
        reflection_class, "createInstance", ":C{std.core.Object}",
        &create_instance);
    RETURN_ON_ANI_ERROR("Class_FindMethod(Class.createInstance)");

    ani_ref instance = nullptr;
    status = env->Object_CallMethod_Ref(
        class_object, create_instance, &instance);
    RETURN_ON_ANI_ERROR("Object_CallMethod(Class.createInstance)");

    ani_type type = nullptr;
    status = env->Object_GetType(static_cast<ani_object>(instance), &type);
    RETURN_ON_ANI_ERROR("Object_GetType(launcher)");

    *result_class = static_cast<ani_class>(type);
    *result_instance = static_cast<ani_object>(instance);
#undef RETURN_ON_ANI_ERROR
    return status;
}

}  // namespace

int main(int argc, char **argv)
{
    if (argc < 4 || argc > 6) {
        std::fprintf(
            stderr,
            "usage: %s <launcher.abc> <app.abc> <class-descriptor> [method=main] [native-library-path=/data/local/tmp]\n",
            argv[0]);
        return 2;
    }

    const char *launcher_abc_path = argv[1];
    const char *abc_path = argv[2];
    const char *class_descriptor = argv[3];
    const char *method_name = argc >= 5 ? argv[4] : "main";
    const char *native_library_path = argc >= 6 ? argv[5] : "/data/local/tmp";
    const size_t iterations = ParsePositiveEnv("ANI_QEMU_ITERATIONS", 1);
    if (iterations >
        static_cast<size_t>(std::numeric_limits<ani_int>::max())) {
        std::fprintf(
            stderr, "ANI_QEMU_ITERATIONS exceeds the ANI int limit\n");
        return 2;
    }
    const bool sample_memory = EnvFlagEnabled("ANI_QEMU_MEMORY_SAMPLE") ||
        std::getenv("ANI_QEMU_MEMORY_SAMPLE_EVERY") != nullptr;

    void *runtime = OpenArkRuntime();
    if (runtime == nullptr) {
        return 1;
    }

    dlerror();
    auto create_vm = reinterpret_cast<AniCreateVm>(dlsym(runtime, "ANI_CreateVM"));
    if (const char *error = dlerror(); error != nullptr) {
        std::fprintf(stderr, "unable to resolve ANI_CreateVM: %s\n", error);
        return 1;
    }

    std::string boot_files = "--ext:boot-panda-files=/system/etc/etsstdlib.abc";
    std::string library_path = "--ext:native-library-path=";
    library_path += native_library_path;

    std::vector<ani_option> option_values = {
        {boot_files.c_str(), nullptr},
        {library_path.c_str(), nullptr},
        {"--ext:verification-mode=ahead-of-time", nullptr},
        {"--ext:gc-type=g1-gc", nullptr},
    };
    if (EnvFlagEnabled("ANI_QEMU_DISABLE_JIT")) {
        option_values.push_back({"--ext:compiler-enable-jit=false", nullptr});
    }
    ani_options options = {option_values.size(), option_values.data()};

    ani_vm *vm = nullptr;
    ani_status status = create_vm(&options, ANI_VERSION_1, &vm);
    if (status != ANI_OK) {
        return Fail("ANI_CreateVM", status);
    }

    ani_env *env = nullptr;
    status = vm->GetEnv(ANI_VERSION_1, &env);
    if (status != ANI_OK) {
        vm->DestroyVM();
        return Fail("GetEnv", status);
    }

    ani_object launcher_class_object = nullptr;
    status = LoadApplicationClassObject(
        env, launcher_abc_path,
        "ohos_qemu_abc_launcher.OhosQemuLauncher",
        &launcher_class_object);
    if (status != ANI_OK) {
        env->DescribeError();
        vm->DestroyVM();
        return Fail("LoadApplicationClassObject", status);
    }

    ani_class launcher_class = nullptr;
    ani_object launcher = nullptr;
    status = CreateApplicationInstance(
        env, launcher_class_object, &launcher_class, &launcher);
    if (status != ANI_OK) {
        env->DescribeError();
        vm->DestroyVM();
        return Fail("CreateApplicationInstance", status);
    }

    ani_method invoke = nullptr;
    const bool repeated = iterations > 1;
    status = env->Class_FindMethod(
        launcher_class, repeated ? "invokeRepeated" : "invoke",
        repeated
            ? "C{std.core.String}C{std.core.String}C{std.core.String}i:"
            : "C{std.core.String}C{std.core.String}C{std.core.String}:",
        &invoke);
    if (status != ANI_OK) {
        vm->DestroyVM();
        return Fail(
            repeated ? "Class_FindMethod(launcher.invokeRepeated)"
                     : "Class_FindMethod(launcher.invoke)",
            status);
    }

    ani_string abc_path_string = nullptr;
    ani_string class_descriptor_string = nullptr;
    ani_string method_name_string = nullptr;
    status = env->String_NewUTF8(
        abc_path, std::strlen(abc_path), &abc_path_string);
    if (status == ANI_OK) {
        status = env->String_NewUTF8(
            class_descriptor, std::strlen(class_descriptor),
            &class_descriptor_string);
    }
    if (status == ANI_OK) {
        status = env->String_NewUTF8(
            method_name, std::strlen(method_name), &method_name_string);
    }
    if (status != ANI_OK) {
        vm->DestroyVM();
        return Fail("String_NewUTF8(launcher arguments)", status);
    }

    if (sample_memory) {
        PrintMemorySample(0);
    }
    if (repeated) {
        status = env->Object_CallMethod_Void(
            launcher, invoke, abc_path_string, class_descriptor_string,
            method_name_string, static_cast<ani_int>(iterations));
    } else {
        status = env->Object_CallMethod_Void(
            launcher, invoke, abc_path_string, class_descriptor_string,
            method_name_string);
    }
    if (status != ANI_OK) {
        env->DescribeError();
        vm->DestroyVM();
        return Fail(
            repeated ? "Object_CallMethod(launcher.invokeRepeated)"
                     : "Object_CallMethod(launcher.invoke)",
            status);
    }
    if (const char *destructor_library =
            std::getenv("ANI_QEMU_DESTRUCTOR_LIBRARY");
        destructor_library != nullptr && *destructor_library != '\0') {
        status = InvokeAniDestructor(vm, destructor_library);
        if (status != ANI_OK) {
            vm->DestroyVM();
            return Fail("ANI_Destructor", status);
        }
    }
    if (sample_memory) {
        PrintMemorySample(iterations);
    }

    status = vm->DestroyVM();
    if (status != ANI_OK) {
        return Fail("DestroyVM", status);
    }

    std::puts("ANI_ABC_RUNNER_OK");
    return 0;
}
