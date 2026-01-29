# Napi Functions
## 1. Version Information
### napi_get_version迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
uint32_t version;
napi_get_version(env, &version);
```
#### **ANI 示例**
```cpp
uint32_t aniVersion;
env_->GetVersion(&aniVersion);
```

### napi_get_node_version迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
const napi_node_version* version;
napi_get_node_version(env, &version);
```
#### **ANI 示例**
```cpp
uint32_t aniVersion;
env_->GetVersion(&aniVersion);
```

## 2. Class Operations

### napi_is_arraybuffer迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_is_arraybuffer 检查一个值是否为ArrayBuffer，以确保正在处理正确的数据类型。需要注意的是，此函数只能判断一个值是否为ArrayBuffer，而不能判断一个值是否为TypedArray。如果需要判断一个值是否为TypedArray，可以使用napi_is_typedarray函数。

```cpp
static napi_value IsArrayBuffer(napi_env env, napi_callback_info info)
{
    // 接受一个入参
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 调用napi_is_arraybuffer接口判断给定入参是否为ArrayBuffer数据
    bool result = false;
    napi_status status = napi_is_arraybuffer(env, args[0], &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_is_arraybuffer fail");
        return nullptr;
    }
    // 将结果转成napi_value类型返回
    napi_value returnValue = nullptr;
    napi_get_boolean(env, result, &returnValue);
    return returnValue;
}
```

#### **ANI 示例**

```cpp
ani_class arrayBufferClass;
env_->FindClass("Lescompat/ArrayBuffer;", &arrayBufferClass);
ani_boolean isArrayBuffer;
env->Object_IsInstance(obj, arrayBufferClass, &isArrayBuffer);
```

### napi_is_dataview迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value IsDataView(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // 调用napi_is_dataview接口判断给定入参是否为DataView数据。
    bool result;
    napi_status status;
    status = napi_is_dataview(env, args[0], &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_is_dataview fail");
        return nullptr;
    }
    // 将结果转成napi_value类型返回。
    napi_value returnValue;
    napi_get_boolean(env, result, &returnValue);

    return returnValue;
}
```

```ts
// index.d.ts
export const isDataView: (date: DataView) => boolean;

// ets
let buffer = new ArrayBuffer(16);
let dataView = new DataView(buffer);
let flag = testNapi.isDataView(dataView);
```

#### **ANI 示例**
```cpp
// ets
function GetDataView() {
    const buffer = new ArrayBuffer(16);
    const dataView = new DataView(buffer);
    return dataView;
}

// cpp
auto dataView = CallEtsFunction<ani_ref>("GetDataView");
ani_class dataViewClass;
env_->FindClass("Lescompat/DataView;", &dataViewClass);

ani_type typeDataView = dataViewClass;
ani_boolean isDataView;
env->Object_IsInstance(dataView, typeDataView, &dataViewClass);
```


### napi_is_date迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_is_date 在需要确定一个ArkTS对象是否为Date对象时，可使用此接口判断给定的值是否为Date对象。例如，在接收函数参数时，需要验证参数是否为Date对象以确保正确的数据类型。

```cpp
#include "napi/native_api.h"

static napi_value IsDate(napi_env env, napi_callback_info info)
{
    // 接受一个入参
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // 调用napi_is_date接口判断给定入参是否为Date数据
    bool result = false;
    napi_status status = napi_is_date(env, args[0], &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_is_date fail");
        return nullptr;
    }
    // 将结果转成napi_value类型返回
    napi_value returnValue = nullptr;
    napi_get_boolean(env, result, &returnValue);

    return returnValue;
}
```

#### **ANI 示例**

```ts
class Foo{
    static { loadLibrary("ani_date");}
    native testDate(date : Date):void;
}

function main(){
    const f = new Foo;
    f.testDate(new Date());
}
```

```cpp
#include <ani.h>
#include <array>
#include <iostream>

static void testDate([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object, ani_object date)
{
    ani_class dateCls;
    const char * className = "Lescompat/Date;";
    if (ANI_OK != env->FindClass(className, &dateCls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return ;
    }
    ani_method isDateValidMethod;
    if (ANI_OK != env->Class_FindMethod(dateCls, "isDateValid", ":Z", &isDateValidMethod)){
        std::cerr << "Class_FindMethod Failed '" << className << "'" << std::endl;
        return ;
    }

    ani_boolean isDate;
    if (ANI_OK != env->Object_CallMethod_Boolean(date, isDateValidMethod, &isDate)){
        std::cerr << "Object_CallMethod_Boolean '" << "isDateValidMethod" << "'" << std::endl;
        return ;
    }

    std::cout << std::boolalpha;
    std::cout << "isDate is: " << static_cast<bool>(isDate) << std::endl;
    return;
}

ANI_EXPORT ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    if (ANI_OK != vm->GetEnv(ANI_VERSION_1, &env)) {
        std::cerr << "Unsupported ANI_VERSION_1" << std::endl;
        return ANI_ERROR;
    }

    static const char *className = "Lani_date/Foo;";
    ani_class cls;
    if (ANI_OK != env->FindClass(className, &cls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return ANI_ERROR;
    }

    std::array methods = {
        ani_native_function {"testDate", "Lescompat/Date;:V", reinterpret_cast<void *>(testDate)},
    };
    std::cout << "Start bind native methods to '" << className << "'" << std::endl;

    if (ANI_OK != env->Class_BindNativeMethods(cls, methods.data(), methods.size())) {
        std::cerr << "Cannot bind native methods to '" << className << "'" << std::endl;
        return ANI_ERROR;
    };
    std::cout << "Finish bind native methods to '" << className << "'" << std::endl;

    *result = ANI_VERSION_1;
    return ANI_OK;
}
```

### napi_is_map迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_is_map 在需要确定一个ArkTS对象是否为Map对象时，可使用此接口判断给定的值是否为Map对象。例如，在接收函数参数时，需要验证参数是否为Map对象以确保正确的数据类型。

```cpp
#include "napi/native_api.h"

static napi_value IsMap(napi_env env, napi_callback_info info)
{
    // 接受一个入参
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // 调用napi_is_map接口判断给定入参是否为Map数据
    bool result = false;
    napi_status status = napi_is_map(env, args[0], &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_is_map fail");
        return nullptr;
    }
    // 将结果转成napi_value类型返回
    napi_value returnValue = nullptr;
    napi_get_boolean(env, result, &returnValue);

    return returnValue;
}
```

#### **ANI 示例**

```cpp
ani_class mapClass;
env_->FindClass("Lescompat/Map;", &mapClass);

ani_type typeMap = mapClass;
ani_boolean isMap;
env->Object_InstanceOf(obj, typeMap, &isMap);
```

### napi_is_arguments_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_async_function迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_big_int64_array迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_big_uint64_array迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_bitvector迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_boolean_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_generator_function迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_generator_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_map_iterator迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_module_namespace_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_number_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_proxy迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_reg_exp迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_set迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_set_iterator迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_shared_array_buffer迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_string_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_symbol_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_weak_map迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_is_weak_set迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```



### napi_is_detached_arraybuffer迁移示例
---
#### 代码示例对比

#### **N-API 示例**
判断给定的ArrayBuffer是否已被分离。
```C++
napi_value arrayBuffer = nullptr;
void* arrayBufferPtr = nullptr;
napi_create_arraybuffer(env, arrayBufferSize, &arrayBufferPtr, &arrayBuffer);

auto out = napi_detach_arraybuffer(env, arrayBuffer);
if (out == napi_ok) {
    arrayBufferPtr = nullptr;
}
ASSERT_EQ(out, napi_ok);

bool result = false;
ASSERT_CHECK_CALL(napi_is_detached_arraybuffer(env, arrayBuffer, &result));
ASSERT_TRUE(result);
```

#### **ANI 示例**
C++已经对ArrayBuffer的内存进行了管理，提高了内存的访问能力。因此不用对ArrayBuffer进行分离，也就不用查询是否已分离。




## 3. Exceptions



### napi_is_exception_pending迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
bool isExceptionPending = false;
napi_is_exception_pending(env, &isExceptionPending);
ASSERT_TRUE(isExceptionPending);
```
#### **ANI 示例**
```cpp
ani_boolean res;
ExistUnhandledError(env, &res);
ASSERT_TRUE(res);
```

### napi_get_and_clear_last_exception迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value err;
napi_get_and_clear_last_exception(env, &err);
```
#### **ANI 示例**
```cpp
ani_error error;
GetUnhandledError(env, &error);
ResetError(env);
```


### napi_extended_error_info迁移示例
#### 代码示例对比

#### **N-API 示例**
```cpp
const napi_extended_error_info *errorInfo;
napi_get_last_error_info(env, &errorInfo);
assert(errorInfo->error_code == status);
napi_value result = nullptr;
napi_create_string_utf8(env, errorInfo->error_message, NAPI_AUTO_LENGTH, &result);
```
#### **ANI 示例**
```cpp
// ets
class Operations {
    static errorThrow(a0: int): int {
        if(a0==5)
            throw new Error();
        return 1;
    }
};

// cpp
ANIEnv* env;  // 假设已获取 ANIEnv*
ani_class cls = env_->FindClass("com/example/MyClass");
if (cls == nullptr) {
    env_->DescribeError();  // 打印异常信息
    env_->ResetError();  // 清除异常，避免影响后续 ANI 调用
    return;
}
```


### napi_throw迁移示例
---
napi_throw中是创建并添加描述信息抛出一个异常。
ANI中对应的统一为ThrowError抛出异常。
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_throw(env, error);
```

#### **ANI 示例**
```cpp
ani_class errCls;
// 可以查找更多Error类型
char* className = "Lescompat/Error;";
if (ANI_OK != env->FindClass(className, &errCls)) {
    std::cerr << "Not found '"  << className << std::endl;
    return ANI_ERROR;
}

ani_object errObj;
// 查找构造函数，如果没有重载，可以直接传递 nullptr
// 有构造函数重载的情况下，请准确根据需要的函数，传入函数签名的Mangling。
ani_method errCtor;
if (ANI_OK != env->Class_FindMethod(errCls, "<ctor>", "Lstd/core/String;Lescompat/ErrorOptions;:V", &errCtor)) {
    std::cerr << "get errCtor Failed'" << className << "'" << std::endl;
    return ANI_ERROR;
}

std::string name = "This will show message!";
ani_string result_string{};
env->String_NewUTF8(name.c_str(), name.size(), &result_string);

//创建一个 Error  的实例
if (ANI_OK != env->Object_New(errCls, errCtor, &errObj, result_string)) {
    std::cerr << "Create Object Failed'" << className << "'" << std::endl;
    return ANI_ERROR;
}
env->ThrowError(static_cast<ani_error>(errObj));
```


### napi_throw_error迁移示例
---
napi_throw_error中是创建并添加描述信息抛出一个异常。
ANI中对应的统一为ThrowError抛出异常。
描述信息需要在ani_error对象创建时添加。
#### 代码示例对比

#### **N-API 示例1**
```cpp
 napi_throw_error(env, nullptr, "This will show message!");
```

#### **ANI 示例1**
```cpp
ani_class errCls;
if (ANI_OK != env->FindClass("Lstd/core/NullPointerError;", &errCls)) {
    std::cerr << "Not found '"  << "'" << std::endl;
    return ANI_ERROR;
}

ani_object errObj;
//查找构造函数，如果没有重载，可以直接传递 nullptr
ani_method errCtor;
if (ANI_OK != env->Class_FindMethod(errCls, "<ctor>", nullptr, &errCtor)) {
    std::cerr << "get errCtor Failed'" << "Lstd/core/NullPointerError;" << "'" << std::endl;
    return ANI_ERROR;
}

std::string name = "This will show message!";
ani_string result_string{};
env->String_NewUTF8(name.c_str(), name.size(), &result_string);

//创建一个 Error  的实例
if (ANI_OK != env->Object_New(errCls, errCtor, &errObj, result_string)) {
    std::cerr << "Create Object Failed'" << "Lstd/core/NullPointerError;" << "'" << std::endl;
    return ANI_ERROR;
}
env->ThrowError(static_cast<ani_error>(errObj));
```

#### **ANI 示例2**
```cpp
ani_error error;
env_->GetUnhandledError(&error);
env_->ThrowError(error);
```


### napi_fatal_error迁移示例

#### 代码示例对比

#### **N-API 示例**
```cpp
napi_fatal_error("MyFunction", NAPI_AUTO_LENGTH, "Unexpected failure!", NAPI_AUTO_LENGTH);
```
#### **ANI 示例**
```cpp
env->Abort("crash with error")
```



### napi_throw_type_error迁移示例
---
napi_throw_error中是创建并添加描述信息抛出一个异常。
ANI中对应的统一为ThrowError抛出异常，具体的error类型由创建error所选的构造函数的类来源决定。
如调用Lstd/core/NullPointerError;的构造函数构造NullPointerError类对象。
调用Lescompat/TypeError;的构造函数构造TypeError类对象。
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_throw_type_error(env, nullptr, "The argument must be a number.");
```
#### **ANI 示例**
```cpp
ani_class errCls;
if (ANI_OK != env->FindClass("Lescompat/TypeError;", &errCls)) {
    std::cerr << "Not found '"  << "'" << std::endl;
    return ANI_ERROR;
}

ani_object errObj;
//查找构造函数，如果没有重载，可以直接传递 nullptr
ani_method errCtor;
if (ANI_OK != env->Class_FindMethod(errCls, "<ctor>", nullptr, &errCtor)) {
    std::cerr << "get errCtor Failed'" << "Lescompat/TypeError;" << "'" << std::endl;
    return ANI_ERROR;
}

std::string name = "The argument must be a number.";
ani_string result_string{};
env->String_NewUTF8(name.c_str(), name.size(), &result_string);

//创建一个 Error  的实例
if (ANI_OK != env->Object_New(errCls, errCtor, &errObj, result_string)) {
    std::cerr << "Create Object Failed'" << "Lescompat/TypeError;" << "'" << std::endl;
    return ANI_ERROR;
}
env->ThrowError(static_cast<ani_error>(errObj));
```



### napi_throw_range_error迁移示例
---
napi_throw_error中是创建并添加描述信息抛出一个异常。
ANI中对应的统一为ThrowError抛出异常，具体的error类型由创建error所选的构造函数的类来源决定。
如调用Lstd/core/NullPointerError;的构造函数构造NullPointerError类对象。
调用Lescompat/TypeError;的构造函数构造TypeError类对象。
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_throw_range_error(env, nullptr, "The argument must be between 0 and 100.")
```
#### **ANI 示例**
```cpp
ani_class errCls;
if (ANI_OK != env->FindClass("Lstd/core/RangeError;", &errCls)) {
    std::cerr << "Not found '"  << "'" << std::endl;
    return ANI_ERROR;
}

ani_object errObj;
//查找构造函数，如果没有重载，可以直接传递 nullptr
ani_method errCtor;
if (ANI_OK != env->Class_FindMethod(errCls, "<ctor>", nullptr, &errCtor)){
    std::cerr << "get errCtor Failed'" << "Lstd/core/RangeError;" << "'" << std::endl;
    return ANI_ERROR;
}

std::string name = "The argument must be between 0 and 100.";
ani_string result_string{};
env->String_NewUTF8(name.c_str(), name.size(), &result_string);

//创建一个 Error  的实例
if (ANI_OK != env->Object_New(errCls, errCtor, &errObj, result_string )){
    std::cerr << "Create Object Failed'" << "Lstd/core/RangeError;" << "'" << std::endl;
    return ANI_ERROR;
}
env->ThrowError(static_cast<ani_error>(errObj));
```


### napi_get_last_error_info迁移示例
---
napi_get_last_error_info使用DescribeError进行代替。
#### **N-API 示例**
```cpp
const napi_extended_error_info *errorInfo;
napi_get_last_error_info(env, &errorInfo);
assert(errorInfo->error_code == status);
napi_value result = nullptr;
napi_create_string_utf8(env, errorInfo->error_message, NAPI_AUTO_LENGTH, &result);
```
#### **ANI 示例**
```cpp
// ets
class Operations {
    static errorThrow(a0: int): int {
        if(a0==5)
            throw new Error();
        return 1;
    }
};

// cpp
ani_env* env_;  // 假设已获取 ANIEnv*
ani_class cls = env_->FindClass("com/example/MyClass");
if (cls == nullptr) {
    env_->DescribeError('');  // 打印异常信息
    env_->ResetError();  // 清除异常，避免影响后续 ANI 调用
    return;
}
```



### napi_fatal_exception迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


### napi_is_error迁移示例
---
如果不确定对象的类型，可以使用Object_InstanceOf进行类型判断。
但是开发者应当知晓对象的类型范围，用友好的有限的搜索完成类型判断。

#### 代码示例对比

#### **N-API 示例**
```CPP
static napi_value CheckIfError(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args[1];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    bool isError = false;
    napi_is_error(env, args[0], &isError);

    napi_value result;
    napi_get_boolean(env, isError, &result);
    return result;
}
```
#### **ANI 示例**
```CPP
ani_boolean CheckIfError(ani_env *env,[[maybe_unused]] ani_object obj, ani_object value)
{
    ani_class cls;
    env->FindClass("Lescompat/Error;", &cls); // Lescompat/Error;是Error的基类

    ani_type typeError = cls;
    ani_boolean result;
    env->Object_InstanceOf(value, typeError, &result);
    return result;
}
```


### napi_extended_error_info迁移示例
---
#### 代码示例对比

#### **N-API 示例**

```CPP
// napi定义的错误类型

typedef struct {
  const char* error_message;
  void* engine_reserved;
  uint32_t engine_error_code;
  napi_status error_code;
} napi_extended_error_info;
```

#### **ANI 示例**
```CPP
// ani自定义引用类型：

// 假设 __ani_ref 和 __ani_object 已经定义
class __ani_ref {};
class __ani_object : public __ani_ref {};

// 定义 __ani_error 类
class __ani_error : public __ani_object {
public:
    int code;           // 错误码
    const char* message; // 错误消息

    // 构造函数
    __ani_error(int errorCode, const char* errorMessage)
        : code(errorCode), message(errorMessage) {}

    // 其他成员函数（如需要）
};

// 定义 ani_error 类型
typedef __ani_error* ani_error;
```



## 4. Global and Local References

### napi_create_reference迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value obj = nullptr;
napi_ref ref = nullptr;
napi_create_object(env, &obj);
napi_create_reference(env, result, 1, &ref);
```
#### **ANI 示例**
```cpp
ani_ref objectRef;
ani_ref objectGRef;
env_->String_NewUTF8("x", 1, reinterpret_cast<ani_string *>(&objectRef);
env_->GlobalReference_Create(objectRef, &objectGRef);
```


### napi_delete_reference迁移示例

#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value result = nullptr;
napi_ref resultRef = nullptr;
napi_create_object(env, &result);
napi_create_reference(env, result, 1, &resultRef);
napi_delete_reference(env, resultRef);
```
#### **ANI 示例**
```cpp
ani_ref objectRef;
env_->String_NewUTF8("x", 1, reinterpret_cast<ani_string *>(&objectRef));
ani_ref objectGRef;
env_->GlobalReference_Create(objectRef, &objectGRef);

env_->GlobalReference_Delete(objectGRef);
```


### napi_strict_equals迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```CPP
napi_strict_equals 用于 检查两个 JavaScript 值是否严格相等，它的行为等同于 JavaScript 中的 ===（严格相等运算符）。

napi_status napi_strict_equals(napi_env env, napi_value lhs, napi_value rhs, bool* result);
入参：
env：N-API 执行环境
lhs：左侧的 JavaScript 值
rhs：右侧的 JavaScript 值
result：指向 bool 的指针，存储比较结果（true 或 false）

返回值：
napi_ok（成功）
其他错误码（如果 env、lhs 或 rhs 无效）


示例：
#include <node_api.h>

// 函数：检查两个参数是否严格相等
napi_value IsStrictEqual(napi_env env, napi_callback_info info) {
    size_t argc = 2;
    napi_value args[2];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    if (argc < 2) {
        napi_throw_type_error(env, nullptr, "Expected two arguments");
        return nullptr;
    }

    bool result;
    napi_strict_equals(env, args[0], args[1], &result);

    napi_value js_result;
    napi_get_boolean(env, result, &js_result);
    return js_result;
}

// 初始化模块
napi_value Init(napi_env env, napi_value exports) {
    napi_property_descriptor desc = { "strictEquals", 0, IsStrictEqual, 0, 0, 0, napi_default, 0 };
    napi_define_properties(env, exports, 1, &desc);
    return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)


JS调用：
const addon = require('./build/Release/addon');

console.log(addon.strictEquals(5, 5));        // true
console.log(addon.strictEquals(5, "5"));      // false
console.log(addon.strictEquals(null, null));  // true
console.log(addon.strictEquals({}, {}));      // false (不同对象)
const obj = {};
console.log(addon.strictEquals(obj, obj));    // true (同一对象)


```


#### **ANI 示例**
```CPP
ani侧对应的接口是Reference_StrictEquals。

示例：

sts侧：
function GetNull(): null {
    return null;
}

function GetUndefined(): undefined {
    return undefined;
}

function GetObject(): Object {
    return new String("Hello World!");
}


cpp侧：
#include "ani_gtest.h"

namespace ark::ets::ani::testing {

class ReferenceStrictEqualsTest : public AniTest {};

TEST_F(ReferenceStrictEqualsTest, check_null_and_null)
{
    auto nullRef1 = CallEtsFunction<ani_ref>("GetNull");
    auto nullRef2 = CallEtsFunction<ani_ref>("GetNull");
    ani_boolean isEquals;
    ASSERT_EQ(env_->Reference_StrictEquals(nullRef1, nullRef2, &isEquals), ANI_OK);
    ASSERT_EQ(isEquals, ANI_TRUE);
}

TEST_F(ReferenceStrictEqualsTest, check_null_and_undefined)
{
    auto nullRef = CallEtsFunction<ani_ref>("GetNull");
    auto undefinedRef = CallEtsFunction<ani_ref>("GetUndefined");
    ani_boolean isEquals;
    ASSERT_EQ(env_->Reference_StrictEquals(nullRef, undefinedRef, &isEquals), ANI_OK);
    ASSERT_EQ(isEquals, ANI_FALSE);
}

TEST_F(ReferenceStrictEqualsTest, check_null_and_object)
{
    auto nullRef = CallEtsFunction<ani_ref>("GetNull");
    auto objectRef = CallEtsFunction<ani_ref>("GetObject");
    ani_boolean isEquals;
    ASSERT_EQ(env_->Reference_StrictEquals(nullRef, objectRef, &isEquals), ANI_OK);
    ASSERT_EQ(isEquals, ANI_FALSE);
}

TEST_F(ReferenceStrictEqualsTest, check_undefined_and_undefined)
{
    auto undefinedRef1 = CallEtsFunction<ani_ref>("GetUndefined");
    auto undefinedRef2 = CallEtsFunction<ani_ref>("GetUndefined");
    ani_boolean isEquals;
    ASSERT_EQ(env_->Reference_StrictEquals(undefinedRef1, undefinedRef2, &isEquals), ANI_OK);
    ASSERT_EQ(isEquals, ANI_TRUE);
}

TEST_F(ReferenceStrictEqualsTest, check_undefined_and_object)
{
    auto undefinedRef = CallEtsFunction<ani_ref>("GetUndefined");
    auto objectRef = CallEtsFunction<ani_ref>("GetObject");
    ani_boolean isEquals;
    ASSERT_EQ(env_->Reference_StrictEquals(undefinedRef, objectRef, &isEquals), ANI_OK);
    ASSERT_EQ(isEquals, ANI_FALSE);
}

TEST_F(ReferenceStrictEqualsTest, check_object_and_object)
{
    auto objectRef1 = CallEtsFunction<ani_ref>("GetObject");
    auto objectRef2 = CallEtsFunction<ani_ref>("GetObject");
    ani_boolean isEquals;
    ASSERT_EQ(env_->Reference_StrictEquals(objectRef1, objectRef2, &isEquals), ANI_OK);
    ASSERT_EQ(isEquals, ANI_TRUE);
}

TEST_F(ReferenceStrictEqualsTest, invalid_argument)
{
    auto ref = CallEtsFunction<ani_ref>("GetNull");
    ASSERT_EQ(env_->Reference_StrictEquals(ref, ref, nullptr), ANI_INVALID_ARGS);
}

}  // namespace ark::ets::ani::testing

```

#### **ANI 示例2**
在CPP层对于ETS传入的函数对象进行比较是否为同一个对象。
https://gitee.com/openharmony/arkcompiler_runtime_core/issues/IBPY1I

```TS
native function handleData(a: ()=>int, b :()=>int):boolean

function main(){
    loadLibrary("ani_test")
    let f = () => { return 1;}
    let f2 = () => { return 1;}
    let a = handleData(f,f);
    let b = handleData(f,f2);
    let c = handleData(foo,foo);
    let z = foo;
    let d = handleData(z,z);
    console.log(a,b,c,d); // true false false true
}
```

```CPP
static ani_boolean handleData(ani_env *env, ani_object obj, ani_object funcObj1, ani_object funcObj2)
{
    auto ref1 = static_cast<ani_ref>(funcObj1);
    auto ref2 = static_cast<ani_ref>(funcObj2);
    ani_boolean result;
    env->Reference_StrictEquals(ref1,ref2,&result);
    return result;
}

ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    vm->GetEnv(ANI_VERSION_1, &env);
    static const char *className = "Lani_test/ETSGLOBAL;";
    ani_class cls;
    ANI_OK != env->FindClass(className, &cls);
    std::array methods = {
    //函数签名Function0会根据参数返回值的数量进行变化，请反编译确认当前具体的FunctionX
    ani_native_function{"handleData", "Lstd/core/Function0;Lstd/core/Function0;:Z", reinterpret_cast<void *>(handleData)},
    };
    env->Class_BindNativeMethods(cls, methods.data(), methods.size());
    *result = ANI_VERSION_1;
    return ANI_OK;
}
```


### napi_reference_ref迁移示例
---
#### 代码示例对比

#### **N-API 示例**
增加传入的reference的引用计数，并获取该计数。
```cpp
// cpp
napi_ref g_ref;

static napi_value ReferenceRef(napi_env env, napi_callback_info info)
{
    napi_value obj = nullptr;
    napi_create_object(env, &obj);

    // 创建对ArkTS对象的引用
    napi_status status = napi_create_reference(env, obj, 1, &g_ref);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "napi_create_reference fail");
        return nullptr;
    }
    // 增加传入引用的引用计数并返回生成的引用计数
    uint32_t result = 0;
    napi_reference_ref(env, g_ref, &result);
    OH_LOG_INFO(LOG_APP, "napi_reference_ref, count = %{public}d.", result);
    if (result != 2) {
        // 若传入引用的引用计数未增加，则抛出错误
        napi_throw_error(env, nullptr, "napi_reference_ref fail");
        return nullptr;
    }
    return obj;
}
```

#### **ANI 示例**
ANI 的 References 不支持引用计数。

### napi_reference_unref迁移示例
---
#### 代码示例对比

#### **N-API 示例**
减少传入的reference的引用计数，并获取该计数。
```cpp
// cpp
napi_ref g_ref;

static napi_value ReferenceUnref(napi_env env, napi_callback_info info)
{
    // 减少传入引用的引用计数并返回生成的引用计数
    uint32_t result = 0;
    napi_reference_unref(env, g_ref, &result);
    OH_LOG_INFO(LOG_APP, "napi_reference_ref, count = %{public}d.", result);
    if (result != 1) {
        // 若传入引用的引用计数未减少，则抛出错误
        napi_throw_error(env, nullptr, "napi_reference_unref fail");
        return nullptr;
    }

    return nullptr;
}
```

#### **ANI 示例**
ANI 的 References 不支持引用计数。

## 5. Weak Global References

## 6. Object Operations

### napi_instanceof迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```
napi_instanceof 用于检查 一个 JavaScript 值是否是某个构造函数的实例，类似于 JavaScript 中的 instanceof 运算符。

napi_status napi_instanceof(napi_env env, napi_value object, napi_value constructor, bool* result);
入参：
env：N-API 执行环境
object：要检查的 JavaScript 对象（即 obj instanceof Constructor 中的 obj）
constructor：构造函数（即 obj instanceof Constructor 中的 Constructor）
result：bool* 指针，用于存储检查结果（true 或 false）

返回值：
napi_ok（成功），其他错误码（如果 env、object 或 constructor 无效）

示例：
#include <node_api.h>
#include <stdio.h>

// 构造函数引用（全局变量）
napi_ref MyClassConstructorRef = nullptr;

// 检查是否为 MyClass 的实例
napi_value IsInstance(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args[1];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    if (argc < 1) {
        napi_throw_type_error(env, nullptr, "Expected one argument");
        return nullptr;
    }

    // 获取构造函数对象
    napi_value constructor;
    napi_get_reference_value(env, MyClassConstructorRef, &constructor);

    // 进行 instanceof 检查
    bool result;
    napi_instanceof(env, args[0], constructor, &result);

    // 返回布尔值
    napi_value js_result;
    napi_get_boolean(env, result, &js_result);
    return js_result;
}

// MyClass 构造函数
napi_value MyClassConstructor(napi_env env, napi_callback_info info) {
    napi_value this_arg;
    napi_get_cb_info(env, info, nullptr, nullptr, &this_arg, nullptr);
    return this_arg;
}

// 初始化模块
napi_value Init(napi_env env, napi_value exports) {
    napi_value constructor;

    // 定义 MyClass 构造函数
    napi_property_descriptor desc[] = {
        { "isInstance", 0, IsInstance, 0, 0, 0, napi_default, 0 }
    };

    napi_define_class(env, "MyClass", NAPI_AUTO_LENGTH, MyClassConstructor, nullptr, 0, nullptr, &constructor);

    // 创建构造函数引用
    napi_create_reference(env, constructor, 1, &MyClassConstructorRef);

    // 将构造函数导出
    napi_set_named_property(env, exports, "MyClass", constructor);
    napi_define_properties(env, exports, 1, desc);

    return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)


JS侧调用：
const addon = require('./build/Release/addon');

const obj1 = new addon.MyClass();
console.log(addon.isInstance(obj1));  // true

const obj2 = {};
console.log(addon.isInstance(obj2));  // false
```

#### **ANI 示例**
```
ani侧对应的接口是Object_InstanceOf。

示例：
sts侧：
class A {
    public static new_A(): A {
        return new A();
    }

    boolean_method(a0: int, a1: int): boolean {
        if (a0 + a1 > 10) {
            return true;
        } else {
            return false;
        }
    }
}

class B extends A {
    public static new_B(): B {
        return new B();
    }

    boolean_method(a0: int, a1: int): boolean {
        if (a0*a1 > 10) {
            return true;
        } else {
            return false;
        }
    }
}

class C extends B {
    public static new_C(): C {
        return new C();
    }

    boolean_method(a0: int, a1: int): boolean {
        if (a0 - a1 > 10) {
            return true;
        } else {
            return false;
        }
    }
}

class D {
    public static new_D(): D {
        return new D();
    }

    boolean_method(a0: int): boolean {
        if (a0 > 10) {
            return true;
        } else {
            return false;
        }
    }
}

cpp侧：
#include "ani_gtest.h"
// NOLINTBEGIN(cppcoreguidelines-pro-type-vararg)
namespace ark::ets::ani::testing {

/**
 * @brief Unit test class for testing boolean method calls on ani objects.
 *
 * Inherits from the AniTest base class and provides test cases to verify
 * correct functionality of calling boolean-returning methods with various
 * parameter scenarios.
 */
class ObjectInstanceOfTest : public AniTest {
public:
    void GetMethodData(ani_object *objectResult, ani_class *classResult, const char *className,
                       const char *newClassName, const char *signature)
    {
        ani_class cls;
        // Locate the class in the environment.
        ASSERT_EQ(env_->FindClass(className, &cls), ANI_OK);
        ASSERT_NE(cls, nullptr);

        // Emulate allocation an instance of class.
        ani_static_method newMethod;
        ASSERT_EQ(env_->Class_GetStaticMethod(cls, newClassName, signature, &newMethod), ANI_OK);
        ani_ref ref;
        ASSERT_EQ(env_->Class_CallStaticMethod_Ref(cls, newMethod, &ref), ANI_OK);

        *objectResult = static_cast<ani_object>(ref);
        *classResult = cls;
    }
};

/**
 * @brief Test case for calling a boolean-returning method with an argument array.
 *
 * This test verifies the correct behavior of calling a method using an array
 * of integer arguments and checks the return value.
 */
TEST_F(ObjectInstanceOfTest, object_instance_of)
{
    ani_object objectA;
    ani_class classA;
    GetMethodData(&objectA, &classA, "LA;", "new_A", ":LA;");

    ani_object objectB;
    ani_class classB;
    GetMethodData(&objectB, &classB, "LB;", "new_B", ":LB;");

    ani_object objectC;
    ani_class classC;
    GetMethodData(&objectC, &classC, "LC;", "new_C", ":LC;");

    ani_object objectD;
    ani_class classD;
    GetMethodData(&objectD, &classD, "LD;", "new_D", ":LD;");

    ani_type typeRefC = classC;
    ani_type typeRefA = classA;
    ani_boolean res;

    ASSERT_EQ(env_->Object_InstanceOf(objectC, typeRefC, &res), ANI_OK);
    ASSERT_EQ(res, ANI_TRUE);

    ASSERT_EQ(env_->Object_InstanceOf(objectB, typeRefC, &res), ANI_OK);
    ASSERT_EQ(res, false);

    ASSERT_EQ(env_->Object_InstanceOf(objectC, typeRefA, &res), ANI_OK);
    ASSERT_EQ(res, ANI_TRUE);

    ASSERT_EQ(env_->Object_InstanceOf(objectD, typeRefA, &res), ANI_OK);
    ASSERT_EQ(res, ANI_FALSE);

    ASSERT_EQ(env_->Object_InstanceOf(nullptr, typeRefA, &res), ANI_INVALID_ARGS);

    ASSERT_EQ(env_->Object_InstanceOf(objectC, nullptr, &res), ANI_INVALID_ARGS);
}

}  // namespace ark::ets::ani::testing
```


### napi_typeof迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_typeof 在处理传入的ArkTS值时，可以使用这个接口来获取其类型，以便进行相应的处理。
```cpp
#include "napi/native_api.h"

static napi_value NapiTypeOf(napi_env env, napi_callback_info info)
{
    // 接受一个入参
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // 调用napi_typeof判断传入ArkTS参数类型
    napi_valuetype valueType;
    napi_status status = napi_typeof(env, args[0], &valueType);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_typeof fail");
        return nullptr;
    }
    // 将结果转成napi_value类型返回。
    napi_value returnValue = nullptr;
    switch(valueType) {
    case napi_undefined:
        napi_create_string_utf8(env, "Input type is napi_undefined", NAPI_AUTO_LENGTH, &returnValue);
        break;
    case napi_null:
        napi_create_string_utf8(env, "Input type is napi_null", NAPI_AUTO_LENGTH, &returnValue);
        break;
    case napi_boolean:
        napi_create_string_utf8(env, "Input type is napi_boolean", NAPI_AUTO_LENGTH, &returnValue);
        break;
    case napi_number:
        napi_create_string_utf8(env, "Input type is napi_number", NAPI_AUTO_LENGTH, &returnValue);
        break;
    case napi_string:
        napi_create_string_utf8(env, "Input type is napi_string", NAPI_AUTO_LENGTH, &returnValue);
        break;
    case napi_object:
        napi_create_string_utf8(env, "Input type is napi_object", NAPI_AUTO_LENGTH, &returnValue);
        break;
    case napi_function:
        napi_create_string_utf8(env, "Input type is napi_function", NAPI_AUTO_LENGTH, &returnValue);
        break;
    case napi_bigint:
        napi_create_string_utf8(env, "Input type is napi_bigint", NAPI_AUTO_LENGTH, &returnValue);
        break;
    default:
        napi_create_string_utf8(env, "unknown", NAPI_AUTO_LENGTH, &returnValue);
    }

    return returnValue;
}
```

#### **ANI 示例**

```cpp
ani_string result = nullptr;
auto status = env_->String_NewUTF8("a", 1U, &result);
ASSERT_EQ(status, ANI_OK);
ASSERT_NE(result, nullptr);

ani_type type;
ani_boolean res;
ASSERT_EQ(env_->Object_GetType(result, &type), ANI_OK);
ASSERT_EQ(env_->Object_InstanceOf(result, type, &res), ANI_OK);
ASSERT_EQ(res, ANI_TRUE);
```

### napi_module_register迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_module_register()
```
#### **ANI 示例**
```cpp
ani_module module;
env_->FindModule("L@abcModule/test;", &module);

const char *concatSignature = "Lstd/core/String;Lstd/core/String;:Lstd/core/String;";
std::array functions = {
    ani_native_function {"sum", "II:I", reinterpret_cast<void *>(Sum)},
    ani_native_function {"concat", concatSignature, reinterpret_cast<void *>(Concat)},
};
env_->Module_BindNativeFunctions(module, functions.data(), functions.size());
}
```

#### **ANI 示例2**

```cpp
//ets -> propertyError.abc
loadLibrary("propertyError");

export namespace inputConsumer {
    export interface KeyOptions {
        finalKey : number;
        finalKeyDownDuration : number;
        isFinalKeyDown: boolean;
    }
    export type CloudSyncCallback = (data : KeyOptions) => void;
    export native function on(type: string, keyOptions:KeyOptions, callback : CloudSyncCallback):void;
}

function main(){
    
    let ko : inputConsumer.KeyOptions = {
        finalKey : 6,
        finalKeyDownDuration : 1000,
        isFinalKeyDown : true
    }
    let cb : inputConsumer.CloudSyncCallback = (data : inputConsumer.KeyOptions) => {
            console.println("callback data");
        }
    inputConsumer.on("key",ko, cb);
}

//cpp -> libpropertyError.so
#include <ani.h>
#include <array>
#include <iostream>

static void on([[maybe_unused]] ani_env *env, ani_string a, ani_object b, [[maybe_unused]] ani_object callBack)
{
    ani_double finalKeyDownDuration;
    std::cout << a << std::endl;
    if (ANI_OK != env->Object_GetPropertyByName_Double(b, "finalKeyDownDuration",&finalKeyDownDuration)) {
        std::cerr << "call function '<get>finalKeyDownDuration' failed" << std::endl;
    }

    std::cout << finalKeyDownDuration << std::endl;
}
ANI_EXPORT ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    if (ANI_OK != vm->GetEnv(ANI_VERSION_1, &env)) {
        std::cerr << "Unsupported ANI_VERSION_1" << std::endl;
        return (ani_status)9;
    }

    static const char *nsName = "LpropertyError/inputConsumer;";
    ani_namespace ns {};
    if (ANI_OK != env->FindNamespace(nsName, &ns)) {
        std::cerr << "Not found '" << nsName << "'" << std::endl;
        return (ani_status)2;
    }

    const char *onSignature = "Lstd/core/String;LpropertyError/inputConsumer/KeyOptions;Lstd/core/Function1;:V";
    std::array methods = {
        ani_native_function {"on", onSignature, reinterpret_cast<void *>(on)},
    };
    if (ANI_OK != env->Namespace_BindNativeFunctions(ns, methods.data(), methods.size())) {
        std::cerr << "Cannot bind native methods to '" << nsName << "'" << std::endl;
        return (ani_status)3;
    };

    *result = ANI_VERSION_1;
    return ANI_OK;
}
```
#### **ANI 示例3**
```cpp
//ets
class batterInfoInner {
    property1 : string;
    native  property1Setter(a: string) : void;
}

//cpp
static void property1Setter([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object, ani_string string){
    // todo with object
}
ANI_EXPORT ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    if (ANI_OK != vm->GetEnv(ANI_VERSION_1, &env)) {
        std::cerr << "Unsupported ANI_VERSION_1" << std::endl;
        return ANI_ERROR;
    }

    static const char *className = "LbatterInfoInner;";
    ani_class cls;
    if (ANI_OK != env->FindClass(className, &cls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return ANI_ERROR;
    }

    std::array methods = {
        ani_native_function {"property1Setter", "Lstd/core/stirng;:V", reinterpret_cast<void *>(property1Setter)},
    };
    std::cout << "Start bind native methods to '" << className << "'" << std::endl;

    if (ANI_OK != env->Class_BindNativeMethods(cls, methods.data(), methods.size())) {
        std::cerr << "Cannot bind native methods to '" << className << "'" << std::endl;
        return ANI_ERROR;
    };
    std::cout << "Finish bind native methods to '" << className << "'" << std::endl;

    *result = ANI_VERSION_1;
    return ANI_OK;
}
```

#### **ANI 示例4**
```cpp
// 重载函数的绑定
// ets
class Want {}
class StartOptions {}
class AsyncCallback {}
namespace ops {
function startAbility(want: Want, callback: AsyncCallback): void{
    // xxx
}
function startAbility(want: Want, options: StartOptions, callback: AsyncCallback): void{
    // xxx
}
}

// cpp
ani_namespace ns {};
env_->FindNamespace("Lops;", &ns);
ani_function fn1 {};
ani_function fn2 {};
env_->Namespace_FindFunction(ns, "startAbility", "LWant;LStartOptions:V", &fn1);
env_->Namespace_FindFunction(ns, "startAbility", "LWant;LStartOptions;LAsyncCallback;:V", &fn2);
```

### napi_create_function迁移示例
---
ets层声明native函数，显示声明参数类型和返回值类型。并调用loadLibrary。
cpp层实现native函数，调用loadLibrary将会自动执行ANI_Constructor，在其中用Class_BindNativeMethods进行绑定。

#### 代码示例对比

#### **N-API 示例**
```CPP
#include <napi/native_api.h>

// 定义一个 C/C++ 函数，计算矩形面积
static napi_value CalculateArea(napi_env env, napi_callback_info info) {
    size_t argc = 2; // 参数数量
    napi_value args[2] = {nullptr}; // 参数数组
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    double width = 0;
    double height = 0;

    // 获取参数值
    napi_get_value_double(env, args[0], &width);
    napi_get_value_double(env, args[1], &height);

    // 计算面积
    napi_value area = nullptr;
    napi_create_double(env, width * height, &area);

    return area;
}

// 初始化模块，将函数暴露给 JavaScript
EXTERN_C_START
static napi_value Init(napi_env env, napi_value exports) {
    napi_value fn = nullptr;
    // 创建 JavaScript 函数
    napi_create_function(env, nullptr, 0, CalculateArea, nullptr, &fn);
    // 将函数添加到 exports 对象中
    napi_set_named_property(env, exports, "calculateArea", fn);

    return exports;
}
EXTERN_C_END
```

#### **ANI 示例**

```TS
native function handleData(a: ()=>int, b :()=>int):boolean

function main(){
    loadLibrary("ani_test")
    let f = () => { return 1;}
    let f2 = () => { return 1;}
    let a = handleData(f,f);
    let b = handleData(f,f2);
    let c = handleData(foo,foo);
    let z = foo;
    let d = handleData(z,z);
    console.log(a,b,c,d); // true false false true
}
```

```CPP
static ani_boolean handleData(ani_env *env, ani_object obj, ani_object funcObj1, ani_object funcObj2)
{
    auto ref1 = static_cast<ani_ref>(funcObj1);
    auto ref2 = static_cast<ani_ref>(funcObj2);
    ani_boolean result;
    env->Reference_StrictEquals(ref1,ref2,&result);
    return result;
}

ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    vm->GetEnv(ANI_VERSION_1, &env);
    static const char *className = "Lani_test/ETSGLOBAL;";
    ani_class cls;
    ANI_OK != env->FindClass(className, &cls);
    std::array methods = {
    //函数签名Function0会根据参数返回值的数量进行变化，请反编译确认当前具体的FunctionX
    ani_native_function{"handleData", "Lstd/core/Function0;Lstd/core/Function0;:Z", reinterpret_cast<void *>(handleData)},
    };
    env->Class_BindNativeMethods(cls, methods.data(), methods.size());
    *result = ANI_VERSION_1;
    return ANI_OK;
}
```

### napi_new_instance迁移示例

#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value CreateObject(napi_env env, napi_value constructor, size_t argc, napi_value* args) {
    napi_value result;
    napi_status status = napi_new_instance(env, constructor, argc, args, &result);
    if (status != napi_ok) return nullptr;
    return result;
}
```
#### **ANI 示例**
```cpp
// ets
class MobilePhone {
    constructor(model: String, weight: int) {
        this.model = model;
        this.weight = weight;
    }
    model: String;
    weight: int;
}

function checkModel(p: MobilePhone, model: String): boolean {
    return p.model == model;
}

function checkWeight(p: MobilePhone, weight: int): boolean {
    return p.weight == weight;
}
// cpp
void GetTestData(ani_class *clsResult, ani_method *ctorResult, ani_string *modelResult, ani_int *weightResult)
{
    const char m[] = "Pure P60";
    const ani_int weight = 200;

    ani_class cls;
    ASSERT_EQ(env_->FindClass("LMobilePhone;", &cls), ANI_OK);

    ani_method ctor;
    ASSERT_EQ(env_->Class_GetMethod(cls, "<ctor>", "Lstd/core/String;I:V", &ctor), ANI_OK);

    ani_string model;
    ASSERT_EQ(env_->String_NewUTF8(m, strlen(m), &model), ANI_OK);

    *clsResult = cls;
    *ctorResult = ctor;
    *modelResult = model;
    *weightResult = weight;
}

ani_class cls;
ani_method ctor;
ani_string model;
ani_int weight;
GetTestData(&cls, &ctor, &model, &weight);

ani_object phone;
ASSERT_EQ(env_->Object_New(cls, ctor, &phone, model, weight), ANI_OK);

ASSERT_EQ(CallEtsFunction<ani_boolean>("checkModel", phone, model), ANI_TRUE);
ASSERT_EQ(CallEtsFunction<ani_boolean>("checkWeight", phone, weight), ANI_TRUE);

```



### napi_coerce_to_bool迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value CoerceToBool(napi_env env, napi_callback_info info)
{
    // 获取并解析传进的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 将传入的值转换为布尔值
    napi_value result = nullptr;
    napi_coerce_to_bool(env, args[0], &result);
    //返回强转之后的ArkTS boolean值
    return result;
}
```

```ts
// index.d.ts
export const coerceToBool: (data: number) => boolean;

// ets
let value = testNapi.coerceToBool(0);
```

#### **ANI 示例**
强制类型转换，可通过自身逻辑实现。例如将number转为boolean值：
```cpp
// sts
function GetNumber() {
    let num : number = 0;
    return num;
}

// cpp
    ani_double number = CallEtsFunction<ani_double>("GetNumber");
    ani_boolean flag = number > 0 ? true : false ;
```

### napi_coerce_to_number迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
static napi_value CoerceToNumber(napi_env env, napi_callback_info info)
{
    // 获取并解析传进的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 将传入的值转换为number值
    napi_value result = nullptr;
    napi_coerce_to_number(env, args[0], &result);
    return result;
}
```

#### **ANI 示例**
强制类型转换，可通过自身逻辑实现。例如将boolean转为number值：
```cpp
// sts
function GetBoolean() {
    let num : boolean = false;
    return num;
}

// cpp
    ani_boolean flag = CallEtsFunction<ani_boolean>("GetBoolean");
    ani_int number = flag ? 1 : 0;
```

### napi_coerce_to_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value CoerceToObject(napi_env env, napi_callback_info info)
{
    // 获取并解析传进的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_value obj = nullptr;
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 将传入的值转换为Object值
    napi_coerce_to_object(env, args[0], &obj);
    return obj;
}
```

```ts
// index.d.ts
export const coerceToObject: (data: number) => Object;

// ets
let value = testNapi.coerceToObject(1);
```

#### **ANI 示例**
强制类型转换，可通过自身逻辑实现。例如将number转为object值：
```cpp
// sts
class NumClass {
    constructor(num: int) {
        this.num = num;
    }
    num: int;
}

function GetNumber() {
    let num : int = 1;
    return num;
}

// cpp
    ani_int number = CallEtsFunction<ani_int>("GetNumber");

    ani_class cls;
    env_->FindClass("LNumClass;", &cls);

    ani_method ctor;
    env_->Class_GetMethod(cls, "<ctor>", "I:V", &ctor);

    ani_object num;
    env_->c_api->Object_New(env_, cls, ctor, &num, number);
```

### napi_coerce_to_string迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value CoerceToString(napi_env env, napi_callback_info info)
{
    // 获取并解析传进的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 将传入的值转换为string
    napi_value str = nullptr;
    napi_coerce_to_string(env, args[0], &str);
    return str;
}
```

```ts
// index.d.ts
export const coerceToString: (data: bigint) => string;

// ets
let value = BigInt(-9223372036854775807n);
let str = testNapi.coerceToString(value);
```


#### **ANI 示例**
强制类型转换，可通过自身逻辑实现。例如将bigint转为object值：
```cpp
// sts
function GetBigint() {
    let num : bigint = -9223372036854775807n;
    return num;
}

// cpp
    auto bigintRef = CallEtsFunction<ani_ref>("GetBigint");
    ani_object bigintNum = static_cast<ani_object>(bigintRef);
    ani_class bigIntCls;
    const char * className = "Lescompat/BigInt;";
    if (ANI_OK != env_->FindClass(className, &bigIntCls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return ;
    }
    ani_method getLongMethod;
    if (ANI_OK != env_->Class_GetMethod(bigIntCls, "toString", ":Lstd/core/String;", &getLongMethod)){
        std::cerr << "Class_GetMethod Failed '" << className << "'" << std::endl;
        return ;
    }

    ani_ref strRef;
    if (ANI_OK != env_->Object_CallMethod_Ref(bigintNum, getLongMethod, &strRef)){
        std::cerr << "Object_CallMethod_Long '" << "getLongMethod" << "'" << std::endl;
        return ;
    }
    auto str = static_cast<ani_string>(strRef);
    const uint32_t bufferSize = 30;
    char utfBuffer[bufferSize];
    ani_size size;
    env_->String_GetUTF8(str, utfBuffer, bufferSize, &size);
    std::cout << "num value is : '" << utfBuffer << "'" << std::endl;
```



### napi_coerce_to_native_binding_object迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**





### napi_is_array迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value array = nullptr;
napi_create_sendable_array(env, &array);
bool isArray = false;
napi_is_array(env, array, &isArray);
```
#### **ANI 示例**
```cpp
ani_class arrayClass;
env_->FindClass("Lescompat/Array;", &arrayClass);
ani_boolean isArray;
env->Object_IsInstance(obj, arrayClass, &isArray);
```

### napi_is_typedarray迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value IsTypedarray(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 调用napi_is_typedarray接口判断给定入参类型是否为TypedArray。
    bool result = false;
        napi_status status;
    status = napi_is_typedarray(env, args[0], &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_is_typedarray fail");
        return nullptr;
    }
    // 将结果转成napi_value类型返回。
    napi_value returnValue = nullptr;
    napi_get_boolean(env, result, &returnValue);

    return returnValue;
}
```

```ts
// index.d.ts
export const isTypedarray: (data: Object) => boolean;

// cpp
let value = new Uint8Array([1, 2, 3, 4]);
let flag = testNapi.isTypedarray(value);
```

#### **ANI 示例**
```cpp
// sts
function GetTypedArray() {
    const buffer = new ArrayBuffer(16);
    const uint8Array = new Uint8Array(buffer);
    return uint8Array;
}

// cpp
auto typedArray = CallEtsFunction<ani_ref>("GetTypedArray");
ani_class cls;
env_->FindClass("Lescompat/Uint8Array;", &cls);

ani_type typeUint8Array = cls;
ani_boolean isUint8Array;
env->Object_IsInstance(typedArray, typeUint8Array, &isUint8Array);
```


### napi_create_typedarray迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_create_typedarray用于在Node-API模块中通过现有的ArrayBuffer创建指定类型的ArkTS TypedArray。
cpp部分代码
```C++
static napi_value CreateTypedArray(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    int32_t typeNum = 0;
    napi_get_value_int32(env, args[0], &typeNum);
    napi_typedarray_type arrayType;
    // 用于存储每个元素的大小
    size_t elementSize = 0;
    // 根据传递的类型值选择创建对应的类型数组
    arrayType = static_cast<napi_typedarray_type>(typeNum);
        switch (typeNum) {
    case napi_int8_array:
    case napi_uint8_array:
    case napi_uint8_clamped_array:
        elementSize = sizeof(int8_t);
        break;
    case napi_int16_array:
    case napi_uint16_array:
        elementSize = sizeof(int16_t);
        break;
    case napi_int32_array:
    case napi_uint32_array:
        elementSize = sizeof(int32_t);
        break;
    case napi_float32_array:
        elementSize = sizeof(float);
        break;
    case napi_float64_array:
        elementSize = sizeof(double);
        break;
    case napi_bigint64_array:
    case napi_biguint64_array:
        elementSize = sizeof(int64_t);
        break;
    default:
    // 默认创建napi_int8_array类型
        arrayType = napi_int8_array;
        elementSize = sizeof(int8_t);
        break;
    }
    size_t length = 3;
    napi_value arrayBuffer = nullptr;
    napi_value typedArray = nullptr;
    void *data;
    // 创建一个ArrayBuffer
    napi_create_arraybuffer(env, length * elementSize, (void **)&data, &arrayBuffer);
    // 根据给定类型创建TypedArray
    napi_create_typedarray(env, arrayType, length, arrayBuffer, 0, &typedArray);
    return typedArray;
}
```
#### **ANI 示例**
可使用Object_New代替
```C++
class MobilePhone {
    constructor(model: String, weight: int) {
        this.model = model;
        this.weight = weight;
    }
    model: String;
    weight: int;
}

function checkModel(p: MobilePhone, model: String): boolean {
    return p.model == model;
}

function checkWeight(p: MobilePhone, weight: int): boolean {
    return p.weight == weight;
}
```

cpp代码
```C++
public:
    void GetTestData(ani_class *clsResult, ani_method *ctorResult, ani_string *modelResult, ani_int *weightResult)
    {
        const char m[] = "Pure P60";
        const ani_int weight = 200;

        ani_class cls;
        ASSERT_EQ(env_->FindClass("LMobilePhone;", &cls), ANI_OK);

        ani_method ctor;
        ASSERT_EQ(env_->Class_GetMethod(cls, "<ctor>", "Lstd/core/String;I:V", &ctor), ANI_OK);

        ani_string model;
        ASSERT_EQ(env_->String_NewUTF8(m, strlen(m), &model), ANI_OK);

        *clsResult = cls;
        *ctorResult = ctor;
        *modelResult = model;
        *weightResult = weight;
    }

TEST_F(ObjectNewTest, object_new)
{
    ani_class cls;
    ani_method ctor;
    ani_string model;
    ani_int weight;
    GetTestData(&cls, &ctor, &model, &weight);

    ani_object phone;
    ASSERT_EQ(env_->c_api->Object_New(env_, cls, ctor, &phone, model, weight), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkModel", phone, model), ANI_TRUE);
    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkWeight", phone, weight), ANI_TRUE);
}
```


### napi_create_error迁移示例
---
ANI中创建对象统一由Object_New()进行创建。
流程为：
1.FindClass找到Class的指针。
2.Class_FindMethod找到构造函数。
3.调用Object_New，入参顺序为Class指针,构造函数,返回对象的引用,可变入参。

#### 代码示例对比

#### **N-API 示例**
```CPP
napi_value error;
napi_create_error(env, "ERR_INVALID_ARG_TYPE", "The argument must be a number.", &error);
```
#### **ANI 示例**
```cpp
ani_class errCls;
if (ANI_OK != env->FindClass("Lstd/core/NullPointerError;", &errCls)) {
    std::cerr << "Not found '"  << "'" << std::endl;
    return ANI_ERROR;
}

ani_object errObj;
//查找构造函数，如果没有重载，可以直接传递 nullptr
ani_method errCtor;
if (ANI_OK != env->Class_FindMethod(errCls, "<ctor>", nullptr, &errCtor)) {
    std::cerr << "get errCtor Failed'" << "Lstd/core/NullPointerError;" << "'" << std::endl;
    return ANI_ERROR;
}

std::string name = "This will show message!";
ani_string result_string{};
env->String_NewUTF8(name.c_str(), name.size(), &result_string);

//创建一个 Error  的实例
if (ANI_OK != env->Object_New(errCls, errCtor, &errObj, result_string)) {
    std::cerr << "Create Object Failed'" << "Lstd/core/NullPointerError;" << "'" << std::endl;
    return ANI_ERROR;
}
```

### napi_create_type_error迁移示例
---
ANI中创建对象统一由Object_New()进行创建。
流程为：
1.FindClass找到Class的指针。
2.Class_FindMethod找到构造函数。
3.调用Object_New，入参顺序为Class指针,构造函数,返回对象的引用,可变入参。

#### 代码示例对比

#### **N-API 示例**
```CPP
napi_value error;
napi_create_type_error(env, "ERR_INVALID_ARG_TYPE", "The argument must be a number.", &error);
```
#### **ANI 示例**
```cpp
ani_class errCls;
if (ANI_OK != env->FindClass("Lescompat/TypeError;", &errCls)) {
    std::cerr << "Not found '"  << "'" << std::endl;
    return ANI_ERROR;
}

ani_object errObj;
//查找构造函数，如果没有重载，可以直接传递 nullptr
ani_method errCtor;
if (ANI_OK != env->Class_FindMethod(errCls, "<ctor>", nullptr, &errCtor)) {
    std::cerr << "get errCtor Failed'" << "Lescompat/TypeError;" << "'" << std::endl;
    return ANI_ERROR;
}

std::string name = "The argument must be a number.";
ani_string result_string{};
env->String_NewUTF8(name.c_str(), name.size(), &result_string);

//创建一个 Error  的实例
if (ANI_OK != env->Object_New(errCls, errCtor, &errObj, result_string)) {
    std::cerr << "Create Object Failed'" << "Lescompat/TypeError;" << "'" << std::endl;
    return ANI_ERROR;
}
env->ThrowError(static_cast<ani_error>(errObj));
```


### napi_create_range_error迁移示例
---
ANI中创建对象统一由Object_New()进行创建。
流程为：
1.FindClass找到Class的指针。
2.Class_FindMethod找到构造函数。
3.调用Object_New，入参顺序为Class指针,构造函数,返回对象的引用,可变入参。

#### 代码示例对比

#### **N-API 示例**
```CPP
napi_value error;
napi_create_range_error(env, "ERR_INVALID_ARG_TYPE", "The argument must be a number.", &error);
```

#### **ANI 示例**
```cpp
ani_class errCls;
if (ANI_OK != env->FindClass("Lstd/core/RangeError;", &errCls)) {
    std::cerr << "Not found '"  << "'" << std::endl;
    return ANI_ERROR;
}

ani_object errObj;
//查找构造函数，如果没有重载，可以直接传递 nullptr
ani_method errCtor;
if (ANI_OK != env->Class_FindMethod(errCls, "<ctor>", nullptr, &errCtor)){
    std::cerr << "get errCtor Failed'" << "Lstd/core/RangeError;" << "'" << std::endl;
    return ANI_ERROR;
}

std::string name = "The argument must be between 0 and 100.";
ani_string result_string{};
env->String_NewUTF8(name.c_str(), name.size(), &result_string);

//创建一个 Error  的实例
if (ANI_OK != env->Object_New(errCls, errCtor, &errObj, result_string )){
    std::cerr << "Create Object Failed'" << "Lstd/core/RangeError;" << "'" << std::endl;
    return ANI_ERROR;
}
```


### napi_create_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_create_object用于在Node-API模块中创建一个空的ArkTS对象。
```C++
napi_value NewObject(napi_env env, napi_callback_info info)
{
    napi_value object = nullptr;
    // 创建一个空对象
    napi_create_object(env, &object);
    // 设置对象的属性
    napi_value name = nullptr;
    // 设置属性名为"name"
    napi_create_string_utf8(env, "name", NAPI_AUTO_LENGTH, &name);
    napi_value value = nullptr;
    // 设置属性值为"Hello from Node-API!"
    napi_create_string_utf8(env, "Hello from Node-API!", NAPI_AUTO_LENGTH, &value);
    // 将属性设置到对象上
    napi_set_property(env, object, name, value);
    return object;
}
```
#### **ANI 示例**
可使用Object_New代替
```C++
class MobilePhone {
    constructor(model: String, weight: int) {
        this.model = model;
        this.weight = weight;
    }
    model: String;
    weight: int;
}

function checkModel(p: MobilePhone, model: String): boolean {
    return p.model == model;
}

function checkWeight(p: MobilePhone, weight: int): boolean {
    return p.weight == weight;
}
```

cpp代码
```C++
public:
    void GetTestData(ani_class *clsResult, ani_method *ctorResult, ani_string *modelResult, ani_int *weightResult)
    {
        const char m[] = "Pure P60";
        const ani_int weight = 200;

        ani_class cls;
        ASSERT_EQ(env_->FindClass("LMobilePhone;", &cls), ANI_OK);

        ani_method ctor;
        ASSERT_EQ(env_->Class_GetMethod(cls, "<ctor>", "Lstd/core/String;I:V", &ctor), ANI_OK);

        ani_string model;
        ASSERT_EQ(env_->String_NewUTF8(m, strlen(m), &model), ANI_OK);

        *clsResult = cls;
        *ctorResult = ctor;
        *modelResult = model;
        *weightResult = weight;
    }

TEST_F(ObjectNewTest, object_new)
{
    ani_class cls;
    ani_method ctor;
    ani_string model;
    ani_int weight;
    GetTestData(&cls, &ctor, &model, &weight);

    ani_object phone;
    ASSERT_EQ(env_->c_api->Object_New(env_, cls, ctor, &phone, model, weight), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkModel", phone, model), ANI_TRUE);
    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkWeight", phone, weight), ANI_TRUE);
}
```


#### **ANI 示例2**
Interface无法直接创建，必须在sts侧用class implement interface的方式创建一个class类型，调用该类型来创建一个对象。
https://gitee.com/openharmony/arkcompiler_runtime_core/issues/IBNXVU

```TS
loadLibrary("ani_test")
interface Person {
   name: string
   age: number
}

class PersonInfo implements Person {
  name: string = ""
  age: number = 1345234
}
```

```CPP
static const char *className = "Lani_double/PersonInfo;";
ani_class persion_cls;
env->FindClass(className, &persion_cls);
ani_method personInfoCtor;
env->Class_FindMethod(persion_cls, "<ctor>",":V", &personInfoCtor);
ani_object personInfoObj;
env->Object_New(persion_cls, personInfoCtor, &personInfoObj);

// 测试允许使用<get>age, 可跳过。 property的情况下无法使用GetField方法获取字段
ani_method getAge;
env->Class_FindMethod(persion_cls, "<get>age", ":D", &getAge);

ani_double age_value;
env->Object_CallMethodByName_Double(personInfoObj, "<get>age", ":D", &age_value);
```


### napi_create_object_with_properties迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_create_object_with_named_properties迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**






### napi_create_symbol迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_create_symbol用于创建一个新的Symbol。Symbol是一种特殊的数据类型，用于表示唯一的标识符。与字符串或数字不同，符号的值是唯一的，即使两个符号具有相同的描述，它们也是不相等的。符号通常用作对象属性的键，以确保属性的唯一性。
cpp部分代码
```C++
static napi_value CreateSymbol(napi_env env, napi_callback_info info)
{
    napi_value result = nullptr;
    const char *des = "only";
    // 使用napi_create_string_utf8创建描述字符串
    napi_create_string_utf8(env, des, NAPI_AUTO_LENGTH, &result);
    napi_value returnSymbol = nullptr;
    // 创建一个symbol类型，并返回
    napi_create_symbol(env, result, &returnSymbol);
    return returnSymbol;
}
```
#### **ANI 示例**
可使用Object_New代替
```C++
class MobilePhone {
    constructor(model: String, weight: int) {
        this.model = model;
        this.weight = weight;
    }
    model: String;
    weight: int;
}

function checkModel(p: MobilePhone, model: String): boolean {
    return p.model == model;
}

function checkWeight(p: MobilePhone, weight: int): boolean {
    return p.weight == weight;
}
```

cpp代码
```C++
public:
    void GetTestData(ani_class *clsResult, ani_method *ctorResult, ani_string *modelResult, ani_int *weightResult)
    {
        const char m[] = "Pure P60";
        const ani_int weight = 200;

        ani_class cls;
        ASSERT_EQ(env_->FindClass("LMobilePhone;", &cls), ANI_OK);

        ani_method ctor;
        ASSERT_EQ(env_->Class_GetMethod(cls, "<ctor>", "Lstd/core/String;I:V", &ctor), ANI_OK);

        ani_string model;
        ASSERT_EQ(env_->String_NewUTF8(m, strlen(m), &model), ANI_OK);

        *clsResult = cls;
        *ctorResult = ctor;
        *modelResult = model;
        *weightResult = weight;
    }

TEST_F(ObjectNewTest, object_new)
{
    ani_class cls;
    ani_method ctor;
    ani_string model;
    ani_int weight;
    GetTestData(&cls, &ctor, &model, &weight);

    ani_object phone;
    ASSERT_EQ(env_->c_api->Object_New(env_, cls, ctor, &phone, model, weight), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkModel", phone, model), ANI_TRUE);
    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkWeight", phone, weight), ANI_TRUE);
}
```


### napi_create_date迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_date 在需要根据当前系统时间或特定计算生成一个Date对象时，可通过使用此接口创建表示这些时间的ArkTS Date对象，然后将其传递给ArkTS代码进行进一步处理。

```cpp
// cpp
#include "napi/native_api.h"

static napi_value CreateDate(napi_env env, napi_callback_info info)
{
    // 获取传入的Unix Time Stamp时间
    double value = 1501924876711;
    // 调用napi_create_date接口将double值转换成表示日期时间，并创建成一个ArkTS对象放入returnValue中
    napi_value returnValue = nullptr;
    napi_create_date(env, value, &returnValue);
    return returnValue;
}
```

```ts
// index.d.ts
export const createDate: () => Date;

// ets
import hilog from '@ohos.hilog'
import testNapi from 'libentry.so'

hilog.info(0x0000, 'testTag', 'Test Node-API napi_create_date: %{public}s', testNapi.createDate().toString());
```

#### **ANI 示例**

可通过`Object_New`进行替代，该函数可通过给定类和调用确定的构造函数来创建对应的对象。

```ts
// sts
class TestDate {
    constructor(timestamp: double) {
        this.date = new Date(timestamp);
    }
    date: Date;
}

function checkDate(date: TestDate, timestamp: double): boolean {
    return date.date.getTime() == timestamp;
}
```

```cpp
// cpp
TEST_F(ExampleTest, TestDate)
{
    ani_class cls;
    ASSERT_EQ(env_->FindClass("LTestDate;", &cls), ANI_OK);

    ani_method ctor;
    ASSERT_EQ(env_->Class_FindMethod(cls, "<ctor>", "D:V", &ctor), ANI_OK);

    ani_double timestamp = 1501924876711;
    ani_object objDate;
    ASSERT_EQ(env_->c_api->Object_New(env_, cls, ctor, &objDate, timestamp), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkDate", objDate, timestamp), ANI_TRUE);
}
```

### napi_create_dataview迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_create_dataview用于创建dataview对象，便于访问和操作二进制数据，需要提供一个指向二进制数据的缓冲区，并指定要包含的字节数。
cpp部分代码
```C++
static napi_value CreateDataView(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_value arraybuffer = nullptr;
    napi_value result = nullptr;
    // DataView的字节长度
    size_t byteLength = 12;
    // 字节偏移量
    size_t byteOffset = 4;
    // 获取回调函数的参数信息
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 将参数转换为对象类型
    napi_coerce_to_object(env, args[0], &arraybuffer);
    // 创建一个数据视图对象，并指定字节长度和字节偏移量
    napi_status status = napi_create_dataview(env, byteLength, arraybuffer, byteOffset, &result);
    if (status != napi_ok) {
        // 抛出创建DataView内容失败的错误
        napi_throw_error(env, nullptr, "Failed to create DataView");
        return nullptr;
    }
    // 获取DataView的指针和长度信息
    uint8_t *data = nullptr;
    size_t length = 0;
    napi_get_dataview_info(env, result, &length, (void **)&data, nullptr, nullptr);
    // 为DataView赋值
    for (size_t i = 0; i < length; i++) {
        data[i] = static_cast<uint8_t>(i + 1);
    }
    return result;
}
```
#### **ANI 示例**
可使用Object_New代替
```C++
class MobilePhone {
    constructor(model: String, weight: int) {
        this.model = model;
        this.weight = weight;
    }
    model: String;
    weight: int;
}

function checkModel(p: MobilePhone, model: String): boolean {
    return p.model == model;
}

function checkWeight(p: MobilePhone, weight: int): boolean {
    return p.weight == weight;
}
```

cpp代码
```C++
public:
    void GetTestData(ani_class *clsResult, ani_method *ctorResult, ani_string *modelResult, ani_int *weightResult)
    {
        const char m[] = "Pure P60";
        const ani_int weight = 200;

        ani_class cls;
        ASSERT_EQ(env_->FindClass("LMobilePhone;", &cls), ANI_OK);

        ani_method ctor;
        ASSERT_EQ(env_->Class_GetMethod(cls, "<ctor>", "Lstd/core/String;I:V", &ctor), ANI_OK);

        ani_string model;
        ASSERT_EQ(env_->String_NewUTF8(m, strlen(m), &model), ANI_OK);

        *clsResult = cls;
        *ctorResult = ctor;
        *modelResult = model;
        *weightResult = weight;
    }

TEST_F(ObjectNewTest, object_new)
{
    ani_class cls;
    ani_method ctor;
    ani_string model;
    ani_int weight;
    GetTestData(&cls, &ctor, &model, &weight);

    ani_object phone;
    ASSERT_EQ(env_->c_api->Object_New(env_, cls, ctor, &phone, model, weight), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkModel", phone, model), ANI_TRUE);
    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkWeight", phone, weight), ANI_TRUE);
}
```



### napi_create_bigint_int64迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_bigint_int64 用于创建64位带符号整数（int64）的BigInt对象的函数。

```cpp
// cpp
#include "napi/native_api.h"

static napi_value CreateBigintInt64t(napi_env env, napi_callback_info info)
{
    // 声明int64_t的变量value
    int64_t value = -5555555555555555555;
    // 将value转化为napi_value类型返回
    napi_value returnValue = nullptr;
    napi_create_bigint_int64(env, value, &returnValue);
    return returnValue;
}
```

```ts
// index.d.ts
export const createBigintInt64t: () => bigint;

// ets
import hilog from '@ohos.hilog'
import testNapi from 'libentry.so'

hilog.info(0x0000, 'testTag', 'Test Node-API napi_create_bigint_int64: %{public}d', testNapi.createBigintInt64t());
```

#### **ANI 示例**

可通过`Object_New`进行替代，该函数可通过给定类和调用确定的构造函数来创建对应的对象。

```ts
// sts
class TestBigInt {
    constructor(bigInt: string) {
        this.bigInt = new BigInt(bigInt);
    }
    public Get(): string {
        return this.bigInt.toString();
    }
    bigInt: bigint;
}

function checkBigInt(bigInt: TestBigInt, strBigInt: String): boolean {
    return bigInt.Get() === strBigInt;
}
```

```cpp
// cpp
TEST_F(ExampleTest, TestBigInt64)
{
    ani_class cls;
    ASSERT_EQ(env_->FindClass("LTestBigInt;", &cls), ANI_OK);

    ani_method ctor;
    ASSERT_EQ(env_->Class_FindMethod(cls, "<ctor>", "Lstd/core/String;:V", &ctor), ANI_OK);

    const char bigIntValue[] = "-1234567890123456789";
    ani_string strBigIntValue;
    ASSERT_EQ(env_->String_NewUTF8(bigIntValue, strlen(bigIntValue), &strBigIntValue), ANI_OK);
    ani_object objBigInt64;
    ASSERT_EQ(env_->c_api->Object_New(env_, cls, ctor, &objBigInt64, strBigIntValue), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkBigInt", objBigInt64, strBigIntValue), ANI_TRUE);
}
```


### napi_create_bigint_uint64迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_bigint_uint64 用于创建64位无符号整数（uint64）的BigInt对象的函数。

```cpp
// cpp
#include "napi/native_api.h"

static napi_value CreateBigintUint64t(napi_env env, napi_callback_info info)
{
    // 声明uint64_t的变量value
    uint64_t value = 5555555555555555555;
    // 将value转化为napi_value类型返回
    napi_value returnValue = nullptr;
    napi_create_bigint_uint64(env, value, &returnValue);
    return returnValue;
}
```

```ts
// index.d.ts
export const createBigintInt64t: () => bigint;

// ets
import hilog from '@ohos.hilog'
import testNapi from 'libentry.so'

hilog.info(0x0000, 'testTag', 'Test Node-API napi_create_bigint_int64: %{public}d', testNapi.createBigintInt64t());
```

#### **ANI 示例**

可通过`Object_New`进行替代，该函数可通过给定类和调用确定的构造函数来创建对应的对象。

```ts
// sts
class TestBigInt {
    constructor(bigInt: string) {
        this.bigInt = new BigInt(bigInt);
    }
    public Get(): string {
        return this.bigInt.toString();
    }
    bigInt: bigint;
}

function checkBigInt(bigInt: TestBigInt, strBigInt: String): boolean {
    return bigInt.Get() === strBigInt;
}
```

```cpp
// cpp
TEST_F(ExampleTest, TestBigUint64)
{
    ani_class cls;
    ASSERT_EQ(env_->FindClass("LTestBigInt;", &cls), ANI_OK);

    ani_method ctor;
    ASSERT_EQ(env_->Class_FindMethod(cls, "<ctor>", "Lstd/core/String;:V", &ctor), ANI_OK);

    const char bigUintValue[] = "1234567890123456789";
    ani_string strBigUintValue;
    ASSERT_EQ(env_->String_NewUTF8(bigUintValue, strlen(bigUintValue), &strBigUintValue), ANI_OK);
    ani_object objBigUint64;
    ASSERT_EQ(env_->c_api->Object_New(env_, cls, ctor, &objBigUint64, strBigUintValue), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkBigInt", objBigUint64, strBigUintValue), ANI_TRUE);
}
```

### napi_create_bigint_words迁移示例
---
#### 代码示例对比

#### **N-API 示例**
通过一个C的uint64数组创建单个JS BigInt。
```C++
int signBit = 0;
size_t wordCount = 4;
uint64_t* words = nullptr;
napi_value result = nullptr;

napi_status status = napi_create_bigint_words(env, signBit, wordCount, words, &result);
ASSERT_EQ(status, napi_invalid_arg);
```

#### **ANI 示例**
可通过`Object_New`进行替代，该函数可通过给定类和调用确定的构造函数来创建对应的对象。
```C++
// ets file
class MobilePhone {
    constructor(model: String, weight: int) {
        this.model = model;
        this.weight = weight;
    }
    model: String;
    weight: int;
}

function checkModel(p: MobilePhone, model: String): boolean {
    return p.model == model;
}

function checkWeight(p: MobilePhone, weight: int): boolean {
    return p.weight == weight;
}

// cpp file
void GetTestData(ani_class *clsResult, ani_method *ctorResult, ani_string *modelResult, ani_int *weightResult)
{
    const char m[] = "Pure P60";
    const ani_int weight = 200;

    ani_class cls;
    ASSERT_EQ(env_->FindClass("LMobilePhone;", &cls), ANI_OK);

    ani_method ctor;
    ASSERT_EQ(env_->Class_FindMethod(cls, "<ctor>", "Lstd/core/String;I:V", &ctor), ANI_OK);

    ani_string model;
    ASSERT_EQ(env_->String_NewUTF8(m, strlen(m), &model), ANI_OK);

    *clsResult = cls;
    *ctorResult = ctor;
    *modelResult = model;
    *weightResult = weight;
}

ani_class cls;
ani_method ctor;
ani_string model;
ani_int weight;
GetTestData(&cls, &ctor, &model, &weight);

ani_object phone;
ASSERT_EQ(env_->c_api->Object_New(env_, cls, ctor, &phone, model, weight), ANI_OK);

ASSERT_EQ(CallEtsFunction<ani_boolean>("checkModel", phone, model), ANI_TRUE);
ASSERT_EQ(CallEtsFunction<ani_boolean>("checkWeight", phone, weight), ANI_TRUE);
```


### napi_wrap迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```C++
napi_wrap 用于将 C++ 对象包装到 JavaScript 对象中。可以将一个 C++ 对象的指针与一个 JavaScript 对象关联起来，从而在 JavaScript 中访问这个 C++ 对象。
napi_value MyObject::New(napi_env env, napi_callback_info info)
{
  OH_LOG_INFO(LOG_APP, "MyObject::New called");
  napi_value newTarget;
  napi_get_new_target(env, info, &newTarget);
  if (newTarget != nullptr) {
    // 使用`new MyObject(...)`调用方式
    size_t argc = 1;
    napi_value args[1];
    napi_value jsThis;
    napi_get_cb_info(env, info, &argc, args, &jsThis, nullptr);
    double value = 0.0;
    napi_valuetype valuetype;
    napi_typeof(env, args[0], &valuetype);
    if (valuetype != napi_undefined) {
      napi_get_value_double(env, args[0], &value);
    }
    MyObject* obj = new MyObject(value);
    obj->env_ = env;
    // 通过napi_wrap将ArkTS对象jsThis与C++对象obj绑定
    napi_status status = napi_wrap(env,
                                   jsThis,
                                   reinterpret_cast<void*>(obj),
                                   MyObject::Destructor,
                                   nullptr,  // finalize_hint
                                   &obj->wrapper_);
    // napi_wrap失败时，必须手动释放已分配的内存，以防止内存泄漏
    if (status != napi_ok) {
      OH_LOG_INFO(LOG_APP, "Failed to bind native object to js object"
                  ", return code: %{public}d", status);
      delete obj;
      return jsThis;
    }
    // 从napi_wrap接口的result获取napi_ref的行为，将会为jsThis创建强引用，
    // 若开发者不需要主动管理jsThis的生命周期，可直接在napi_wrap最后一个参数中传入nullptr，
    // 或者使用napi_reference_unref方法将napi_ref转为弱引用。
    uint32_t refCount = 0;
    napi_reference_unref(env, obj->wrapper_, &refCount);
    return jsThis;
  } else {
    // 使用`MyObject(...)`调用方式
    size_t argc = 1;
    napi_value args[1];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    napi_value cons;
    napi_get_reference_value(env, g_ref, &cons);
    napi_value instance;
    napi_new_instance(env, cons, argc, args, &instance);
    return instance;
  }
}
```
#### **ANI 示例**
在STS中，通常做法是在STS中声明一个long的变量，在ANI中通过Object_SetField_Long把native侧的指针存到这个field里
```C++
class Package {
    long_value: long = 0;
    string_value: String = "";
}

function newPackageObject() {
    return new Package();
}

function checkLongValue(p: Package, value: long): boolean {
    return p.long_value == value;
}
```
cpp部分代码
```C++
void GetTestData(ani_object *packResult, ani_field *fieldLongResult, ani_field *fieldStringResult)
{
    auto packRef = CallEtsFunction<ani_ref>("newPackageObject");
    ani_class cls;
    ASSERT_EQ(env_->FindClass("LPackage;", &cls), ANI_OK);
    ani_field fieldLong;
    ASSERT_EQ(env_->Class_GetField(cls, "long_value", &fieldLong), ANI_OK);
    ani_field fieldString;
    ASSERT_EQ(env_->Class_GetField(cls, "string_value", &fieldString), ANI_OK);
    *packResult = static_cast<ani_object>(packRef);
    *fieldLongResult = fieldLong;
    *fieldStringResult = fieldString;
}

ani_object pack;
ani_field fieldLong;
ani_field fieldString;
GetTestData(&pack, &fieldLong, &fieldString);
ASSERT_EQ(CallEtsFunction<ani_boolean>("checkLongValue", pack, ani_long(0)), ANI_TRUE);
ASSERT_EQ(env_->Object_SetField_Long(pack, fieldLong, 8L), ANI_OK);
ASSERT_EQ(CallEtsFunction<ani_boolean>("checkLongValue", pack, ani_long(8L)), ANI_TRUE);
```



### napi_unwrap迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_unwrap 用于从 JavaScript 对象中提取之前通过 napi_wrap 关联的 C++ 对象指针。通过 napi_unwrap，可以在 C++ 代码中访问与 JavaScript 对象关联的原生对象。
```C++
napi_value MyObject::GetValue(napi_env env, napi_callback_info info)
{
  OH_LOG_INFO(LOG_APP, "MyObject::GetValue called");
  napi_value jsThis;
  napi_get_cb_info(env, info, nullptr, nullptr, &jsThis, nullptr);
  MyObject* obj;
  // 通过napi_unwrap将jsThis之前绑定的C++对象取出，并对其进行操作
  napi_unwrap(env, jsThis, reinterpret_cast<void**>(&obj));
  napi_value num;
  napi_create_double(env, obj->value_, &num);
  return num;
}
```
#### **ANI 示例**
在STS中，通常做法是在STS中声明一个long的变量，在ANI中通过Object_SetField_Long把native侧的指针存到这个field里,使用时通过Object_GetField_Long取出来。
```C++
class Woman {
    constructor(name: String, age: long) {
        this.name = name;
        this.age = age;
    }
    name: String;
    age: long;
}

function newSarahObject() {
    return new Woman("Sarah", 24);
}
```
CPP部分代码
```C++
void GetTestData(ani_object *objectResult, ani_field *fieldNameResult, ani_field *fieldAgeResult)
{
    auto sarahRef = CallEtsFunction<ani_ref>("newSarahObject");
    auto sarah = static_cast<ani_object>(sarahRef);
    ani_class cls;
    ASSERT_EQ(env_->FindClass("LWoman;", &cls), ANI_OK);
    ani_field fieldName;
    ASSERT_EQ(env_->Class_GetField(cls, "name", &fieldName), ANI_OK);
    ani_field fieldAge;
    ASSERT_EQ(env_->Class_GetField(cls, "age", &fieldAge), ANI_OK);
    *objectResult = sarah;
    *fieldNameResult = fieldName;
    *fieldAgeResult = fieldAge;
}

ani_object sarah {};
ani_field field {};
ani_field fieldAge {};
GetTestData(&sarah, &field, &fieldAge);
ani_long age {};
ASSERT_EQ(env_->Object_GetField_Long(sarah, fieldAge, &age), ANI_OK);
ASSERT_EQ(age, 24L);
```


### napi_wrap_with_size迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




### napi_get_value_bigint_words迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value GetValueBigintWords(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    int signBit = 0;
    size_t wordCount = 0;
    uint64_t words = 0;
    // 调用napi_get_value_bigint_words接口获取wordCount
    napi_status status = napi_get_value_bigint_words(env, args[0], nullptr, &wordCount, nullptr);
    OH_LOG_INFO(LOG_APP, "Node-API , wordCount:%{public}d.", wordCount);
    // 调用napi_get_value_bigint_words接口获取传入bigInt相关信息，如：signBit传入bigInt正负信息
    status = napi_get_value_bigint_words(env, args[0], &signBit, &wordCount, &words);
    OH_LOG_INFO(LOG_APP, "Node-API , signBit: %{public}d.", signBit);
    if (status != napi_ok) {
        OH_LOG_ERROR(LOG_APP, "Node-API , reason:%{public}d.", status);
        napi_throw_error(env, nullptr, "napi_get_date_value fail");
        return nullptr;
    }
    // 将符号位转化为int类型传出去
    napi_value returnValue = nullptr;
    napi_create_int32(env, signBit, &returnValue);
    return returnValue;
}
```

```ts
// index.d.ts
export const getValueBigintWords: (bigIntWords: bigint) => bigint;

// ets
let bigInt = BigInt(9223372036854775807n);
testNapi.getValueBigintWords(bigInt);
```

#### **ANI 示例**
可通过Bigint类直接处理Bigint值。
```cpp
// sts
function GetBigint() {
    let num : bigint = 9223372036854775807n;
    return num;
}

// cpp
auto bigintRef = CallEtsFunction<ani_ref>("GetBigint");
ani_object bigintNum = static_cast<ani_object>(bigintRef);
ani_class bigIntCls;
const char * className = "Lescompat/BigInt;";
if (ANI_OK != env_->FindClass(className, &bigIntCls)) {
    std::cerr << "Not found '" << className << "'" << std::endl;
    return ;
}
ani_method isPositiveMethod;
if (ANI_OK != env_->Class_GetMethod(bigIntCls, "positive", ":Z", &isPositiveMethod)){
    std::cerr << "Class_GetMethod Failed '" << className << "'" << std::endl;
    return ;
}

ani_boolean isPositive;
if (ANI_OK != env_->Object_CallMethod_Boolean(bigintNum, isPositiveMethod, &isPositive)){
    std::cerr << "Object_CallMethod_Long '" << "getLongMethod" << "'" << std::endl;
    return ;
}
```




### napi_create_array_with_length迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value CreateArrayWithLength(napi_env env, napi_callback_info info)
{
    napi_value jsArray = nullptr;
    const char *utf8String = "test";
    const size_t stringLength = std::strlen(utf8String);
    // 使用napi_create_array_with_length创建指定长度的数组
    napi_create_array_with_length(env, stringLength, &jsArray);
    // 返回数组
    return jsArray;
}
```

#### **ANI 示例**
```cpp
// Test creating array with initial element
ani_string str = nullptr;
const char *utf8String = "test";
const ani_size stringLength = strlen(utf8String);
env_->String_NewUTF8(utf8String, stringLength, &str);
ani_array_ref array2 = nullptr;
env_->Array_New_Ref(cls, stringLength, str, &array2);
```


### napi_create_uint32迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


### napi_get_prototype迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_get_prototype 当需要获取一个ArkTS对象的原型时，可以使用这个接口。通过这个接口可以在C/C++中获取到这个原型对象。

```cpp
#include "napi/native_api.h"

static napi_value GetPrototype(napi_env env, napi_callback_info info)
{
    // 获取并解析传参
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args , nullptr, nullptr);
    napi_value result = nullptr;
    // 获取此对象的原型对象，将结果返回到napi_value类型的变量result中
    napi_get_prototype(env, args[0], &result);
    return result;
}
```

#### **ANI 示例**

```cpp
// 用法不同，不实现
```


### napi_get_date_value迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_get_date_value 在Node-API模块中接收到一个ArkTS的Date对象，并且需要获取其对应的时间戳或日期值时，可以使用此接口。
```cpp
#include <hilog/log.h>
#include "napi/native_api.h"

static napi_value GetDateValue(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // 获取传入的Unix Time Stamp时间
    double value = 0;
    napi_status status = napi_get_date_value(env, args[0], &value);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "napi_get_date_value fail");
        return nullptr;
    }

    // 将获取到的Unix Time Stamp时间打印
    OH_LOG_INFO(LOG_APP, "Node-API gets unix time stamp is:%{public}lf.", value);

    // 把转换后的Unix Time Stamp时间创建成ArkTS double数值，并放入returnValue中
    napi_value returnValue = nullptr;
    napi_create_double(env, value, &returnValue);
    return returnValue;
}
```

#### **ANI 示例**

```cpp
// 用法不同，不实现
```


### napi_get_typedarray_info迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_get_typedarray_info 用于在Node-API模块中获得某个TypedArray的各种属性。

```cpp
#include "napi/native_api.h"

static napi_value GetTypedarrayInfo(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数，第一个参数为需要获得的信息的TypedArray类型数据，第二个参数为需要获得的信息类型的枚举值
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 将第二个参数转为int32类型便于比较
    int32_t infoTypeParam;
    napi_get_value_int32(env, args[1], &infoTypeParam);
    // 定义枚举类型与ArkTS侧枚举类型InfoType顺序含义一致
    enum InfoType { INFO_TYPE = 1, INFO_LENGTH, INFO_ARRAY_BUFFER, INFO_BYTE_OFFSET };
    void *data;
    napi_typedarray_type type;
    size_t byteOffset, length;
    napi_value arraybuffer;
    // 调用接口napi_get_typedarray_info获得TypedArray类型数据的信息
    napi_get_typedarray_info(env, args[0], &type, &length, &data, &arraybuffer, &byteOffset);
    napi_value result;
    // 根据属性名，返回TypedArray对应的属性值
    switch (infoTypeParam) {
    case INFO_TYPE:
        // 如果传入的参数是int8类型的TypedArray数据，它的类型（type）为napi_int8_array
        napi_value int8_type;
        napi_get_boolean(env, type == napi_int8_array, &int8_type);
        result = int8_type;
        break;
    case INFO_LENGTH:
        // TypedArray中元素的字节长度
        napi_value napiLength;
        napi_create_int32(env, length, &napiLength);
        result = napiLength;
        break;
    case INFO_BYTE_OFFSET:
        // TypedArray数组的第一个元素所在的基础原生数组中的字节偏移量
        napi_value napiByteOffset;
        napi_create_int32(env, byteOffset, &napiByteOffset);
        result = napiByteOffset;
        break;
    case INFO_ARRAY_BUFFER:
        // TypedArray下的ArrayBuffer
        result = arraybuffer;
        break;
    default:
        break;
    }
    return result;
}
```

#### **ANI 示例**

```cpp
// 用法不同，不实现
```



### napi_get_dataview_info迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_get_dataview_info 用于在Node-API模块中获得某个DataView的各种属性。

```cpp
#include "napi/native_api.h"

static napi_value GetDataViewInfo(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 将第二个参数转为int32类型的数字
    int32_t infoType;
    napi_get_value_int32(env, args[1], &infoType);
    size_t byteLength;
    void *data;
    napi_value arrayBuffer;
    size_t byteOffset;
    // 定义枚举类型与ArkTS侧枚举类型InfoType顺序含义一致
    enum InfoType { BYTE_LENGTH = 0, ARRAY_BUFFER, BYTE_OFFSET };
    // 获取dataview信息
    napi_get_dataview_info(env, args[0], &byteLength, &data, &arrayBuffer, &byteOffset);
    napi_value result;
    switch (infoType) {
        case BYTE_LENGTH:
            // 返回查询DataView的字节数
            napi_value napiByteLength;
            napi_create_int32(env, byteLength, &napiByteLength);
            result = napiByteLength;
            break;
        case ARRAY_BUFFER:
            // 返回查询DataView的arraybuffer
            result = arrayBuffer;
            break;
        case BYTE_OFFSET:
            // 返回查询DataView的偏移字节量
            napi_value napiByteOffset;
            napi_create_int32(env, byteOffset, &napiByteOffset);
            result = napiByteOffset;
            break;
        default:
            break;
    }
    return result;
}
```

#### **ANI 示例**

```cpp
// 用法不同，不实现
```



### napi_get_value_external迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static int external = 5;
static napi_value GetValueExternal(napi_env env, napi_callback_info info)
{
    // 创建外部数据
    int* data = &external;
    napi_value setExternal = nullptr;
    napi_create_external(env, data, nullptr, nullptr, &setExternal);
    // 获得外部数据的值
    void *getExternal;
    napi_get_value_external(env, setExternal, &getExternal);
    // 返回获得到的外部数据
    napi_value result = nullptr;
    napi_create_int32(env, *(int *)getExternal, &result);
    return result;
}
```
#### **ANI 示例**
获取先前通过napi_create_external()传递的外部数据指针。可配合napi_create_external迁移方法实现。




### napi_get_value_string_latin1迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static const int MAX_BUFFER_SIZE = 128;
static napi_value GetValueStringLatin1(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args , nullptr, nullptr);
    char buf[MAX_BUFFER_SIZE];
    size_t length = 0;
    napi_value napi_Res = nullptr;
    napi_status status = napi_get_value_string_latin1(env, args[0], buf, MAX_BUFFER_SIZE, &length);
    // 当输入的值不是字符串时，接口会返回napi_string_expected
    if (status == napi_string_expected) {
        return nullptr;
    }
    OH_LOG_INFO(LOG_APP, "buf=%{public}s length=%{public}d", buf, length);
    return nullptr;
}
```

```ts
// index.d.ts
export const getValueStringLatin1: (param: number | string) => string | void;

// ets
testNapi.getValueStringLatin1("123456");
```

#### **ANI 示例**
```cpp
// sts
function GetString() {
    const str = "123456";
    return str;
}

// cpp
auto stringRef = CallEtsFunction<ani_ref>("GetString");
ani_string string = static_cast<ani_string>(stringRef);
ani_size buf_size = 256;
char buf[buf_size];
ani_size result = 0;
ani_status status = env_->String_GetUTF16(string, buf_size, buf_size, &result);
```

### napi_check_object_type_tag迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_check_object_type_tag 使用此接口可以检查给定的对象上是否关联了特定类型的标记。
```cpp
#include "napi/native_api.h"

#define NUMBERINT_FOUR 4
// 定义一个静态常量napi_type_tag数组存储类型标签
static const napi_type_tag TagsData[NUMBERINT_FOUR] = {
    {0x9e4b2449547061b3, 0x33999f8a6516c499},
    {0x1d55a794c53a726d, 0x43633f509f9c944e},
    // 用于表示无标签或默认标签
    {0, 0},
    {0x6a971439f5b2e5d7, 0x531dc28a7e5317c0},
};

static napi_value SetTypeTagToObject(napi_env env, napi_callback_info info)
{
    // 获取函数调用信息和参数
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 获取索引数字转换为napi_value
    int32_t index = 0;
    napi_get_value_int32(env, args[1], &index);
    // 给参数（对象）设置类型标签
    napi_status status = napi_type_tag_object(env, args[0], &TagsData[index]);
    if (status != napi_ok) {
        napi_throw_error(env, "Reconnect error", "napi_type_tag_object failed");
        return nullptr;
    }
    // 将bool结果转换为napi_value并返回
    napi_value result = nullptr;
    napi_get_boolean(env, true, &result);
    return result;
}

static napi_value CheckObjectTypeTag(napi_env env, napi_callback_info info)
{
    // 获取函数调用信息和参数
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 获取索引数字转换为napi_value
    int32_t index = 0;
    napi_get_value_int32(env, args[1], &index);
    // 检查对象的类型标签
    bool checkResult = true;
    napi_check_object_type_tag(env, args[0], &TagsData[index], &checkResult);
    // 将bool结果转换为napi_value并返回
    napi_value checked = nullptr;
    napi_get_boolean(env, checkResult, &checked);

    return checked;
}
```

#### **ANI 示例**

```cpp
// 用法不同，不实现
```




### napi_type_tag_object迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_type_tag_object 可以将指针的特定值与ArkTS对象关联起来，这对于一些自定义的内部对象标记非常有用。

```cpp
#include "napi/native_api.h"

#define NUMBERINT_FOUR 4
// 定义一个静态常量napi_type_tag数组存储类型标签
static const napi_type_tag TagsData[NUMBERINT_FOUR] = {
    {0x9e4b2449547061b3, 0x33999f8a6516c499},
    {0x1d55a794c53a726d, 0x43633f509f9c944e},
    // 用于表示无标签或默认标签
    {0, 0},
    {0x6a971439f5b2e5d7, 0x531dc28a7e5317c0},
};

static napi_value SetTypeTagToObject(napi_env env, napi_callback_info info)
{
    // 获取函数调用信息和参数
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 获取索引数字转换为napi_value
    int32_t index = 0;
    napi_get_value_int32(env, args[1], &index);
    // 给参数（对象）设置类型标签
    napi_status status = napi_type_tag_object(env, args[0], &TagsData[index]);
    if (status != napi_ok) {
        napi_throw_error(env, "Reconnect error", "napi_type_tag_object failed");
        return nullptr;
    }
    // 将bool结果转换为napi_value并返回
    napi_value result = nullptr;
    napi_get_boolean(env, true, &result);
    return result;
}

static napi_value CheckObjectTypeTag(napi_env env, napi_callback_info info)
{
    // 获取函数调用信息和参数
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 获取索引数字转换为napi_value
    int32_t index = 0;
    napi_get_value_int32(env, args[1], &index);
    // 检查对象的类型标签
    bool checkResult = true;
    napi_check_object_type_tag(env, args[0], &TagsData[index], &checkResult);
    // 将bool结果转换为napi_value并返回
    napi_value checked = nullptr;
    napi_get_boolean(env, checkResult, &checked);

    return checked;
}
```

#### **ANI 示例**

```cpp
// 用法不同，不实现
```



### napi_remove_wrap迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```
napi_remove_wrap 是 N-API 提供的一个函数，用于从给定的 JavaScript 对象中移除与之关联的 C/C++ 层面的数据（即所谓的“包装”数据）。在 Node.js 的 N-API 中，通常会使用 napi_wrap 函数将一个 C/C++ 对象或指针与 JavaScript 对象关联起来。这种机制允许你通过 JavaScript 对象访问底层的 C/C++ 数据结构，并且可以在回调或其他异步操作完成后安全地清理这些资源。

napi_status napi_remove_wrap(napi_env env,
                             napi_value js_object,
                             void* data);


env: 当前的 N-API 环境。
js_object: 需要从中移除关联数据的 JavaScript 对象。
data: 指向存储在 JavaScript 对象中的数据的指针。这个参数用于验证，确保你正在移除正确的数据。

返回值是一个 napi_status 类型的变量，表示操作的状态。如果成功，则返回 napi_ok；否则，返回错误代码。

使用场景
当你需要手动解除 JavaScript 对象和其底层 C/C++ 数据之间的关联时，可以使用 napi_remove_wrap。例如，在对象被垃圾回收之前，或者当你需要重新关联不同的数据到同一个 JavaScript 对象时，你可以调用这个函数来清理之前的关联。

#include <node_api.h>
#include <assert.h>

typedef struct {
  int value;
} my_struct;

napi_value CreateObject(napi_env env, napi_callback_info info) {
  napi_value obj;
  napi_create_object(env, &obj);

  my_struct* s = new my_struct();
  s->value = 123; // 假设这是我们要关联的数据

  // 将C结构体与JavaScript对象关联
  napi_status status = napi_wrap(env, obj, s, NULL, NULL, NULL);
  assert(status == napi_ok);

  return obj;
}

napi_value RemoveWrap(napi_env env, napi_callback_info info) {
  napi_value obj;
  size_t argc = 1;
  napi_get_cb_info(env, info, &argc, &obj, nullptr, nullptr);

  my_struct* s;
  napi_unwrap(env, obj, (void**)&s); // 先解包以获取原始指针

  // 移除关联的数据
  napi_remove_wrap(env, obj, s);

  delete s; // 清理分配的内存

  return nullptr;
}

napi_value Init(napi_env env, napi_value exports) {
  napi_property_descriptor desc[] = {
    { "createObject", 0, CreateObject, 0, 0, 0, napi_default, 0 },
    { "removeWrap", 0, RemoveWrap, 0, 0, 0, napi_default, 0 }
  };
  napi_define_properties(env, exports, sizeof(desc) / sizeof(*desc), desc);
  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)

```

#### **ANI 示例**
```
在 ANI 中，没有对应的napi_remove_wrap接口。

```



### napi_get_value_bigint_int64迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value GetValueBigintInt64t(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 从传入的参数中提取64位整数的BigInt数据
    int64_t value = 0;
    bool lossLess = false;
    napi_status status = napi_get_value_bigint_int64(env, args[0], &value, &lossLess);
    // 判断从ArkTS侧获取bigint是否为无损转换，如果不是抛出异常
    if (!lossLess) {
        napi_throw_error(env, nullptr, "BigInt values have not been lossless converted");
        return nullptr;
    }
    // 如果接口调用成功正常调用则返回true给ArkTS侧
    napi_value returnValue = nullptr;
    napi_get_boolean(env, status == napi_ok, &returnValue);
    return returnValue;
}
```

```ts
// index.d.ts
export const getValueBigintInt64t: (bigInt64: bigint) => boolean;

// ets
let bigInt = BigInt(9223372036854775807n);
testNapi.getValueBigintInt64t(bigInt);
```

#### **ANI 示例**
```cpp
// sts
function GetBigint() {
    let num : bigint = -9223372036854775807n;
    return num;
}

// cpp
auto bigintRef = CallEtsFunction<ani_ref>("GetBigint");
ani_object bigintNum = static_cast<ani_object>(bigintRef);
ani_class bigIntCls;
const char * className = "Lescompat/BigInt;";
if (ANI_OK != env_->FindClass(className, &bigIntCls)) {
    std::cerr << "Not found '" << className << "'" << std::endl;
    return ;
}
ani_method getLongMethod;
if (ANI_OK != env_->Class_GetMethod(bigIntCls, "getLong", ":J", &getLongMethod)){
    std::cerr << "Class_GetMethod Failed '" << className << "'" << std::endl;
    return ;
}

ani_long longNum;
if (ANI_OK != env_->Object_CallMethod_Long(bigintNum, getLongMethod, &longNum)){
    std::cerr << "Object_CallMethod_Long '" << "getLongMethod" << "'" << std::endl;
    return ;
}
```




### napi_get_value_bigint_uint64迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value GetValueBigintUint64t(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 从参数值中获取BigInt的数值
    uint64_t value = 0;
    bool lossLess = false;
    napi_status status = napi_get_value_bigint_uint64(env, args[0], &value, &lossLess);
    // 判断从ArkTS侧获取bigint是否为无损转换，如果不是抛出异常
    if (!lossLess) {
        napi_throw_error(env, nullptr, "BigInt values have no lossless converted");
        return nullptr;
    }
    // 如果接口调用成功正常调用则返回true给ArkTS侧
    napi_value returnValue = nullptr;
    napi_get_boolean(env, status == napi_ok, &returnValue);
    return returnValue;
}
```

```ts
// index.d.ts
export const getValueBigintUint64t: (bigUint64: bigint) => boolean;

// ets
let bigUint = BigInt(9223372036854775807n);
testNapi.getValueBigintUint64t(bigUint);
```

#### **ANI 示例**
```cpp
// sts
function GetBigint() {
    let num : bigint = 9223372036854775807n;
    return num;
}

// cpp
auto bigintRef = CallEtsFunction<ani_ref>("GetBigint");
ani_object bigintNum = static_cast<ani_object>(bigintRef);
ani_class bigIntCls;
const char * className = "Lescompat/BigInt;";
if (ANI_OK != env_->FindClass(className, &bigIntCls)) {
    std::cerr << "Not found '" << className << "'" << std::endl;
    return ;
}
ani_method getULongMethod;
if (ANI_OK != env_->Class_GetMethod(bigIntCls, "getULong", ":J", &getULongMethod)){
    std::cerr << "Class_GetMethod Failed '" << className << "'" << std::endl;
    return ;
}

ani_long longNum;
if (ANI_OK != env_->Object_CallMethod_Long(bigintNum, getULongMethod, &longNum)){
    std::cerr << "Object_CallMethod_Long '" << "getLongMethod" << "'" << std::endl;
    return ;
}
```



### napi_object_freeze迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```
napi_object_freeze 是 N-API 中的一个函数，用于冻结一个 JavaScript 对象。被冻结的对象不能添加新的属性、删除已有属性、更改可枚举性、可配置性或可写性，也不能重新定义属性的值（除非属性是可写的）。这与 JavaScript 中的 Object.freeze() 方法功能相同。

napi_status napi_object_freeze(napi_env env, napi_value object);
env: 当前的 N-API 环境。
object: 要冻结的 JavaScript 对象。
返回值是一个 napi_status 类型的变量，表示操作的状态。如果成功，则返回 napi_ok；否则，返回错误代码。


#include <node_api.h>
#include <assert.h>

napi_value CreateFrozenObject(napi_env env, napi_callback_info info) {
  // 创建一个新的空对象
  napi_value obj;
  napi_create_object(env, &obj);

  // 向对象中添加一些属性
  napi_value value;
  napi_create_int32(env, 100, &value);
  napi_set_named_property(env, obj, "number", value);

  // 冻结对象
  napi_status status = napi_object_freeze(env, obj);
  assert(status == napi_ok);

  return obj;
}

napi_value Init(napi_env env, napi_value exports) {
  napi_value fn;
  napi_create_function(env, nullptr, 0, CreateFrozenObject, nullptr, &fn);
  napi_set_named_property(env, exports, "createFrozenObject", fn);
  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)
```

#### **ANI 示例**
```
ArkTS 作为一种静态类型语言，在编译期就已经确定了类的结构，不允许随意更改对象的属性或方法。所以不存在这种接口。
```


### napi_object_seal迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```
napi_object_seal 是 N-API 提供的另一个函数，用于密封（seal）一个 JavaScript 对象。密封对象与冻结对象类似，但有一些关键的区别。密封对象不允许添加新属性或删除已有属性，但是可以修改现有属性的值（前提是这些属性是可写的）。换句话说，napi_object_seal 阻止了对对象结构的改变（即添加或删除属性），但并不阻止对属性值的更改。这与 JavaScript 中的 Object.seal() 方法功能相同。

env: 当前的 N-API 环境。
object: 要密封的 JavaScript 对象。
返回一个 napi_status 类型的变量，表示操作的状态。如果成功，则返回 napi_ok；否则，返回错误代码。
下面是一个简单的例子，展示了如何使用 napi_object_seal 来密封一个对象：

#include <node_api.h>
#include <assert.h>

napi_value CreateSealedObject(napi_env env, napi_callback_info info) {
  // 创建一个新的空对象
  napi_value obj;
  napi_create_object(env, &obj);

  // 向对象中添加一些属性
  napi_value value;
  napi_create_int32(env, 100, &value);
  napi_set_named_property(env, obj, "number", value);

  // 密封对象
  napi_status status = napi_object_seal(env, obj);
  assert(status == napi_ok);

  return obj;
}

napi_value Init(napi_env env, napi_value exports) {
  napi_value fn;
  napi_create_function(env, nullptr, 0, CreateSealedObject, nullptr, &fn);
  napi_set_named_property(env, exports, "createSealedObject", fn);
  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)

```

#### **ANI 示例**
```
ArkTS 作为一种静态类型语言，在编译期就已经确定了类的结构，不允许随意更改对象的属性或方法。所以不存在这种接口。
```


### napi_wrap_enhance迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**
```
暂未找到对应接口
```


## 7. Accessing Fields of Objects

### napi_get_named_property迁移比对

#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value GetField(napi_env env, napi_value object, const char* fieldName) {
    napi_value result;
    napi_status status = napi_get_named_property(env, object, fieldName, &result);
    if (status != napi_ok) {
        return nullptr;
    }
    return result;
}
```
#### **ANI 示例**
```cpp
// ets
class Point {
    x: int;
    y: int;
}

// cpp
ani_class cls;
env_->FindClass("LPoint;", &cls);

ani_field fieldX;
env_->Class_GetField(cls, "x", &fieldX);

ani_field fieldY;
env_->Class_GetField(cls, "y", &fieldY);


```

### napi_get_property迁移示例
#### 函数功能比对

#### 代码示例对比

#### **N-API 示例**
```cpp
// js
const obj = { 0: 1234567890123 };

// cpp
napi_value obj;
napi_create_object(env, &obj);

napi_value value;
napi_create_int64(env, 1234567890123, &value);

napi_value key;
napi_create_uint32(env, 0, &key);
napi_set_property(env, obj, key, value);

napi_value prop;
int64_t result;
napi_get_property(env, obj, key, &prop);
napi_get_value_int64(env, prop, &result);
```
#### **ANI 示例**
```cpp
// ets
class Woman {
    constructor(name: String, age: long) {
        this.name = name;
        this.age = age;
    }
    name: String;
    age: long;
}

function newSarahObject() {
    return new Woman("Sarah", 24);
}

// cpp
void GetData(ani_object *objectResult, ani_field *fieldNameResult, ani_field *fieldAgeResult)
{
    auto sarahRef = CallEtsFunction<ani_ref>("newSarahObject");
    auto sarah = static_cast<ani_object>(sarahRef);

    ani_class cls;
    ASSERT_EQ(env_->FindClass("LWoman;", &cls), ANI_OK);

    ani_field fieldName;
    ASSERT_EQ(env_->Class_GetField(cls, "name", &fieldName), ANI_OK);

    ani_field fieldAge;
    ASSERT_EQ(env_->Class_GetField(cls, "age", &fieldAge), ANI_OK);

    *objectResult = sarah;
    *fieldNameResult = fieldName;
    *fieldAgeResult = fieldAge;
}

*fieldAgeResult = fieldAge;
ani_object sarah {};
ani_field field {};
ani_field fieldAge {};
GetData(&sarah, &field, &fieldAge);
ani_long age {};

env_->Object_GetField_Long(sarah, fieldAge, &age);
ASSERT_EQ(age, 24L);
```

### napi_set_property迁移示例

#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value obj;
napi_create_object(env, &obj);

napi_value key;
napi_create_uint32(env, 0, &key);

napi_value value;
napi_create_int64(env, 1234567890123, &value);

// 设置属性 obj[0] = 1234567890123
napi_set_property(env, obj, key, value);
```
#### **ANI 示例**
```cpp
// ets
class Package {
    long_value: long = 0;
}

function newPackageObject() {
    return new Package();
}

function checkLongValue(p: Package, value: long): boolean {
    return p.long_value == value;
}


// cpp
void GetData(ani_object *packResult, ani_field *fieldLongResult)
{
    auto packRef = CallEtsFunction<ani_ref>("newPackObject");

    ani_class cls;
    ASSERT_EQ(env_->FindClass("LPackage;", &cls), ANI_OK);

    ani_field fieldLong;
    ASSERT_EQ(env_->Class_GetField(cls, "long_value", &fieldLong), ANI_OK);

    *packResult = static_cast<ani_object>(packRef);
    *fieldLongResult = fieldLong;
}

ani_object pack;
ani_field fieldLong;
GetTestData(&pack, &fieldLong);

CallEtsFunction<ani_boolean>("checkLongValue", pack, ani_long(0));
env_->Object_SetField_Int(pack, fieldLong, 8L);
CallEtsFunction<ani_boolean>("checkLongValue", pack, ani_long(8L));
```

### napi_get_named_property迁移示例

#### 代码示例对比

#### **N-API 示例**
```cpp
// js
const obj = { intProperty: 42 };

// cpp
napi_value obj; // 已与obj绑定
napi_status status;
// 获取对象的“intProperty”属性
status = napi_get_named_property(env, obj, "intProperty", &result);
```
#### **ANI 示例**
```cpp
// ets
class Animal {
    constructor(name: String, age: int) {
        this.name = name;
        this.age = age;
    }
    name: String;
    age: int;
}

function newAnimalObject() {
    return new Animal("Cat", 2);
}

// cpp
ani_object NewAnimal()
{
    auto animalRef = CallEtsFunction<ani_ref>("newAnimalObject");
    return static_cast<ani_object>(animalRef);
}
ani_object animal = NewAnimal();
ani_int age;
ASSERT_EQ(env_->Object_GetFieldByName_Int(animal, "age", &age), ANI_OK);
ASSERT_EQ(age, 2U);
```
#### **ANI 示例2**
可选参数会对于原来的数值类型，会自动的变成装箱的类型 如 int -> Int 目的是为了有 undefined的状态（没有设置）
参考以下代码片段
```C++
//this.optionField 现在是一个对象，要使用 Object_GetFieldByName_Ref
    ani_ref int_ref;
    if(ANI_OK != env->Object_GetFieldByName_Ref(object, "optionField", &int_ref)){
        std::cerr << "Object_GetFieldByName_Ref optionField Failed" << std::endl;
    }

    //判断是否是undefined
    ani_boolean isUndefined;
    if(ANI_OK != env->Reference_IsUndefined(int_ref,&isUndefined)){
        std::cerr << "Object_GetFieldByName_Ref optionField Failed" << std::endl;
        return ;
    }

    if(isUndefined){
        std::cout << "optionField is Undefined Now" << std::endl;
        return;
    }

    //从Int对象中解出数值
    ani_int int_value;
    if(ANI_OK != env->Object_CallMethodByName_Int(static_cast<ani_object>(int_ref), "intValue", nullptr,&int_value)){
        std::cerr << "Object_GetFieldByName_Ref optionField Failed" << std::endl;
        return;
    }
    std::cout << "optionField is:" <<  int_value  << std::endl;
```

### napi_set_named_property迁移示例

#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value obj;
napi_create_object(env, &obj);  // 创建一个空对象

napi_value napiValue;
napi_create_int32(env, 42, &napiValue);  // 创建一个整数值为42的napi_value

napi_set_named_property(env, obj, "myField", napiValue);  // 设置对象的 "myField" 字段为42
```
#### **ANI 示例**
```cpp
// ets
class Animal {
    constructor(name: String, age: int) {
        this.name = name;
        this.age = age;
    }
    name: String;
    age: int;
}

function newAnimalObject() {
    return new Animal("Cat", 2);
}

// cpp
ani_object NewAnimal()
{
    auto animalRef = CallEtsFunction<ani_ref>("newAnimalObject");
    return static_cast<ani_object>(animalRef);
}
ani_object animal = NewAnimal();
ASSERT_EQ(env_->Object_SetFieldByName_Int(animal, "age", 20U), ANI_OK);
ani_int age;
ASSERT_EQ(env_->Object_GetFieldByName_Int(animal, "age", &age), ANI_OK);
ASSERT_EQ(age, 20U);
```


### napi_get_named_property迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


### napi_get_property_names迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


### napi_define_properties迁移示例
---
ANI为静态语言服务，因此无需动态地进行对象操作，在对象生成后编辑对象的内存结构。
ANI中需要STS侧直接定义类的完整属性，随后使用构造函数创建对象，再赋值给字段。
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_property_descriptor descriptors[] = {
  { "method", NULL, MyMethod, NULL, NULL, NULL, napi_default, NULL },
  { "value", NULL, NULL, NULL, NULL, MyValue, napi_default, NULL }
};
napi_define_properties(env, obj, 2, descriptors);
```

#### **ANI 示例**
```TS
class MobilePhone {
    constructor(model: String, weight: int) {
        this.model = model;
        this.weight = weight;
    }
    model: String;
    weight: int;
}
```

```cpp
ani_class cls;
env->FindClass("LMobilePhone;", &cls)
ani_method ctor;
env->Class_GetMethod(cls, "<ctor>", "Lstd/core/String;I:V", &ctor)
env->Object_New(cls, ctor, &phone, model, weight)
```


### napi_get_own_property_descriptor迁移示例
---
迁移方式参考napi_define_properties。



### napi_get_all_property_names迁移示例
---
napi该反射特性在ANI中不支持，ANI需要已知目标类或对象的属性名进行动作。
可以根据需求模拟部分后续的目的行为。
property无法通过ANI配置仅获取own原型，将会按照继承关系进行获取。
property无法通过ANI配置仅获取特定可写入类型等，仅可以通过Object_GetPropertyByName_Boolean，Object_GetPropertyByName_Int获取指定类型返回值的字段。
property获取时的key转换需要自行实现。
如需要获取对象property的值，请使用Object_GetPropertyByName_Boolean等方法，可以按字段名进行获取。
如需要设置对象property的值，请使用Object_SetPropertyByName_Boolean等方法，可以按字段名进行设置。


### napi_has_property迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value HasProperty(napi_env env, napi_callback_info info)
{
    // 从ArkTS侧传入两个参数：第一个参数为要检验的对象，第二个参数为要检测是否存在对象的属性
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // 将参数传入napi_has_property方法中，若接口调用成功则将结果转化为napi_value类型抛出，否则抛出错误
    bool result;
    napi_status status = napi_has_property(env, args[0], args[1], &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_has_property fail");
        return nullptr;
    }

    // 若传入属性存在传入对象中，则输出true将结果转化为napi_value类型抛出
    napi_value returnResult;
    napi_get_boolean(env, result, &returnResult);
    return returnResult;
}
```

```ts
// index.d.ts
export const hasProperty: (obj: Object, key: string) => boolean;

// ets
class Obj {
    data: number = 0
    message: string = ""
}
let obj: Obj = { data: 0, message: "hello world"};
let flag = testNapi.hasProperty(obj, "data");
```

#### **ANI 示例**
```cpp
// ets
class Obj {
    constructor(data: number, message: string) {
        this.data = data;
        this.message = message;
    }
    data: number;
    message: string;
}

function newObject() {
    return new Obj(0, "hello world");
}

// cpp
ani_property property // 创建 TODO
ani_boolean model;
auto objRef = CallEtsFunction<ani_ref>("newObject");
ani_object obj = static_cast<ani_object>(objRef);

if (env->Object_GetProperty_Boolean(obj, property, &model) == ANI_NOT_FOUND){
    // 没有这个property
} else {
    // 有这个property
}
```


### napi_delete_property迁移示例
---
由于静态语言特性，不支持动态改变对象内存结构。
可以通过实现一个减少对应属性的类，重新创建一个新对象。
该方式可以替代napi进行迁移。
#### 代码示例对比

#### **N-API 示例**
```TS
// 从传入的Object对象中删除指定属性，返回是否删除成功的bool结果值
static napi_value DeleteProperty(napi_env env, napi_callback_info info)
{
    // 接收两个ArkTS传来的参数
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    napi_valuetype valueType;
    napi_typeof(env, args[0], &valueType);
    if (valueType != napi_object) {
        napi_throw_error(env, nullptr, "Expects an object as argument.");
        return nullptr;
    }
    // 删除指定属性，结果存储在result中
    bool result = false;
    napi_status status = napi_delete_property(env, args[0], args[1], &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_delete_property failed");
        return nullptr;
    }
    // 将bool结果转换为napi_value并返回
    napi_value ret;
    napi_get_boolean(env, result, &ret);
    return ret;
}
```

#### **ANI 示例**
```TS
Class Origin {
    public:
    int a = 0;
    int b = 1;
    constructor(){}
}
Class OriginDeleteA {
    public:
    int b = 1;
    constructor(){}
}
```

```CPP
ani_class cls;
ASSERT_EQ(env_->FindClass("LOriginDeleteA;", &cls), ANI_OK);
ani_method ctor;
ASSERT_EQ(env_->Class_FindMethod(cls, "<ctor>", "V:V", &ctor), ANI_OK);
ani_object objOriginDeleteA;
env->Object_New(env_, cls, ctor, &phone);
```




### napi_has_own_property迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
```cpp
// cpp
static napi_value HasOwnProperty(napi_env env, napi_callback_info info)
{
    // 从ArkTS侧传入两个参数：第一个参数为要检验的对象，第二个参数为要检测是否存在对象的属性
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // 将参数传入napi_has_property方法中，若接口调用成功则将结果转化为napi_value类型抛出，否则抛出错误
    bool result;
    napi_status status = napi_has_own_property(env, args[0], args[1], &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_has_own_property fail");
        return nullptr;
    }

    // 若传入属性存在传入对象中，则输出true将结果转化为napi_value类型抛出
    napi_value returnResult;
    napi_get_boolean(env, result, &returnResult);
    return returnResult;
}
```

```ts
// index.d.ts
export const hasOwnProperty: (obj: Object, key: string) => boolean;

// ets
class Obj {
    data: number = 0
    message: string = ""
}
let obj: Obj = { data: 0, message: "hello world"};
let flag = testNapi.hasOwnProperty(obj, "data");
```

#### **ANI 示例**
```cpp
// ets
```cpp
// ets
class Obj {
    constructor(data: number, message: string) {
        this.data = data;
        this.message = message;
    }
    data: number;
    message: string;
}

function newObject() {
    return new Obj(0, "hello world");
}

// cpp
ani_property property // 创建 TODO
ani_boolean model;
auto objRef = CallEtsFunction<ani_ref>("newObject");
ani_object obj = static_cast<ani_object>(objRef);

if (env->Object_GetProperty_Boolean(obj, property, &model) == ANI_NOT_FOUND){
    // 没有这个property
} else {
    // 有这个property
}
```




### napi_has_element迁移示例
---
ANI的API可以返回的值都可以检查其是否为nullptr等无效情况来确认是否存在对应的元素。
尽管没有直接对应的ANI函数，但可以结合遍历进行查找，实现napi函数对应的功能。
#### 代码示例对比

#### **N-API 示例**
```CPP
bool hasElement;
napi_has_element(env, jsObject, index, &hasElement);
```

#### **ANI 示例**
```CPP
bool HasElement(ani_env env, ani_fixedarray_ref arr, ani_ref value)
{
    ani_size size;
    env->FixedArray_GetLength(arr, &size);
    auto getValue = nullptr;
    for (ani_size i = 0; i < size; ++i) {
        env->FixedArray_Get_Ref(arr, i, getValue);
        if (value == getValue) {
            return true;
        }
        getValue = nullptr;
    }
    return false;
}
```


### napi_delete_element迁移示例
---
ANI的API可以返回的值都可以检查其是否为nullptr等无效情况来确认是否存在对应的元素。
尽管没有直接对应的ANI函数，但可以结合遍历进行查找，实现napi函数对应的功能。
#### 代码示例对比

#### **N-API 示例**
```CPP
bool hasElement;
napi_has_element(env, jsObject, index, &hasElement);
```

#### **ANI 示例**
```CPP
bool DelElement(ani_env env, ani_fixedarray_ref arr, index i)
{
    ani_size size;
    env->FixedArray_GetLength(arr, &size);
    statu;
    if (i < 0 || i >= size) {
        return false;
    }
    if (ani_statu statu = env->FixedArray_Set_Ref(arr, i, nullptr) == ANI_OK) {
        return true;
    } else {
        return false;
    }
}
```


### napi_object_get_keys迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




## 8. Calling Instance Methods
### napi_call_function迁移示例
---
#### 代码示例对比

#### **N-API 示例1**
```CPP
// 调用 obj 的 add 方法
napi_value add_result;
napi_value args[2];
args[0] = napi_create_int32(env, 20);
args[1] = napi_create_int32(env, 22);

status = napi_call_function(env, obj, "add", 2, args, &add_result);
if (status != napi_ok) {
    return -1;
}
```

#### **ANI 示例1**
```TS
// sts
class A {
    public static new_A()
    {
        return new A();
    }

    public int_method(a0: int, a1: int): int
    {
        return a0 + a1;
    }
}

// cpp
ani_static_method newMethod;
env_->Class_GetStaticMethod(cls, "new_A", ":LA;", &newMethod);
ani_ref ref;
env_->Class_CallStaticMethod_Ref(cls, newMethod, &ref);
object = static_cast<ani_object>(ref);
ani_method method;
env_->Class_GetMethod(cls, "int_method", "II:I", &method);

ani_int sum;
env_->c_api->Object_CallMethod_Int(env_, object, method, &sum, 2, 3);
```

#### **N-API 示例2**
```cpp
napi_value fn;  // 假设已经获得 JavaScript 函数引用
bool result;
napi_value arg1 = ..., arg2 = ...;  // 假设有两个参数
napi_call_function(env, nullptr, fn, 2, (napi_value[]){arg1, arg2}, &napi_result);
napi_get_value_bool(env, napi_result, &result);  // 获取布尔结果
```

#### **ANI 示例2**
```cpp
// ets
namespace ops {
  export function checkConcat(a: boolean): boolean {
    return ops.concat("abc", "def") == "abcdef";
  }
  export function concat(str1: string, str2: string) {
    return str1 + str2;
  }
}

// cpp
ani_namespace ns {};
env_->FindNamespace("Lops;", &ns);
ani_function fn {};
env_->Namespace_FindFunction(ns, "checkConcat", "Z:Z", &fn);
ani_bool result;
env_->Function_Call_Boolean(env_, fn, &result, 1, 0);
```

#### **N-API 示例3**
```cpp
napi_value object;  // 假设已获得 JavaScript 对象引用
napi_value method;  // 假设已获得方法引用
bool result;
napi_value arg1 = ..., arg2 = ...;  // 假设有两个参数
napi_call_function(env, object, method, 2, (napi_value[]){arg1, arg2}, &napi_result);
napi_get_value_bool(env, napi_result, &result);  // 获取布尔返回值
```

#### **ANI 示例3**
```cpp
// ets
class A {
boolean_method(a0: int, a1: int): boolean {
    if (a0 + a1 > 10) {
        return true;
    } else {
        return false;
    }
}
}

// cpp
// Retrieve a method named "boolean_method" with signature "II:Z".
ani_method method; // 假定已获取method
ani_object object; // 假定已经获取Class A的Object
ani_boolean res;
ani_int arg1 = 2U;
ani_int arg2 = 3U;
// Call the method and verify the return value.
// NOLINTNEXTLINE(cppcoreguidelines-pro-type-vararg)
env_->Object_CallMethod_Boolean(env_, object, method, &res, arg1, arg2);
ASSERT_EQ(res, ANI_FALSE);
```

#### **N-API 示例4**
```cpp
// js
class MyClass {
    isEven(number) {
        return number % 2 === 0;
    }
}
// cpp
napi_status status;
napi_value myClassInstance;     // 假设通过构造函数创建了 MyClass 的实例
napi_value isEvenMethod;
status = napi_get_named_property(env, myClassInstance, "isEven", &isEvenMethod);
napi_value arg;
status = napi_create_int32(env, 42, &arg);  // 创建整数参数 42
napi_value args[] = { arg };  // 将参数传递给方法
status = napi_call_function(env, myClassInstance, isEvenMethod, 1, args, &result);
status = napi_get_value_bool(env, result, &isEvenResult);
```
#### **ANI 示例4**
```cpp
// ets
class A {
boolean_method(a0: int, a1: int): boolean {
    if (a0 + a1 > 10) {
        return true;
    } else {
        return false;
    }
}
}

// cpp
// Retrieve a method named "boolean_method" with signature "II:Z".
ani_object object; // 假定已经获取Class A的Object
ani_boolean res;
ani_int arg1 = 2U;
ani_int arg2 = 3U;
// Call the method and verify the return value.
// NOLINTNEXTLINE(cppcoreguidelines-pro-type-vararg)
env_->Object_CallMethodByName_Boolean(env_, object, "boolean_method", "II:Z",&res, arg1, arg2);
ASSERT_EQ(res, ANI_FALSE);
```

#### **N-API 示例5**
```cpp
// js
export class TsClass { // 这里定义一个类
  public static TsMethod(): void { // 这里定义一个静态方法
    log.info('do static TsMethod');
  }
}
// cpp
napi_value js_Class_name; // 假定类名已获取
napi_value staticMethod; // 直接调用类名下的静态方法
napi_get_named_property(env, js_Class_name, "TsMethod", &staticMethod);
napi_call_function(env, js_Class_name, staticMethod, 0, nullptr, nullptr);
```
#### **ANI 示例5**
```cpp
// ets
class Operations {
    static or(a0: boolean, a1: boolean): boolean {
        return a0 || a1;
    }
};

// cpp
ani_class cls;
env_->FindClass("LOperations;", &cls);
ani_static_method method;
env_->Class_GetStaticMethod(cls, "or", "ZZ:Z", &method);
ani_boolean result;
env_->Class_CallStaticMethod_Boolean(cls, method, &result, ANI_TRUE, ANI_FALSE);
ASSERT_EQ(result, ANI_TRUE);
```

#### **N-API 示例6**
```cpp
// js
export class TsClass { // 这里定义一个类
  public static TsMethod(): void { // 这里定义一个静态方法
    log.info('do static TsMethod');
  }
}
// cpp
napi_value js_Class_name; // 假定类名已获取
napi_value staticMethod; // 直接调用类名下的静态方法
napi_get_named_property(env, js_Class_name, "TsMethod", &staticMethod);
napi_call_function(env, js_Class_name, staticMethod, 0, nullptr, nullptr);
```
#### **ANI 示例6**
```cpp
// ets
class Operations {
    static or(a0: boolean, a1: boolean): boolean {
        return a0 || a1;
    }
};

// cpp
ani_class cls;
env_->FindClass("LOperations;", &cls);
ani_boolean result;
env_->Class_CallStaticMethodByName_Boolean(cls, "or", &result, ANI_TRUE, ANI_FALSE);
ASSERT_EQ(result, ANI_TRUE);
```

#### **N-API 示例7**
```cpp
// js
export class TsClass { // 这里定义一个类
  public static TsMethod(): void { // 这里定义一个静态方法
    log.info('do static TsMethod');
  }
}
// cpp
napi_value js_Class_name; // 假定类名已获取
napi_value staticMethod; // 直接调用类名下的静态方法
napi_get_named_property(env, js_Class_name, "TsMethod", &staticMethod);
napi_call_function(env, js_Class_name, staticMethod, 0, nullptr, nullptr);
```

#### **ANI 示例7**
```cpp
// ets
class B {
    constructor(root : int) {
        this.root = root;
    }
    root : int
}

class Foo {
    static Add(a0: int, a1: int): int
    {
        return a0 + a1;
    }

    static Add(a0: int, a1: B): int
    {
        return a0;
    }
}

// cpp
static const char *className = "Lsetfield/Foo;";
ani_class cls;
if (ANI_OK != env->FindClass(className, &cls)) {
    std::cerr << "Not found '" << className << "'" << std::endl;
    return ;
}

ani_class cls1;
if (env->FindClass("Lsetfield/B;", &cls1) != ANI_OK) {
    std::cerr << "Not found '" << "B" << "'" << std::endl;
    return ;
}

ani_method ctor1;
if (env->Class_FindMethod(cls1, "<ctor>", "I:V", &ctor1)  != ANI_OK) {
    std::cerr << "Not found '" << "B ctor" << "'" << std::endl;
    return ;
}
ani_object b;
ani_int root = 1;
if (env->c_api->Object_New(env, cls1, ctor1, &b, root)!= ANI_OK) {
    std::cerr << "Not create '" << "B ctor" << "'" << std::endl;
    return ;
}

ani_static_method method;
if (env->Class_FindStaticMethod(cls, "Add", "ILsetfield/B;:I", &method)!= ANI_OK) {
    std::cerr << "Not found '" << "Add" << "'" << std::endl;
    return ;
}

ani_int sum;
if (env->c_api->Class_CallStaticMethodByName_Int(env, cls, "Add", "II:I", &sum, 5U, 6U) != ANI_OK) {
    std::cerr << "Not call '" << "Add_II:I" << "'" << std::endl;
    return ;
}

ani_int sum1;
if (env->c_api->Class_CallStaticMethodByName_Int(env, cls, "Add", "ILsetfield/B;:I", &sum1, 5U, &b) != ANI_OK) {
    std::cerr << "Not call '" << "Add_ILsetfield/B;:I" << "'" << std::endl;
    return ;
}
```

#### **ANI 示例8**
CPP层调用ETS传入的函数对象。
https://gitee.com/openharmony/arkcompiler_runtime_core/issues/IBPZLT

```ts
native function handleData(a:()=>int):int

function main(){
    loadLibrary("ani_test")
    let f = () => { return 42;}
    let rs = handleData(f);
    console.log(rs); //42
}
```

```CPP
ani_int handleData(ani_env *env,[[maybe_unused]] ani_object obj, ani_object funcObj1)
{
    ani_ref ref_int;
    env->Object_CallMethodByName_Ref(funcObj1, "invoke0", ":Lstd/core/Object;", &ref_int);
    ani_int rs_int;
    env->Object_CallMethodByName_Int(static_cast<ani_object>(ref_int), "unboxed", ":I", &rs_int);
    return rs_int; // 42
}

ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    vm->GetEnv(ANI_VERSION_1, &env);
    static const char *className = "Lani_test/ETSGLOBAL;";
    ani_class cls;
    ANI_OK != env->FindClass(className, &cls);
    std::array methods = {
    //函数签名Function0会根据参数返回值的数量进行变化，请反编译确认当前具体的FunctionX
        ani_native_function{"handleData", "Lstd/core/Function0;:I", reinterpret_cast<void *>(handleData)},
    };
    env->Class_BindNativeMethods(cls, methods.data(), methods.size());
    *result = ANI_VERSION_1;
    return ANI_OK;
}
```


### napi_get_cb_info迁移示例
---
ANI中不需要专门的函数进行解析参数。在声明函数时已经自动进行可变参数解析。
ANI在cpp层实现的native函数，第一个入参是env，第二个入参是native声明所属的class对象，随后的入参是ets层native函数的入参，将自动按顺序进行解析。

#### 代码示例对比

#### **N-API 示例**
```TS
const calc = requireInternal('calc');
const number = requireNapi('number');
function sub(x, y)
{
    return x - y;
};
export default {
    add: calc.add,
    sub: sub,
    ValueConstant: {
        TYPE_VALUE_0: 0,
        TYPE_VALUE_1: number.Number.ONE,
    }
};
```
```CPP
static napi_value Add(napi_env env, napi_callback_info info)
{
    size_t requireArgc = 2;
    size_t argc = 2;
    napi_value args[2] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));

    NAPI_ASSERT(env, argc >= requireArgc, "Wrong number of arguments");

    napi_valuetype valuetype0;
    NAPI_CALL(env, napi_typeof(env, args[0], &valuetype0));

    napi_valuetype valuetype1;
    NAPI_CALL(env, napi_typeof(env, args[1], &valuetype1));

    NAPI_ASSERT(env, valuetype0 == napi_number && valuetype1 == napi_number, "Wrong argument type. Numbers expected.");

    double value0;
    NAPI_CALL(env, napi_get_value_double(env, args[0], &value0));

    double value1;
    NAPI_CALL(env, napi_get_value_double(env, args[1], &value1));

    napi_value sum;
    NAPI_CALL(env, napi_create_double(env, value0 + value1, &sum));

    return sum;
}
```


#### **ANI 示例**
```TS
native function handleData(a: double, b:double):double
function main(){
    loadLibrary("ani_add")
    let a:double = 2.0;
    let b:double = 3.0;
    handleData(a+b);
}
```

```cpp
ani_double handleData_add(ani_env *env, ani_object obj /* native函数所属父对象 */, ani_double obj1 /* ets侧入参 */, ani_double obj2 /* ets侧入参 */){
    return obj1+obj2;
}

ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    vm->GetEnv(ANI_VERSION_1, &env);
    static const char *className = "Lani_add/ETSGLOBAL;";
    ani_class cls;
    env->FindClass(className, &cls);
    std::array methods = {
        ani_native_function {"handleData_add", "DD:D", reinterpret_cast<void *>(handleData_add)},
    };
    env->Class_BindNativeMethods(cls, methods.data(), methods.size());
    return ANI_OK;
}
```

####  **ANI 示例2**
cpp侧如何解析ets侧传入的对象?
对于sts侧传入对象，开发者应该已知该对象的具体类型，根据类型获取其方法和属性。
https://gitee.com/openharmony/arkcompiler_runtime_core/issues/IBPYA0

```ts
type DataType = string | Object | ArrayBuffer

native function handleData(data: DataType):void
function handleData(data: int):void{}
function handleData(data: Record<String, int>):void{}

function main(){
    loadLibrary("ani_union")
    handleData("hello")
    handleData(new ArrayBuffer(1024))
    handleData(new Array<int>)
}
```
cpp侧使用Object_InstanceOf解析其类型
```cpp
static void handleData_union(ani_env *env, ani_object obj /* native函数所属父对象 */, ani_object union_obj /* ets侧入参 */){
    ani_class stringClass;
    env->FindClass("Lstd/core/String;", &stringClass);

    ani_class arrayBufferClass;
    env->FindClass("Lescompat/ArrayBuffer;", &arrayBufferClass);

    ani_boolean isString;
    env->Object_InstanceOf(union_obj, stringClass, &isString);
    if(isString){
        auto stringContent = ANIUtils_ANIStringToStdString(env, static_cast<ani_string>(union_obj));
        std::cout << "Object is String Object Content:" << stringContent.c_str() << std::endl;
        return;
    }

    ani_boolean isArrayBuffer;
    env->Object_InstanceOf(union_obj, arrayBufferClass, &isArrayBuffer);
    if(isArrayBuffer){
        ani_int length;
        env->Object_CallMethodByName_Int(union_obj, "getByteLength", nullptr, &length);
        std::cout << "Object is ArraryBuffer Lenght:" << length << std::endl;
        return;
    }
}

ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    if (ANI_OK != vm->GetEnv(ANI_VERSION_1, &env)) {
        std::cerr << "Unsupported ANI_VERSION_1" << std::endl;
        return ANI_ERROR;
    }
    // 当前文件全局的 Function 实际时 ani_interface.ETSGLOBAL 这个class下的一个方法
    static const char *className = "Lani_union/ETSGLOBAL;";
    ani_class cls;
    if (ANI_OK != env->FindClass(className, &cls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return ANI_ERROR;
    }
    std::array methods = {
        ani_native_function {"handleData", "Lstd/core/Object;:V", reinterpret_cast<void *>(handleData_union)},
    };
    std::cout << "Start bind native methods to '" << className << "'" << std::endl;
    if (ANI_OK != env->Class_BindNativeMethods(cls, methods.data(), methods.size())) {
        std::cerr << "Cannot bind native methods to '" << className << "'" << std::endl;
        return ANI_ERROR;
    };
    std::cout << "Finish bind native methods to '" << className << "'" << std::endl;
    *result = ANI_VERSION_1;
    return ANI_OK;
}
```
#### **ANI 示例3**
napi_get_cb_info没有对应ANI借口，但是可以这样代替
```C++
static ani_ref findComponentSync(ani_env *env, ani_object obj, ani_object on_obj)
{
    ApiCallInfo callInfo_;
    ApiReplyInfo reply_;
    callInfo_.apiId_ = "Driver.findComponent";
    callInfo_.callerObjRef_ = aniStringToStdString(env, unwrapp(env, obj));
    callInfo_.paramList_.push_back(aniStringToStdString(env, unwrapp_on(env, on_obj)));
    g_apiTransactClient.Transact(callInfo_, reply_);
    ani_ref nativeComponent = UnmarshalReply(env, callInfo_, reply_);
    if (nativeComponent==nullptr) {
        std::cout<<"r nullptr" <<std::endl;
    }
    ani_object component_obj;
    static const char *className = "Luitest_ani/Component;";
    ani_class cls = findCls(env, className);
    ani_method ctor;
    if (cls) {
        static const char *name = "Lstd/core/String;:V";
        ctor = findCtorMethod(env, cls, name);
    }
    if (cls == nullptr || ctor==nullptr) {
        std::cout<<299<<std::endl;
        return nullptr;
    }
    if (ANI_OK !=env->Object_New(cls, ctor, &component_obj, reinterpret_cast<ani_object>(nativeComponent))) {
        std::cerr << "New Component Fail" << std::endl;
    }
    return component_obj;
}

static ani_class findCls(ani_env *env, const char *className)
{
    ani_class cls;
    ani_ref nullref;
    env->GetNull(&nullref);
    if (ANI_OK != env->FindClass(className, &cls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return cls;
    }
    std::cout << "found '" << className << "'" << std::endl;
    return cls;
}

static ani_method findCtorMethod(ani_env *env, ani_class cls, const char *name)
{
    ani_method ctor;
    if(ANI_OK != env->Class_FindMethod(cls, "", name, &ctor)){
        std::cerr << "Not found '" << "ctor" << "'" << std::endl;
        return ctor;
    }
    std::cout << "found '" << "ctor" << "'" << std::endl;
    return ctor;
}
```

### napi_get_new_target迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_get_new_target用来获取构造函数的new.target。
```C++
napi_value MyObject::New(napi_env env, napi_callback_info info)
{
  napi_value newTarget;
  napi_get_new_target(env, info, &newTarget);
  if (newTarget != nullptr) {
    // 使用`new MyObject(...)`调用方式
    size_t argc = 1;
    napi_value args[1];
    napi_value jsThis;
    napi_get_cb_info(env, info, &argc, args, &jsThis, nullptr);

    double value = 0.0;
    napi_valuetype valuetype;
    napi_typeof(env, args[0], &valuetype);
    if (valuetype != napi_undefined) {
      napi_get_value_double(env, args[0], &value);
    }

    MyObject* obj = new MyObject(value);

    obj->env_ = env;
    // 通过napi_wrap将ArkTS对象jsThis与C++对象obj绑定
    napi_status status = napi_wrap(env,
                                   jsThis,
                                   reinterpret_cast<void*>(obj),
                                   MyObject::Destructor,
                                   nullptr,  // finalize_hint
                                   &obj->wrapper_);
    // napi_wrap失败时，必须手动释放已分配的内存，以防止内存泄漏
    if (status != napi_ok) {
      OH_LOG_INFO(LOG_APP, "Failed to bind native object to js object"
                  ", return code: %{public}d", status);
      delete obj;
      return jsThis;
    }
    // 从napi_wrap接口的result获取napi_ref的行为，将会为jsThis创建强引用，
    // 若开发者不需要主动管理jsThis的生命周期，可直接在napi_wrap最后一个参数中传入nullptr，
    // 或者使用napi_reference_unref方法将napi_ref转为弱引用。
    uint32_t refCount = 0;
    napi_reference_unref(env, obj->wrapper_, &refCount);

    return jsThis;
  } else {
    // 使用`MyObject(...)`调用方式
    size_t argc = 1;
    napi_value args[1];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    napi_value cons;
    napi_get_reference_value(env, g_ref, &cons);
    napi_value instance;
    napi_new_instance(env, cons, argc, args, &instance);

    return instance;
  }
}
```
#### **ANI 示例**
在js中一个函数既可以用于构造函数，也可以用作普通函数，在arkTS中一个函数只能作为构造函数，或者能作为普通函数，工厂函数需要单独写。所以不存在这种接口。


### napi_define_class迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_define_class用于定义一个JavaScript类和其方法。通过这个函数，可以将C中定义的类和方法转换为JavaScript中可用的对象和函数。
```C++
napi_value MyObject::Init(napi_env env, napi_value exports)
{
  napi_property_descriptor properties[] = {
      { "value", 0, 0, GetValue, SetValue, 0, napi_default, 0 },
      { "plusOne", nullptr, PlusOne, nullptr, nullptr, nullptr, napi_default, nullptr }
  };

  napi_value cons;
  napi_define_class(env, "MyObject", NAPI_AUTO_LENGTH, New, nullptr, 2,
                           properties, &cons);

  napi_create_reference(env, cons, 1, &g_ref);
  napi_set_named_property(env, exports, "MyObject", cons);
  return exports;
}
```
#### **ANI 示例**
arkTS作为静态语言，不能在运行时创建类，需要在sts文件中显式地声明类。所以不存在这种接口。




## 9. Accessing Static Fields
### napi_get_named_property迁移示例

#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value cls;  // 假设已经有一个 class 对象 cls
napi_value result;
napi_status status;

// 获取静态字段 "staticField"
status = napi_get_named_property(env, cls, "staticField", &result);
```
#### **ANI 示例**
```cpp
// ets
class Singleton {
    static instance: int = 0;
}
ani_class cls;
env_->FindClass("LSingleton;", &cls);
ani_static_field field;
env_->Class_GetStaticField(cls, "instance", &field);
```

#### **N-API 示例2**
```cpp
napi_value cls;  // 假设已经有一个 class 对象 cls
napi_value result;
napi_status status;

// 获取静态字段 "staticField"
status = napi_get_named_property(env, cls, "staticField", &result);
```
#### **ANI 示例2**
```cpp
// ets
class Singleton {
    static instance: int = 0;
}

// cpp
ani_class cls;
env_->FindClass("LSingleton;", &cls);
ani_int result;
env_->Class_GetStaticFieldByName_Int(cls, "instance", &result);
```

#### **N-API 示例3**
```cpp
napi_value cls;  // 假设已经有一个 class 对象 cls
napi_value result;
napi_status status;

// 获取静态字段 "staticField"
status = napi_get_named_property(env, cls, "staticField", &result);
```
#### **ANI 示例3**
```cpp
// ets
class Singleton {
    static instance: int = 0;
}

// cpp
ani_class cls;
env_->FindClass("LSingleton;", &cls);
ani_int result;
env_->Class_SetStaticFieldByName_Int(cls, "instance", 20U);
env_->Class_GetStaticFieldByName_Int(cls, "instance", &result);
ASSERT_EQ(result, 20U);
```



### napi_get_instance_data迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_get_instance_data 检索出与当前运行的环境相关联的数据项。

```cpp
static napi_value GetInstanceData(napi_env env, napi_callback_info info) {
    InstanceData *resData = nullptr;
    // napi_get_instance_data获取之前想关联的数据项
    napi_get_instance_data(env, (void **)&resData);
    napi_value result;
    napi_create_int32(env, resData->value, &result);
    return result;
}
```

```ts
// index.d.ts
export const getInstanceData: () => number;

// ets
import hilog from '@ohos.hilog'
import testNapi from 'libentry.so'
let data = 5;
testNapi.setInstanceData(data);
let value = testNapi.getInstanceData();
hilog.info(0x0000, 'testTag', 'Test Node-API napi_set_instance_data:%{public}d', value);
```

#### **ANI 示例**

可以获取全局的一个对象后，使用一些列的`Object_SetField_*`，`Object_GetField_*`来访问或设置一些属性。
```ts
// ets file
class Pack {
    int_value: number = 0;
    string_value: string = "";
    bool_value: boolean = false;
}

function newPackObject(): Pack {
    return new Pack();
}

function checkBooleanValue(p: Pack, value: boolean): boolean {
    return p.bool_value === value;
}
```

```cpp
// cpp file
void GetTestData(ani_object *packResult, ani_field *fieldBoolResult, ani_field *fieldStringResult)
{
    auto packRef = CallEtsFunction<ani_ref>("newPackObject");

    ani_class cls;
    ASSERT_EQ(env_->FindClass("LPack;", &cls), ANI_OK);

    ani_field fieldBool;
    ASSERT_EQ(env_->Class_FindField(cls, "bool_value", &fieldBool), ANI_OK);

    ani_field fieldString;
    ASSERT_EQ(env_->Class_FindField(cls, "string_value", &fieldString), ANI_OK);

    *packResult = static_cast<ani_object>(packRef);
    *fieldBoolResult = fieldBool;
    *fieldStringResult = fieldString;
}

ani_object pack;
ani_field fieldBool;
ani_field fieldString;
GetTestData(&pack, &fieldBool, &fieldString);

ani_boolean boolValue;
ASSERT_EQ(env_->Object_GetField_Boolean(pack, fieldBool, &boolValue), ANI_OK);
ASSERT_EQ(boolValue, ANI_FALSE);
```

### napi_set_instance_data迁移示例
---
#### 代码示例对比

#### **N-API 示例**
绑定与当前运行的环境相关联的数据项。
```C++
typedef struct {
    size_t value;
    bool print;
    napi_ref js_cb_ref;
} AddonData;

static void DeleteAddonData(napi_env env, void* raw_data, void* hint)
{
    AddonData* data = (AddonData*)raw_data;
    if (data->print) {
        printf("deleting addon data\n");
    }
    if (data->js_cb_ref != NULL) {
        NAPI_CALL_RETURN_VOID(env, napi_delete_reference(env, data->js_cb_ref));
    }
    free(data);
}

AddonData* data = (AddonData*)malloc(sizeof(*data));
data->value = 41;
data->print = false;
data->js_cb_ref = NULL;
ASSERT_CHECK_CALL(napi_set_instance_data(env, data, DeleteAddonData, NULL));
```
#### **ANI 示例**
可以获取全局的一个对象后，使用一些列的`Object_SetField_*`，`Object_GetField_*`来访问或设置一些属性。
```C++
// ets file
class Pack {
    int_value: number = 0;
    string_value: string = "";
    bool_value: boolean = false;
}

function newPackObject(): Pack {
    return new Pack();
}

function checkBooleanValue(p: Pack, value: boolean): boolean {
    return p.bool_value === value;
}

// cpp file
void GetTestData(ani_object *packResult, ani_field *fieldBoolResult, ani_field *fieldStringResult)
{
    auto packRef = CallEtsFunction<ani_ref>("newPackObject");

    ani_class cls;
    ASSERT_EQ(env_->FindClass("LPack;", &cls), ANI_OK);

    ani_field fieldBool;
    ASSERT_EQ(env_->Class_FindField(cls, "bool_value", &fieldBool), ANI_OK);

    ani_field fieldString;
    ASSERT_EQ(env_->Class_FindField(cls, "string_value", &fieldString), ANI_OK);

    *packResult = static_cast<ani_object>(packRef);
    *fieldBoolResult = fieldBool;
    *fieldStringResult = fieldString;
}

ani_object pack;
ani_field fieldBool;
ani_field fieldString;
GetTestData(&pack, &fieldBool, &fieldString);

ASSERT_EQ(env_->Object_SetField_Boolean(pack, fieldString, ANI_TRUE), ANI_INVALID_TYPE);

```


## 10. Calling Static Methods


## 11. String Operations

### napi_get_value_string_utf8迁移示例
---
#### 代码示例对比

#### **N-API 示例**
获取给定JS vaule对应的UTF8编码的字符串。
```C++
const char testStr[] = "中文,English,123456,!@#$%$#^%&";
size_t testStrLength = strlen(testStr);
napi_value result = nullptr;
ASSERT_CHECK_CALL(napi_create_string_utf8(env, testStr, testStrLength, &result));
ASSERT_CHECK_VALUE_TYPE(env, result, napi_string);

char* buffer = nullptr;
size_t bufferSize = 0;
size_t strLength = 0;
ASSERT_CHECK_CALL(napi_get_value_string_utf8(env, result, nullptr, 0, &bufferSize));
ASSERT_GT(bufferSize, static_cast<size_t>(0));
buffer = new char[bufferSize + 1]{ 0 };
ASSERT_CHECK_CALL(napi_get_value_string_utf8(env, result, buffer, bufferSize + 1, &strLength));
ASSERT_STREQ(testStr, buffer);
ASSERT_EQ(testStrLength, strLength);
```
#### **ANI 示例**
使用`String_GetUTF8Size`替代。
```C++
const std::string example {"example"};
ani_string string = nullptr;
auto status = env_->String_NewUTF8(example.c_str(), example.size(), &string);
ASSERT_EQ(status, ANI_OK);

ani_size result = 0U;
status = env_->String_GetUTF8Size(string, &result);
ASSERT_EQ(status, ANI_OK);
ASSERT_EQ(result, example.size());
```

### napi_get_value_string_utf16迁移示例
---
#### 代码示例对比

#### **N-API 示例**
获取给定JS vaule对应的UTF16编码的字符串。
```C++
const char16_t testStr[] = u"中文,English,123456,!@#$%$#^%&12345";
int testStrLength = static_cast<int>(std::char_traits<char16_t>::length(testStr));
napi_value result = nullptr;
ASSERT_CHECK_CALL(napi_create_string_utf16(env, testStr, testStrLength, &result));
ASSERT_CHECK_VALUE_TYPE(env, result, napi_string);

char16_t* buffer = nullptr;
size_t bufferSize = 0;
size_t strLength = 0;
ASSERT_CHECK_CALL(napi_get_value_string_utf16(env, result, nullptr, 0, &bufferSize));
ASSERT_GT(bufferSize, (size_t)0);
char16_t* buffer = new char16_t[bufferSize + 1] { 0 };
ASSERT_CHECK_CALL(napi_get_value_string_utf16(env, result, buffer, bufferSize + 1, &strLength));
for (int i = 0; i < testStrLength; i++) {
    ASSERT_EQ(testStr[i], buffer[i]);
}
```

#### **ANI 示例**
使用`String_GetUTF16Size`替代。
```C++
const std::string example {"example"};
ani_string string = nullptr;
auto status = env_->String_NewUTF8(example.c_str(), example.size(), &string);
ASSERT_EQ(status, ANI_OK);

ani_size result = 0U;
status = env_->String_GetUTF16Size(string, &result);
ASSERT_EQ(status, ANI_OK);
ASSERT_EQ(result, example.size());
```

### napi_create_string_utf8迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value namestr;
napi_status status = napi_create_string_utf8(env, "abcdef", strlen("abcdef"), &namestr);
```

#### **ANI 示例**
```cpp
ani_string string {};
env_->String_NewUTF8("abcdef", 6U, &string);
```

### napi_create_string_latin1迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_string_latin1 需要通过ISO-8859-1编码的字符串创建ArkTS string值时使用这个函数。
```cpp
const char *str = "Hello, World! éçñ, successes to create Latin1 string! 111";
napi_value result = nullptr;
napi_create_string_latin1(env, str, strlen(str), &result);
```

#### **ANI 示例**

```cpp
// cpp
const char *str = "Hello, World! éçñ, successes to create Latin1 string! 111";
ani_string latin1String = nullptr;
env_->String_NewUTF8(str, strlen(str), &latin1String);
```

### napi_create_string_utf16迁移示例
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value CreateUTF16String(napi_env env, const char16_t* utf16_str, size_t length) {
    napi_value result;
    napi_status status = napi_create_string_utf16(einterpret_cast<const char16_t*>(utf16_str), length, &result);
    if (status != napi_ok) return nullptr;
    return result;
}
```
#### **ANI 示例**
```cpp
const std::u16string example_utf16 = u"example";  // 使用 u"" 创建 UTF-16 字符串
ani_string result = nullptr;

// 调用 UTF-16 版本的 String_New
ani_status status = env_->String_NewUTF16(reinterpret_cast<const uint16_t*>(example_utf16.c_str()),
                                          example_utf16.size(),
                                          &result);
```

### napi_get_value_string_utf16迁移示例
#### 代码示例对比

#### **N-API 示例1**
```cpp
napi_status GetUTF16String(napi_env env, napi_value string, uint16_t *utf16_buffer, size_t utf16_buffer_size, size_t *result) {
    return napi_get_value_string_utf16(env, string, utf16_buffer, utf16_buffer_size, result);
}

napi_value jsString;  // 假设这是从 JavaScript 传入的字符串
uint16_t buffer[100]; // UTF-16 缓冲区
size_t result;
napi_status status = GetUTF16String(env, jsString, buffer, 100, &result);

```
#### **ANI 示例1**
```cpp
ani_string string_; //ani string已创建
ani_size utf16_buffer_size = 256;
std::vector<uint16_t> utf16_buffer(utf16_buffer_size);
ani_size result = 0;

// 调用 String_GetUTF16
ani_status status = env_->String_GetUTF16(string_, utf16_buffer.data(), utf16_buffer_size, &result);
```

#### **N-API 示例2**
```cpp
napi_status GetUTF16Substring(napi_env env, napi_value string, size_t offset, size_t length, uint16_t* buffer, size_t buffer_size, size_t* result) {
    size_t str_len;
    napi_status status = napi_get_value_string_utf16(env, string, nullptr, 0, &str_len);
    if (status != napi_ok) return status;

    // 判断截取长度是否超过缓冲区大小
    if (length > buffer_size) return napi_generic_failure;

    // 获取完整的 UTF-16 字符串
    status = napi_get_value_string_utf16(env, string, buffer, buffer_size, &str_len);
    if (status != napi_ok) return status;

    // 计算子字符串偏移
    *result = length;
    return napi_ok;
}

napi_value string;  // 已获取的字符串
uint16_t buffer[10];
size_t result;
napi_status status = GetUTF16Substring(env, string, 0, 5, buffer, 10, &result);

```
#### **ANI 示例2**
```cpp
ani_string string_; //ani string已创建

ani_size substr_offset = 5;
ani_size substr_size = 10;

ani_size utf16_buffer_size = 256;
std::vector<uint16_t> utf16_buffer(utf16_buffer_size);
ani_size result = 0;

// 调用 String_GetUTF16SubString
ani_status status = env_->String_GetUTF16SubString(string_, substr_offset, substr_size,
                                                   utf16_buffer.data(), utf16_buffer_size, &result);
```
## 12. Array Operations


### napi_create_arraybuffer迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value CreateArrayBuffer(napi_env env, napi_callback_info info)
{
    int32_t value = 10;
    size_t length;
    length = size_t(value);
    void *data;
    napi_value result = nullptr;
    // 创建一个新的ArrayBuffer
    napi_create_arraybuffer(env, length, &data, &result);
    if (data != nullptr) {
        // 确保安全后才能使用data进行操作
    }
    // 返回ArrayBuffer
    return result;
}
```

#### **ANI 示例**
```cpp
int32_t value = 10;
size_t length;
length = size_t(value);
void *data;
ani_arraybuffer arraybuffer;
env_->CreateArrayBuffer(length, &data, &arraybuffer);
```

### napi_create_array_with_length迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_value CreateFixedArray(napi_env env, size_t length, napi_value* initial_array) {
    napi_value array;
    napi_create_array_with_length(env, length, &array); // 创建固定大小的 JS 数组

    // 如果有初始值，填充数组
    for (size_t i = 0; i < length && initial_array; i++) {
        napi_set_element(env, array, i, initial_array[i]);
    }

    return array;
}

napi_value myArray;
napi_value initial_values[3]; // 预初始化的值
CreateFixedArray(env, 3, initial_values);

```
#### **ANI 示例**
```cpp
constexpr ani_size TEST_ARRAY_SIZE = 5U;

ani_array_boolean booleanArray;
ASSERT_EQ(env_->Array_New_Boolean(TEST_ARRAY_SIZE, &booleanArray), ANI_OK);

ani_array_byte byteArray;
ASSERT_EQ(env_->Array_New_Byte(TEST_ARRAY_SIZE, &byteArray), ANI_OK);

ani_array_char charArray;
ASSERT_EQ(env_->Array_New_Char(TEST_ARRAY_SIZE, &charArray), ANI_OK);

ani_array_double doubleArray;
ASSERT_EQ(env_->Array_New_Double(TEST_ARRAY_SIZE, &doubleArray), ANI_OK);

ani_array_float floatArray;
ASSERT_EQ(env_->Array_New_Float(TEST_ARRAY_SIZE, &floatArray), ANI_OK);

ani_array_int intArray;
ASSERT_EQ(env_->Array_New_Int(TEST_ARRAY_SIZE, &intArray), ANI_OK);

ani_array_long longArray;
ASSERT_EQ(env_->Array_New_Long(TEST_ARRAY_SIZE, &longArray), ANI_OK);

ani_array_short shortArray;
ASSERT_EQ(env_->Array_New_Short(TEST_ARRAY_SIZE, &shortArray), ANI_OK);
```

### napi_create_array迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
static napi_value CreateArray(napi_env env, napi_callback_info info)
{
    // 创建一个空数组
    napi_value jsArray = nullptr;
    napi_create_array(env, &jsArray);
    // 将创建好的数组进行赋值
    for (int i = 0; i < 5; i++) {
        napi_value element;
        napi_create_int32(env, i, &element);
        napi_set_element(env, jsArray, i, element);
    }
    // 返回已创建好的数组
    return jsArray;
}
```

#### **ANI 示例**
```cpp
// Test creating array with initial element
ani_string str = nullptr;
const char *utf8String = "test";
const ani_size stringLength = strlen(utf8String);
env_->String_NewUTF8(utf8String, stringLength, &str);
ani_array_ref array2 = nullptr;
env_->Array_New_Ref(cls, stringLength, str, &array2);

// Verify initial element was set for all elements
for (ani_size i = 0; i < stringLength; i++) {
    ani_ref element = nullptr;
    env_->Array_Get_Ref(array2, i, &element));
    ani_size resultSize = 0;
    const ani_size utfBufferSize = 10;
    char utfBuffer[utfBufferSize] = {0};
    env_->String_GetUTF8SubString(reinterpret_cast<ani_string>(element), 0, stringLength, utfBuffer,
                                            sizeof(utfBuffer), &resultSize);
}
```


### napi_set_element迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_set_element
用于在ArkTS数组中设置指定索引位置的元素。

对于以索引值为键的object，可以使用napi_set_element来设置属性值。

```cpp
// cpp
static napi_value NapiSetElement(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数
    size_t argc = 3;
    napi_value args[3] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 检查第一个参数是否为数组
    bool isArr = false;
    napi_is_array(env, args[0], &isArr);
    if (!isArr) {
        napi_throw_type_error(env, nullptr, "Argument should be an object of type array");
        return nullptr;
    }
    // 获取要设置的元素索引
    double index = 0;
    napi_get_value_double(env, args[1], &index);
    // 将传入的值设置到数组指定索引位置
    napi_set_element(env, args[0], static_cast<uint32_t>(index), args[2]);

    return nullptr;
}
```

```ts
// index.d.ts
export const napiSetElement: <T>(arr: Array<T>, index: number, value: T) => void;

// ets
import hilog from '@ohos.hilog'
import testNapi from 'libentry.so'
let arr = [10, 20, 30];
testNapi.napiSetElement<number | string>(arr, 1, 'newElement');
testNapi.napiSetElement<number | string>(arr, 2, 50);
hilog.info(0x0000, 'testTag', 'Test Node-API napi_set_element arr: %{public}s', arr.toString());
hilog.info(0x0000, 'testTag', 'Test Node-API napi_set_element arr[3]: %{public}s', arr[3]);
interface MyObject {
  first: number;
  second: number;
}
let obj: MyObject = {
  first: 1,
  second: 2
};
testNapi.napiSetElement<number | string | Object>(arr, 4, obj);
let objAsString = JSON.stringify(arr[4]);
hilog.info(0x0000, 'testTag', 'Test Node-API napi_set_element arr[4]: %{public}s', objAsString);
```

#### **ANI 示例**

```cpp
auto array = static_cast<ani_array_ref>(CallEtsFunction<ani_ref>("GetArray"));

auto newValue1 = static_cast<ani_ref>(CallEtsFunction<ani_ref>("GetNewString1"));
const ani_size index1 = 0;
ASSERT_EQ(env_->Array_Set_Ref(array, index1, newValue1), ANI_OK);

auto newValue2 = static_cast<ani_ref>(CallEtsFunction<ani_ref>("GetNewString2"));
const ani_size index2 = 2;
ASSERT_EQ(env_->Array_Set_Ref(array, index2, newValue2), ANI_OK);

ani_boolean result = static_cast<ani_boolean>(CallEtsFunction<ani_boolean>("CheckArray", array));
ASSERT_EQ(result, ANI_TRUE);
```

```ts
// ets
function GetArray(): (String | null)[] {
    let a = [null, null , new String("Hello World!")];
    return a;
}

function GetNewString1(): String {
    return new String("New String 1!");
}

function GetNewString2(): String {
    return new String("New String 2!");
}

function CheckArray(array: (String | null)[]): boolean {
    return array[0] == "New String 1!" && array[1] == null && array[2] == "New String 2!";
}

function GetNumber(): Number {
    return new Number(42);
}
```

### napi_get_array_length迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_get_array_length 用于在Node-API模块中获取ArkTS数组对象的长度。

```cpp
#include "napi/native_api.h"

static napi_value GetArrayLength(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_value result;
    uint32_t length;
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 检查参数是否为数组
    bool is_array;
    napi_is_array(env, args[0], &is_array);
    if (!is_array) {
        napi_throw_type_error(env, nullptr, "Argument must be an array");
        return nullptr;
    }
    napi_get_array_length(env, args[0], &length);
    // 创建返回值
    napi_create_uint32(env, length, &result);
    return result;
}
```

#### **ANI 示例**

```CPP
ani_array_byte array;
const ani_size arraySize = 5U;
env_->Array_New_Byte(arraySize, &array);
ani_size length = 0;
env_->Array_GetLength(array, &length);
```


### napi_get_element迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_get_element
用于从ArkTS数组中获取请求索引位置的元素值。请求索引值应在数组的有效范围内，如果索引超出数组长度，函数会返回undefined。

```cpp
// cpp
static napi_value NapiGetElement(napi_env env, napi_callback_info info)
{
    // 获取ArkTS侧传入的参数
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 获取请求元素的索引值
    uint32_t index;
    napi_get_value_uint32(env, args[1], &index);
    // 获取请求索引位置的元素值并存储在result中
    napi_value result;
    napi_get_element(env, args[0], index, &result);

    return result;
}
```

```ts
// index.d.ts
export const napiGetElement: <T>(arr: Array<T>, index: number) => number | string | Object | boolean | undefined;

// ets
import hilog from '@ohos.hilog'
import testNapi from 'libentry.so'

interface MyObject {
  first: number;
  second: number;
}
let obj: MyObject = {
  first: 1,
  second: 2
};
let arr = [10, 'hello', null, obj];
hilog.info(0x0000, 'testTag', 'Test Node-API napi_get_element arr[0]: %{public}d', testNapi.napiGetElement<number | string | null | Object>(arr, 0));
hilog.info(0x0000, 'testTag', 'Test Node-API napi_get_element arr[1]: %{public}s', testNapi.napiGetElement<number | string | null | Object>(arr, 1));
hilog.info(0x0000, 'testTag', 'Test Node-API napi_get_element arr[2]: %{public}s', testNapi.napiGetElement<number | string | null | Object>(arr, 2));
hilog.info(0x0000, 'testTag', 'Test Node-API napi_get_element arr[3]: %{public}s', testNapi.napiGetElement<number | string | null | Object>(arr, 3));
hilog.info(0x0000, 'testTag', 'Test Node-API napi_get_element arr[4]: %{public}s', JSON.stringify(testNapi.napiGetElement(arr, 4)));
hilog.info(0x0000, 'testTag', 'Test Node-API napi_get_element arr[null]: %{public}s', testNapi.napiGetElement<number | string | null | Object>(arr, 5));
```

#### **ANI 示例**

```cpp
// cpp
auto array = static_cast<ani_array_ref>(CallEtsFunction<ani_ref>("GetArray"));
const ani_size index1 = 1;
const ani_size index2 = 2;
ani_ref ref1 = nullptr;
ani_ref ref2 = nullptr;
ani_boolean isNull;
ASSERT_EQ(env_->Array_Get_Ref(array, index1, &ref1), ANI_OK);
ASSERT_EQ(env_->Array_Get_Ref(array, index2, &ref2), ANI_OK);
ASSERT_EQ(env_->Reference_IsNull(ref1, &isNull), ANI_OK);
ASSERT_EQ(isNull, ANI_TRUE);
ASSERT_EQ(env_->Reference_IsNull(ref2, &isNull), ANI_OK);
ASSERT_EQ(isNull, ANI_FALSE);
```

```ts
// ets
function GetArray(): (Object | null)[] {
    return [null, null , new String("Hello World!")];
}
```

### napi_get_arraybuffer_info迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_get_arraybuffer_info 获取给定的ArrayBuffer对象的相关信息，包括数据指针和数据长度。
```cpp
static napi_value GetArrayBufferInfo(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 检查参数是否为ArrayBuffer
    bool isArrayBuffer = false;
    napi_is_arraybuffer(env, args[0], &isArrayBuffer);
    if (!isArrayBuffer) {
        napi_throw_type_error(env, nullptr, "Argument must be an ArrayBuffer");
        return nullptr;
    }

    void *data = nullptr;
    size_t byteLength = 0;
    // 获取ArrayBuffer的底层数据缓冲区和长度
    napi_status status = napi_get_arraybuffer_info(env, args[0], &data, &byteLength);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Failed to get ArrayBuffer info");
        return nullptr;
    }
    // 创建结果对象
    napi_value result = nullptr;
    napi_create_object(env, &result);
    // 创建数据缓冲区的字节长度属性
    napi_value byteLengthValue = nullptr;
    napi_create_uint32(env, byteLength, &byteLengthValue);
    napi_set_named_property(env, result, "byteLength", byteLengthValue);
    napi_value bufferData;
    napi_create_arraybuffer(env, byteLength, &data, &bufferData);
    napi_set_named_property(env, result, "buffer", bufferData);
    return result;
}
```

#### **ANI 示例**

```cpp
int32_t value = 10;
size_t length;
length = size_t(value);
void *data;
ani_arraybuffer arraybuffer;
env_->CreateArrayBuffer(length, &data, &arraybuffer);

size_t getLength;
void *getData;
env_->ArrayBuffer_GetInfo(arraybuffer, &getData, &getLength);
```

### napi_detach_arraybuffer迁移示例
---
由于与napi的差异性，ANI中没有分离arraybuffer与数据内存区域关联性的直接功能。
ANI中可以直接访问arraybuffer的data的指针。
因此实际上napi_detach_arraybuffer的目标，提高内存访问能力，ANI对其已经进行了C++内存管理，无需执行napi_detach_arraybuffer实现内存管理委托到C++组件。

#### 代码示例对比

#### **N-API 示例**
```CPP
static napi_value DetachedArrayBuffer(napi_env env, napi_callback_info info)
{
    // 调用napi_detach_arraybuffer接口分离给定ArrayBuffer的底层数据
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    napi_value arrayBuffer = args[0];
    napi_detach_arraybuffer(env, arrayBuffer);
    // 将分离后的arraybuffer传出去
    return arrayBuffer;
}
```

#### **ANI 示例**
```CPP
void getArrayBufferData(ani_env env, napi_callback_info info)
{
    // ArrayBuffer_GetInfo() 未合入还没有测试用例，待补充【lizhonghan】
}
```


### napi_create_external迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_create_external_arraybuffer迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
typedef struct {
    uint8_t *data;
    size_t length;
} BufferData;

void FinalizeCallback(napi_env env, void *finalize_data, void *finalize_hint)
{
    // 获取终结时的数据
    BufferData *bufferData = static_cast<BufferData *>(finalize_data);

    // 执行清理操作，比如释放资源
    delete[] bufferData->data;
    delete bufferData;
}

napi_value CreateExternalArraybuffer(napi_env env, napi_callback_info info)
{
    // 创建一个有五个元素的C++数组
    uint8_t *dataArray = new uint8_t[5]{1, 2, 3, 4, 5};
    napi_value externalBuffer = nullptr;
    BufferData *bufferData = new BufferData{dataArray, 5};

    // 使用napi_create_external_arraybuffer创建一个外部Array Buffer对象，并指定终结回调函数
    napi_status status =
        napi_create_external_arraybuffer(env, dataArray, 5, FinalizeCallback, bufferData, &externalBuffer);
    if (status != napi_ok) {
        // 处理错误
        napi_throw_error(env, nullptr, "Node-API napi_create_external_arraybuffer fail");
        return nullptr;
    }
    napi_value outputArray;
    // 使用napi_create_typedarray创建一个Array对象，并将externalBuffer对象作为参数传入
    status = napi_create_typedarray(env, napi_int8_array, 5, externalBuffer, 0, &outputArray);
    if (status != napi_ok) {
        // 处理错误
        napi_throw_error(env, nullptr, "Node-API napi_create_typedarray fail");
        return nullptr;
    }
    return outputArray;
}
```

```ts
// index.d.ts
export const createExternalArraybuffer: () => ArrayBuffer;

// ets
testNapi.createExternalArraybuffer();
```

#### **ANI 示例**
```cpp
typedef struct {
    uint8_t *data;
    size_t length;
} BufferData;

void FinalizeCallback(void *finalize_data, [[maybe_unused]] void *finalize_hint)
{
    // 获取终结时的数据
    BufferData *bufferData = static_cast<BufferData *>(finalize_data);

    // 执行清理操作，比如释放资源
    delete[] bufferData->data;
    delete bufferData;
}

uint8_t *dataArray = new uint8_t[5]{1, 2, 3, 4, 5};
ani_arraybuffer externalBuffer = nullptr;
BufferData *bufferData = new BufferData{dataArray, 5};

ani_status status =
    env->CreateArrayBufferExternal(dataArray, 5, FinalizeCallback, bufferData, &externalBuffer);
```


## 13. Reflection Support


### napi_has_named_property迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value HasNamedProperty(napi_env env, napi_callback_info info)
{
    // 从ArkTS侧传入两个参数：第一个参数为要检验的对象，第二个参数为要检测是否存在对象的属性
    size_t argc = 2;
    napi_value args[2] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // 将参数传入napi_has_property方法中，若接口调用成功则将结果转化为napi_value类型抛出，否则抛出错误
    bool result;
    napi_status status = napi_has_named_property(env, args[0], "data", &result);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_has_named_property fail");
        return nullptr;
    }

    // 若传入属性存在传入对象中，则输出true将结果转化为napi_value类型抛出
    napi_value returnResult;
    napi_get_boolean(env, result, &returnResult);
    return returnResult;
}
```

```ts
// index.d.ts
export const hasNamedProperty: (obj: Object) => boolean;

// ets
class Obj {
    data: number = 0
    message: string = ""
}
let obj: Obj = { data: 0, message: "hello world"};
let flag = testNapi.hasNamedProperty(obj);
```

#### **ANI 示例**
```cpp
// ets
class Obj {
    constructor(data: number, message: string) {
        this.data = data;
        this.message = message;
    }
    data: number;
    message: string;
}

function newObject() {
    return new Obj(0, "hello world");
}

// cpp
auto objRef = CallEtsFunction<ani_ref>("newObject");
ani_object obj = static_cast<ani_object>(objRef);

ani_boolean model;
if (env_->Object_GetPropertyByName_Boolean(obj, "data", &model) == ANI_NOT_FOUND){
    // 没有这个property
} else {
    // 有这个property
}
```


## 14. Coroutine Support

## 15. Variable Operations

### napi_create_int64迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_int64 将Node-API模块中的int64_t类型转换为ArkTS环境中number类型。

```cpp
#include "napi/native_api.h"

static napi_value CreateInt64(napi_env env, napi_callback_info info)
{
    // int64是有符号的64位整数类型，可以表示范围从-2^63到2^63 - 1的整数，即 -9223372036854775808到9223372036854775807
    // 要表示的整数值
    int64_t value = 2147483648;
    // 创建ArkTS中的int64数字
    napi_value result = nullptr;
    napi_status status = napi_create_int64(env, value, &result);
    if (status != napi_ok) {
        // 处理错误
        napi_throw_error(env, nullptr, "Failed to create int64 value");
    }
    return result;
}
```

#### **ANI 示例**

```ts
// sts
class TestNumber {
    static long_value: long = 0;
}
```

```cpp
// cpp
ani_class cls;
ASSERT_EQ(env_->FindClass("LTestNumber;", &cls), ANI_OK);

ani_static_field fieldLong;
ASSERT_EQ(env_->Class_FindStaticField(cls, "long_value", &fieldLong), ANI_OK);
ani_long int64Value = 2L;
ASSERT_EQ(env_->Class_SetStaticField_Long(cls, fieldLong, int64Value), ANI_OK);
ani_long result = 0;
ASSERT_EQ(env_->Class_GetStaticField_Long(cls, fieldLong, &result), ANI_OK);
ASSERT_EQ(result, int64Value);
```

### napi_create_int32迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_create_int32用于创建一个ArkTS数字（int32类型）的值。
cpp部分代码
```C++
static napi_value CreateInt32(napi_env env, napi_callback_info info)
{
    // int32_t是有符号的32位整数类型，表示带有符号的整数，它的范围是从-2^31到2^31 - 1，也就是-2147483648到2147483647
    // 要表示的整数值
    int32_t value = -26;
    // 创建ArkTS中的int32数字
    napi_value result = nullptr;
    napi_status status = napi_create_int32(env, value, &result);
    if (status != napi_ok) {
        // 处理错误
        napi_throw_error(env, nullptr, "Failed to create int32 value");
    }
    return result;
}
```
#### **ANI 示例**
可以使用Variable_SetValue_Int替换
```C++
namespace anyns {
    export let intValue: int = 3;
    export let floatValue: float = 3.14;
}

function checkIntValue(value: int): boolean {
    return anyns.intValue == value;
}
```
cpp部分代码
```C++
TEST_F(VariableSetValueIntTest, set_int_value_normal)
{
    ani_namespace ns {};
    ASSERT_EQ(env_->FindNamespace("Lanyns;", &ns), ANI_OK);
    ASSERT_NE(ns, nullptr);

    ani_variable variable {};
    ASSERT_EQ(env_->Namespace_FindVariable(ns, "intValue", &variable), ANI_OK);
    ASSERT_NE(variable, nullptr);
    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkIntValue", 3U), ANI_TRUE);

    ani_int value = 6U;
    ASSERT_EQ(env_->Variable_SetValue_Int(variable, value), ANI_OK);
    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkIntValue", value), ANI_TRUE);
}
```

### napi_get_value_uint32迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
static napi_value GetValueUint32(napi_env env, napi_callback_info info)
{
    // 获取传入的数字类型参数
    size_t argc = 1;
    napi_value argv[1] = {nullptr};
    // 解析传入的参数
    napi_get_cb_info(env, info, &argc, argv, nullptr, nullptr);

    uint32_t number = 0;
    // 获取传入参数的值中的无符号32位整数
    napi_status status = napi_get_value_uint32(env, argv[0], &number);
    // 如果传递的参数不是数字,将会返回napi_number_expected，设置函数返回nullptr
    if (status == napi_number_expected) {
        return nullptr;
    }
    OH_LOG_INFO(LOG_APP, "number=%{public}d", number);
    return nullptr;
}
```

```ts
// index.d.ts
export const getValueUint32: (a: number) => void;

// ets
testNapi.getValueUint32(95);
```

#### **ANI 示例**
```cpp
// sts
function Getuint32() {
    let num : number = 95;
    return num;
}

// cpp
ani_double numDouble = CallEtsFunction<ani_double>("Getuint32");
uint32_t num = static_cast<uint32_t>(numDouble);
```



### napi_create_double迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_double 将Node-API模块中的double类型转换为ArkTS环境中number类型。

```cpp
#include "napi/native_api.h"

static napi_value CreateDouble(napi_env env, napi_callback_info info)
{
    double value = 1.234;
    // 创建ArkTS中的double数字
    napi_value result = nullptr;
    napi_status status = napi_create_double(env, value, &result);
    if (status != napi_ok) {
        // 处理错误
        napi_throw_error(env, nullptr, "Failed to create double value");
    }
    return result;
}
```

#### **ANI 示例**

```ts
// sts
class TestNumber {
    static double_value: double = 0;
}
```

```cpp
// cpp
ani_class cls;
ASSERT_EQ(env_->FindClass("LTestNumber;", &cls), ANI_OK);

ani_static_field fieldDouble;
ASSERT_EQ(env_->Class_FindStaticField(cls, "double_value", &fieldDouble), ANI_OK);
ani_double doubleValue = 2.5;
ASSERT_EQ(env_->Class_SetStaticField_Double(cls, fieldDouble, doubleValue), ANI_OK);
ani_double result = 0;
ASSERT_EQ(env_->Class_GetStaticField_Double(cls, fieldDouble, &result), ANI_OK);
ASSERT_EQ(result, doubleValue);
```

### napi_get_reference_value迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
// cpp
napi_ref g_ref;

static napi_value UseReference(napi_env env, napi_callback_info info)
{
    napi_value obj = nullptr;
    // 通过调用napi_get_reference_value获取引用的ArkTS对象
    napi_status status = napi_get_reference_value(env, g_ref, &obj);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "napi_get_reference_value fail");
        return nullptr;
    }
    // 将获取到的对象返回
    return obj;
}
```

#### **ANI 示例**
```cpp
ani_ref nullRef;
env_->GetNull(&nullRef);
ani_wref wref;
env_->WeakReference_Create(nullRef, &wref));

ani_ref ref;
ani_boolean wasReleased;
env_->WeakReference_GetReference(wref, &wasReleased, &ref);
```


### napi_get_value_double迁移示例
---
#### 代码示例对比

#### **N-API 示例**
根据c对象的double值创建JavaScript的double对象
```C++
napi_value argument;
double number = 0.1;
napi_create_double(env, number, &argument);
ASSERT_CHECK_VALUE_TYPE(env, argument, napi_number);

double numberValue;
napi_get_value_double(env, argument, &numberValue);
```

#### **ANI 示例**
使用`Variable_GetValue_Double`替代，获取ani_variable对象的c double值。
```C++
// ets file
namespace anyns {
    let x: double = 3;
    let s: String = "abc";
    let z: boolean = false;
}

// cpp file
ani_namespace ns {};
ASSERT_EQ(env_->FindNamespace("Lanyns;", &ns), ANI_OK);
ASSERT_NE(ns, nullptr);

ani_variable variable {};
ASSERT_EQ(env_->Namespace_FindVariable(ns, "x", &variable), ANI_OK);
ASSERT_NE(variable, nullptr);

ani_double x;
ASSERT_EQ(env_->Variable_GetValue_Double(variable, &x), ANI_OK);
ASSERT_EQ(x, 3.0F);
```


### napi_get_value_bool迁移示例
---
#### 代码示例对比

#### **N-API 示例**
获取给定js Boolean对应的C bool值。
```c++
napi_value boolTrue = nullptr;
bool ret = false;

res = napi_get_boolean(env, true, &boolTrue);
ASSERT_EQ(res, napi_ok);

res = napi_get_value_bool(env, boolTrue, &ret);
ASSERT_EQ(res, napi_ok);
ASSERT_EQ(ret, true);
```
#### **ANI 示例**
使用`Variable_GetValue_Boolean`替代，获取ani_variable对象的c boolean值。
```C++
// ets file
namespace anyns {
    let x: double = 3.0;
    let s: String = "abc";
    let z: boolean = true;
}

// cpp file
ani_namespace ns {};
ASSERT_EQ(env_->FindNamespace("Lanyns;", &ns), ANI_OK);
ASSERT_NE(ns, nullptr);

ani_variable variable {};
ASSERT_EQ(env_->Namespace_FindVariable(ns, "z", &variable), ANI_OK);
ASSERT_NE(variable, nullptr);

ani_boolean res {};
ASSERT_EQ(env_->Variable_GetValue_Boolean(variable, &res), ANI_OK);
ASSERT_EQ(res, ANI_TRUE);
```


### napi_get_boolean迁移示例
---
#### 代码示例对比

#### **N-API 示例**
根据c对象的`Boolean`值创建JavaScript的`boolean`对象
```C++
napi_value boolTrue = nullptr;
bool ret = false;
res = napi_get_boolean(env, true, &boolTrue);
ASSERT_EQ(res, napi_ok);
```

#### **ANI 示例**
对于已确定类型的ani_value对象，可直接进行强转。因此不需要对应接口。


### napi_get_value_int32迁移示例
---
#### 代码示例对比

#### **N-API 示例**
获取给定JS number对应的C int32值。
```c++

auto numone = static_cast<int32_t>(10);

napi_value int32result;
napi_status status = napi_create_int32(env, numone, &int32result);
EXPECT_EQ(status, napi_status::napi_ok);

int32_t newint32res;
status = napi_get_value_int32(env, int32result, &newint32res);
EXPECT_EQ(status, napi_status::napi_ok);
```

#### **ANI 示例**
使用`Variable_GetValue_Int`替代，获取ani_variable对象的c in32值。
```C++
// ets file
namespace anyns {
    let x: int = 3;
    let s: String = "abc";
    let z: boolean = false;
}

// cpp file
ani_namespace ns {};
ASSERT_EQ(env_->FindNamespace("Lanyns;", &ns), ANI_OK);
ASSERT_NE(ns, nullptr);

ani_variable variable {};
ASSERT_EQ(env_->Namespace_FindVariable(ns, "x", &variable), ANI_OK);
ASSERT_NE(variable, nullptr);

ani_int x;
ASSERT_EQ(env_->Variable_GetValue_Int(variable, &x), ANI_OK);
ASSERT_EQ(x, 3U);
```


### napi_get_value_int64迁移示例
---
#### 代码示例对比

#### **N-API 示例**
获取给定JS number对应的C int64值。
```C++
int64_t testValue = 9007199254740991;
napi_value result = nullptr;
ASSERT_CHECK_CALL(napi_create_int64(env, testValue, &result));
ASSERT_CHECK_VALUE_TYPE(env, result, napi_number);

int64_t resultValue = 0;
ASSERT_CHECK_CALL(napi_get_value_int64(env, result, &resultValue));
ASSERT_EQ(resultValue, testValue);
```

#### **ANI 示例**
使用`Variable_GetValue_Long`替代，获取ani_variable对象的c in64值。
```C++
// ets file
namespace anyns {
    let x: long = 3;
    let s: String = "abc";
    let z: boolean = false;
}

// cpp file
ani_namespace ns {};
ASSERT_EQ(env_->FindNamespace("Lanyns;", &ns), ANI_OK);
ASSERT_NE(ns, nullptr);

ani_variable variable {};
ASSERT_EQ(env_->Namespace_FindVariable(ns, "x", &variable), ANI_OK);
ASSERT_NE(variable, nullptr);

ani_long x;
ASSERT_EQ(env_->Variable_GetValue_Long(variable, &x), ANI_OK);
ASSERT_EQ(x, 3L);
```


## 16. Module Support

### napi_load_module迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_load_module 通常用于在原生模块中动态加载 JavaScript 模块。例如在原生代码中根据某些条件加载不同的 JavaScript 模块。
```C++
static napi_value loadModule(napi_env env, napi_callback_info info) {
    // 1. 使用napi_load_module加载模块@ohos.hilog
    napi_value result;
    napi_status status = napi_load_module(env, "@ohos.hilog", &result);
    // 2. 使用napi_get_named_property获取info函数
    napi_value infoFn;
    napi_get_named_property(env, result, "info", &infoFn);
    napi_value tag;
    std::string formatStr = "test";
    napi_create_string_utf8(env, formatStr.c_str(), formatStr.size(), &tag);
    napi_value outputString;
    std::string str = "Hello HarmonyOS";
    napi_create_string_utf8(env, str.c_str(), str.size(), &outputString);
    napi_value flag;
    napi_create_int32(env, 0, &flag);
    napi_value args[3] = {flag, tag, outputString};
    // 3. 使用napi_call_function调用info函数
    napi_call_function(env, result, infoFn, 3, args, nullptr);
    return result;
}
```
#### **ANI 示例**
在 ArkTS中，不支持动态加载模块。这是因为 ArkTS 的设计目标和运行环境限制了动态模块加载的能力。
1.静态类型检查：
ArkTS 是基于 TypeScript 的，而 TypeScript 强调静态类型检查和编译时优化。动态加载模块会破坏这种静态特性，导致类型检查和编译优化变得困难。
2.性能优化：
ArkTS 运行在资源受限的设备（如 IoT 设备或嵌入式设备）上，动态加载模块会增加运行时开销，影响性能。
3.安全性：
动态加载模块可能会引入安全风险，尤其是在嵌入式设备或 IoT 场景中。ArkTS 的设计目标之一是确保代码的安全性和可靠性。
4.编译时优化：
ArkTS 的代码在编译时会被优化和打包，动态加载模块会破坏这种优化机制。所以不存在这种接口。



### node_api_get_module_file_name迁移示例
---
#### 代码示例对比

#### **N-API 示例**
node_api_get_module_file_name 用于获取当前模块的文件名。
```C++
static napi_value GetModuleFileName(napi_env env, napi_callback_info info)
{
    // 声明一个const char类型的指针变量file，用于存储模块绝对路径
    const char *file = nullptr;
    napi_value value = nullptr;
    // 获取当前模块的绝对路径，并将结果存储在file变量中
    napi_status status = node_api_get_module_file_name(env, &file);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Failed to get module file name");
        return nullptr;
    }
    // 创建一个包含绝对路径的napi_value类型的字符串
    napi_create_string_utf8(env, file, NAPI_AUTO_LENGTH, &value);
    return value;
}
```
#### **ANI 示例**
未使用不关注


### napi_load_module_with_info迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_load_module_with_info 通常用于在原生模块中动态加载 JavaScript 模块。例如，你可能需要在原生代码中加载一个配置文件或插件模块，并获取其导出的内容。
```C++
static napi_value loadModule(napi_env env, napi_callback_info info) {
    napi_value result;
    // 1. 使用napi_load_module_with_info加载Test文件中的模块
    napi_status status = napi_load_module_with_info(env, "entry/src/main/ets/Test", "com.example.application/entry", &result);
    if (status != napi_ok) {
       return nullptr;
    }
    napi_value testFn;
    // 2. 使用napi_get_named_property获取test函数
    napi_get_named_property(env, result, "test", &testFn);
    // 3. 使用napi_call_function调用函数test
    napi_call_function(env, result, testFn, 0, nullptr, nullptr);
    napi_value value;
    napi_value key;
    std::string keyStr = "value";
    napi_create_string_utf8(env, keyStr.c_str(), keyStr.size(), &key);
    // 4. 使用napi_get_property获取变量value
    napi_get_property(env, result, key, &value);
    return result;
}
```
#### **ANI 示例**
在 ArkTS中，不支持直接动态加载模块（例如通过 import() 动态导入）。这是因为 ArkTS 的设计目标是静态化模块依赖关系，以提高性能和安全性。
所以不存在这种接口。




## 17. NameSpace Support

## 18. Enum Operations

## 19. CLS Support


### napi_add_finalizer迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



## 20. Tulpe Operations

## 21. Async Operations


### napi_get_uv_event_loop迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




### napi_create_async_work迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_delete_async_work迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_queue_async_work迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_create_promise迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_create_promise用于创建一个Promise对象。
```C++
napi_value NapiPromiseDemo(napi_env env, napi_callback_info)
{
    napi_deferred deferred = nullptr;
    napi_value promise = nullptr;
    napi_status status = napi_ok;

    napi_throw_error(env, "500", "common error");

    status = napi_create_promise(env, &deferred, &promise); // 有异常返回napi_pending_exception，且deferred、promise都为nullptr
    if (status == napi_ok) {
        // do something
    }

    return nullptr;
}
```
#### **ANI 示例**
可以使用Promise_New代替
```C++
function checkReject(promise: Promise<String>, rejection: String): boolean {
  try {
    await promise;
    return false;
  } catch (exception) {
    return exception == rejection;
  }
}

cpp代码
TEST_F(PromiseRejectTest, ResolvePromise)
{
    ani_object promise;
    ani_resolver resolver;

    ASSERT_EQ(env_->Promise_New(&resolver, &promise), ANI_OK);

    std::string rejected = "rejected";
    ani_string rejection;
    ASSERT_EQ(env_->String_NewUTF8(rejected.c_str(), rejected.size(), &rejection), ANI_OK);

    ASSERT_EQ(env_->PromiseResolver_Reject(resolver, reinterpret_cast<ani_error>(rejection)), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkReject", promise, rejection), ANI_TRUE);
}
```




### napi_resolve_deferred迁移示例
---
#### 代码示例对比

#### **N-API 示例**
用于对Promise关联的deferred对象进行解析，napi_resolve_deferred将其从挂起状态转换为已兑现状态。
```C++
static napi_value CreatePromise(napi_env env, napi_callback_info info)
{
    // deferred是一个延迟对象，作用是将函数延迟一定时间再执行
    napi_deferred deferred = nullptr;
    napi_value promise = nullptr;
    // 调用接口创建Promise对象
    napi_status status = napi_create_promise(env, &deferred, &promise);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Create promise failed");
        return nullptr;
    }
    // 调用napi_is_promise判断napi_create_promise接口创建的是不是Promise对象
    bool isPromise = false;
    napi_value returnIsPromise = nullptr;
    napi_is_promise(env, promise, &isPromise);
    // 将布尔值转为可以返回的napi_value
    napi_get_boolean(env, isPromise, &returnIsPromise);
    return returnIsPromise;
}

static napi_value ResolveRejectDeferred(napi_env env, napi_callback_info info)
{
    // 获得并解析参数
    size_t argc = 3;
    napi_value args[3] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 第一个参数为向resolve传入的信息，第二个参数为向reject传入的信息，第三个参数为Promise的状态
    bool status;
    napi_get_value_bool(env, args[2], &status);
    // 创建Promise对象
    napi_deferred deferred = nullptr;
    napi_value promise = nullptr;
    napi_status createStatus = napi_create_promise(env, &deferred, &promise);
    if (createStatus != napi_ok) {
        napi_throw_error(env, nullptr, "Create promise failed");
        return nullptr;
    }
    // 根据第三个参数设置resolve或reject
    if (status) {
        napi_resolve_deferred(env, deferred, args[0]);
    } else {
        napi_reject_deferred(env, deferred, args[1]);
    }
    // 返回设置了resolve或reject的Promise对象
    return promise;
}
```
#### **ANI 示例**
可以使用PromiseResolver_Resolve替换
```C++
function checkResolve(promise: Promise<String>, resolution: String): boolean {
    let value = await promise;
    return value == resolution;
}
cpp代码
TEST_F(PromiseResolveTest, ResolvePromise)
{
    ani_object promise;
    ani_resolver resolver;

    ASSERT_EQ(env_->Promise_New(&resolver, &promise), ANI_OK);

    std::string resolved = "resolved";
    ani_string resolution;
    ASSERT_EQ(env_->String_NewUTF8(resolved.c_str(), resolved.size(), &resolution), ANI_OK);

    ASSERT_EQ(env_->PromiseResolver_Resolve(resolver, resolution), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkResolve", promise, resolution), ANI_TRUE);
}
```



### napi_reject_deferred迁移示例
---
#### 代码示例对比

#### **N-API 示例**
用于对Promise关联的deferred对象进行解析，napi_reject_deferred将其从挂起状态转换为已拒绝状态。
```C++
static napi_value CreatePromise(napi_env env, napi_callback_info info)
{
    // deferred是一个延迟对象，作用是将函数延迟一定时间再执行
    napi_deferred deferred = nullptr;
    napi_value promise = nullptr;
    // 调用接口创建Promise对象
    napi_status status = napi_create_promise(env, &deferred, &promise);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Create promise failed");
        return nullptr;
    }
    // 调用napi_is_promise判断napi_create_promise接口创建的是不是Promise对象
    bool isPromise = false;
    napi_value returnIsPromise = nullptr;
    napi_is_promise(env, promise, &isPromise);
    // 将布尔值转为可以返回的napi_value
    napi_get_boolean(env, isPromise, &returnIsPromise);
    return returnIsPromise;
}

static napi_value ResolveRejectDeferred(napi_env env, napi_callback_info info)
{
    // 获得并解析参数
    size_t argc = 3;
    napi_value args[3] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    // 第一个参数为向resolve传入的信息，第二个参数为向reject传入的信息，第三个参数为Promise的状态
    bool status;
    napi_get_value_bool(env, args[2], &status);
    // 创建Promise对象
    napi_deferred deferred = nullptr;
    napi_value promise = nullptr;
    napi_status createStatus = napi_create_promise(env, &deferred, &promise);
    if (createStatus != napi_ok) {
        napi_throw_error(env, nullptr, "Create promise failed");
        return nullptr;
    }
    // 根据第三个参数设置resolve或reject
    if (status) {
        napi_resolve_deferred(env, deferred, args[0]);
    } else {
        napi_reject_deferred(env, deferred, args[1]);
    }
    // 返回设置了resolve或reject的Promise对象
    return promise;
}
```
#### **ANI 示例**
可以使用PromiseResolver_Reject替换
```C++
function checkReject(promise: Promise<String>, rejection: String): boolean {
  try {
    await promise;
    return false;
  } catch (exception) {
    return exception == rejection;
  }
}
cpp代码
TEST_F(PromiseRejectTest, ResolvePromise)
{
    ani_object promise;
    ani_resolver resolver;

    ASSERT_EQ(env_->Promise_New(&resolver, &promise), ANI_OK);

    std::string rejected = "rejected";
    ani_string rejection;
    ASSERT_EQ(env_->String_NewUTF8(rejected.c_str(), rejected.size(), &rejection), ANI_OK);

    ASSERT_EQ(env_->PromiseResolver_Reject(resolver, reinterpret_cast<ani_error>(rejection)), ANI_OK);

    ASSERT_EQ(CallEtsFunction<ani_boolean>("checkReject", promise, rejection), ANI_TRUE);
}
```



### napi_is_promise迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_is_promise 检查一个napi_value是否代表一个Promise对象时，可以使用这个函数。

```cpp
#include "napi/native_api.h"

static napi_value IsPromise(napi_env env, napi_callback_info info)
{
    napi_value argv[1] = {nullptr};
    size_t argc = 1;
    napi_status status;
    // 获取传入的参数
    napi_get_cb_info(env, info, &argc, argv, nullptr, nullptr);
    bool isPromise = false;
    // 检查给定的入参是否为Promise对象，将结果保存在isPromise变量中
    status = napi_is_promise(env, argv[0], &isPromise);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Node-API napi_is_promise failed");
        return nullptr;
    }
    napi_value result = nullptr;
    // 将isPromise的值转换为napi_value中的类型返回
    napi_get_boolean(env, isPromise, &result);
    return result;
}
```

#### **ANI 示例**

```cpp
// 用法不同，无风险
```



### napi_cancel_async_work迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_queue_async_work_with_qos迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_make_callback迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




### napi_run_event_loop迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_stop_event_loop迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




### napi_set_promise_rejection_callback迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_send_event迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


### napi_send_cancelable_event迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


### napi_cancel_event迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


### napi_is_callable迁移示例
---

如果不确定对象的类型，可以使用Object_InstanceOf进行类型判断。
但是开发者应当知晓对象的类型范围，用友好的有限的搜索完成类型判断。

#### 代码示例对比

#### **N-API 示例**
```CPP
static napi_value CheckIfCallable(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args[1];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    bool isCallable = false;
    napi_is_callable迁移示例(env, args[0], &isCallable);

    napi_value result;
    napi_get_boolean(env, isCallable, &result);
    return result;
}
```

#### **ANI 示例**
```CPP
ani_boolean CheckIfCallable(ani_env *env,[[maybe_unused]] ani_object obj, ani_object value)
{
    ani_class cls;
    env->FindClass("Lstd/core/Function;", &cls); // Lescompat/Function;是FunctionN的基类

    ani_type typeFunction = cls;
    ani_boolean result;
    env->Object_InstanceOf(value, typeFunction, &result);
    return result;
}
```


### napi_wrap_async_finalizer迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


## 22. Scope Support



### napi_open_handle_scope迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_handle_scope scope = nullptr;
napi_open_handle_scope(env, &scope);
```
#### **ANI 示例**
```cpp
ani_size nr_refs = 16;
CreateLocalScope(env, nr_refs);
```

### napi_close_handle_scope迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_handle_scope scope = nullptr;
napi_open_handle_scope(env, &scope);
napi_close_handle_scope(env, scope);
```
#### **ANI 示例**
```cpp
ani_size nr_refs = 16;
CreateLocalScope(env, nr_refs);
DestroyLocalScope(env);
```

### napi_open_callback_scope迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**


### napi_close_callback_scope迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_open_escapable_handle_scope迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_handle_scope scope = nullptr;
napi_open_escapable_handle_scope(env, &scope);
```
#### **ANI 示例**
```cpp
ani_size nr_refs = 60;
CreateEscapeLocalScope(env, nr_refs);
```


### napi_close_escapable_handle_scope迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```cpp
napi_handle_scope scope = nullptr;
napi_open_escapable_handle_scope(env, &scope);
napi_close_escapable_handle_scope(env, scope);
```
#### **ANI 示例**
```cpp
ani_size nr_refs = 60;
CreateEscapeLocalScope(env, nr_refs);
ani_string string = nullptr;
String_NewUTF8(env, "test", 4, &string);
ani_ref result;
DestroyEscapeLocalScope(env, string, &result);
ASSERT_NE(result, nullptr);
```


### napi_escape_handle迁移示例
---
#### 代码示例对比

#### **N-API 示例**
`napi_escape_handle`用于提升传入的ArkTS对象的生命周期到其父作用域。
```C++
napi_value EscapableHandleScopeTest() {
    napi_escapable_handle_scope scope;
    napi_open_escapable_handle_scope(env, &scope);

    napi_value obj = nullptr;
    napi_create_object(env, &obj);
    napi_value value = nullptr;
    napi_create_string_utf8(env, "Test napi_escapable_handle_scope", NAPI_AUTO_LENGTH, &value);
    napi_set_named_property(env, obj, "key", value);

    napi_value escapedObj = nullptr;
    napi_escape_handle(env, scope, obj, &escapedObj);

    napi_close_escapable_handle_scope(env, scope);

    napi_value result = nullptr;
    napi_get_named_property(env, escapedObj, "key", &result);
    return result;
}
```
#### **ANI 示例**
使用`CreateEscapeLocalScope`创建一个新的逃逸局部作用域，在使用`DestroyEscapeLocalScope`销毁当前的逃逸局部作用域，并允许检索逃逸引用。
```C++
ani_ref CreateRefObject(ani_env* env) {
    ani_ref objectRef;
    env->String_NewUTF8("x", 1, reinterpret_cast<ani_string *>(&objectRef));
    return objectRef;
}

ani_status EscapeObject(ani_env* env, ani_ref* escapedObj) {
    ani_size maxEscapeNum = 1;
    auto status = env->CreateEscapeLocalScope(maxEscapeNum);
    if (status != ANI_OK) {
        return status;
    }
    ani_ref obj = CreateRefObject(env);
    status = env->DestroyEscapeLocalScope(obj, escapedObj);
    return status;
}
```

### napi_open_fast_native_scope迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用且语义不一致，无示例。
```

### napi_close_fast_native_scope迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用且语义不一致，无示例。
```


## 23. Program Operations


### napi_run_script迁移示例
---
#### 代码示例对比

#### **N-API 示例**
当前接口实际为空实现，无示例。

#### **ANI 示例**
napi接口实际为空实现，无示例。



### napi_run_script_path迁移示例
---
#### 代码示例对比

#### **N-API 示例**
运行指定abc文件。
```C++
// example.sts  =>  example.abc
console.log('test log');
// C
napi_status status = napi_run_script_path(env, "./example.abc", &result);

if (status != napi_ok) {
    printf("Fail to run abc file ");
} else {
    printf("Run abc file");
}
```

#### **ANI 示例**
通过找到待执行的abc文件的入口函数，并执行对应的函数获取结果。可通过CallEtsFunction来运行对应函数。
```C++
ani_status GetFunctionResule(ani_env* env, ani_ref res) {
    ASSERT_EQ(CallEtsFunction<ani_boolean>("main"), ANI_TRUE);
}
```


## 24. ThreadSafe Support



### napi_create_threadsafe_function迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_get_threadsafe_function_context迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_call_threadsafe_function迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_acquire_threadsafe_function迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




### napi_release_threadsafe_function迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_unref_threadsafe_function迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_ref_threadsafe_function迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




### napi_call_threadsafe_function_with_priority迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




## 25. Buffer Operations

### napi_is_buffer迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_is_buffer 判断给定ArkTS value是否为Buffer对象。

```cpp
#include <string>
#include "napi/native_api.h"

static napi_value IsBuffer(napi_env env, napi_callback_info info)
{
    // 创建一个Buffer对象
    std::string str = "buffer";
    napi_value buffer = nullptr;
    napi_create_buffer(env, strlen(str.data()), (void **)(str.data()), &buffer);

    // 调用napi_is_buffer接口判断创建的对象是否为buffer
    bool result = false;
    napi_is_buffer(env, buffer, &result);
    // 将结果返回出去
    napi_value returnValue = nullptr;
    napi_get_boolean(env, result, &returnValue);
    return returnValue;
}
```

#### **ANI 示例**

```cpp
// 用法不同，无风险
```


### napi_create_buffer迁移示例
---
ANI中根据业务需求用arraybuffer类型或者ani_byte类型替代buffer。
#### 代码示例对比

#### **N-API 示例**
```cpp
static void createBuffer(napi_env env, napi_callback_info info)
{
    // 创建一个Buffer对象
    std::string str = "buffer";
    napi_value buffer = nullptr;
    napi_create_buffer(env, strlen(str.data()), (void **)(str.data()), &buffer);
}
```

#### **ANI 示例**
arraybuffer
```cpp
ani_arraybuffer arrayBuffer;
void *data = nullptr;
auto status = env_->CreateArrayBuffer(0, &data, &arrayBuffer);
```

### napi_create_external_buffer迁移示例
---

ANI中不存在单独的buffer，请用arraybuffer或者byte替代。
参考napi_create_external_arraybuffer迁移示例。


### napi_create_buffer_copy迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_buffer_copy 用于创建并获取一个指定大小的ArkTS Buffer，并以给定的入参数据对buffer的缓冲区进行初始化。

```cpp
#include <string>
#include "napi/native_api.h"

static napi_value CreateBuffer(napi_env env, napi_callback_info info)
{
    std::string str("CreateBuffer");
    void *bufferPtr = nullptr;
    size_t bufferSize = str.size();
    napi_value buffer = nullptr;
    // 调用napi_create_buffer接口创建并获取一个指定大小的ArkTS Buffer
    napi_create_buffer(env, bufferSize, &bufferPtr, &buffer);
    // 将字符串str的值复制到buffer的内存中
    strcpy((char *)bufferPtr, str.data());
    return buffer;
}
```

#### **ANI 示例**

```cpp
// 用法不同，无风险
```


### napi_get_buffer_info迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_get_buffer_info用于从 JavaScript 传递的 Buffer 对象中提取底层数据和长度。
```C++
static napi_value GetBufferInfo(napi_env env, napi_callback_info info)
{
    // 创建一个字符串
    std::string str("GetBufferInfo");
    napi_value buffer = nullptr;
    void *bufferPtr = nullptr;
    size_t bufferSize = str.size();
    napi_create_buffer(env, bufferSize, &bufferPtr, &buffer);
    strcpy((char *)bufferPtr, str.data());
    // 获取Buffer的信息
    void *tmpBufferPtr = nullptr;
    size_t bufferLength = 0;
    napi_get_buffer_info(env, buffer, &tmpBufferPtr, &bufferLength);
    // 创建一个新的ArkTS字符串来保存Buffer的内容并返出去
    napi_value returnValue = nullptr;
    napi_create_string_utf8(env, (char*)tmpBufferPtr, bufferLength, &returnValue);
    return returnValue;
}
```
#### **ANI 示例**
//ArrayBuffer_GetInfo待开发实现



## 26. TypeCast Support

## 27. Env Operations



### napi_add_env_cleanup_hook迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_add_env_cleanup_hook 允许注册一个钩子，该钩子将在 Node.js 环境被拆除时调用。
```C++
// 定义内存结构，包含指向数据的指针和数据的大小
typedef struct {
    char *data;
    size_t size;
} Memory;
// 外部缓冲区清理回调函数，用于释放分配的内存
void ExternalFinalize(napi_env env, void *finalize_data, void *finalize_hint)
{
    Memory *wrapper = (Memory *)finalize_hint;
    free(wrapper->data);
    free(wrapper);
    OH_LOG_INFO(LOG_APP, "Node-API napi_add_env_cleanup_hook ExternalFinalize");
}
// 在环境关闭时执行一些清理操作，如清理全局变量或其他需要在环境关闭时处理的资源
static void Cleanup(void *arg)
{
    // 执行清理操作
    OH_LOG_INFO(LOG_APP, "Node-API napi_add_env_cleanup_hook cleanuped: %{public}d", *(int *)(arg));
}
// 创建外部缓冲区并注册环境清理钩子函数
static napi_value NapiEnvCleanUpHook(napi_env env, napi_callback_info info)
{
    // 分配内存并复制字符串数据到内存中
    std::string str("Hello from Node-API!");
    Memory *wrapper = (Memory *)malloc(sizeof(Memory));
    wrapper->data = (char *)malloc(str.size());
    strcpy(wrapper->data, str.c_str());
    wrapper->size = str.size();
    // 创建外部缓冲区对象，并指定清理回调函数
    napi_value buffer = nullptr;
    napi_create_external_buffer(env, wrapper->size, (void *)wrapper->data, ExternalFinalize, wrapper, &buffer);
    // 静态变量作为钩子函数参数
    static int hookArg = 42;
    static int hookParameter = 1;
    // 注册环境清理钩子函数
    napi_status status = napi_add_env_cleanup_hook(env, Cleanup, &hookArg);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Test Node-API napi_add_env_cleanup_hook failed.");
        return nullptr;
    }
    // 注册环境清理钩子函数，此处不移除环境清理钩子，为了在Java环境被销毁时，这个钩子函数被调用，用来模拟执行一些清理操作，例如释放资源、关闭文件等。
    status = napi_add_env_cleanup_hook(env, Cleanup, &hookParameter);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Test Node-API napi_add_env_cleanup_hook failed.");
        return nullptr;
    }
    // 立即移除环境清理钩子函数，确保不会在后续环境清理时被调用
    // 通常，当为其添加此钩子的资源无论如何都被拆除时调用这个接口
    napi_remove_env_cleanup_hook(env, Cleanup, &hookArg);
    // 返回创建的外部缓冲区对象
    return buffer;
}
```
#### **ANI 示例**
ArkTS 通常不需要手动注册环境清理钩子函数，因为资源管理由框架自动处理。
如果使用了原生资源或全局资源，可以通过组件的生命周期回调（如 aboutToDisappear）来手动清理资源。
对于普通的 JavaScript 对象，依赖垃圾回收机制即可。所以不存在这种接口。



### napi_remove_env_cleanup_hook迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```C++
// 定义内存结构，包含指向数据的指针和数据的大小
typedef struct {
    char *data;
    size_t size;
} Memory;
// 外部缓冲区清理回调函数，用于释放分配的内存
void ExternalFinalize(napi_env env, void *finalize_data, void *finalize_hint)
{
    Memory *wrapper = (Memory *)finalize_hint;
    free(wrapper->data);
    free(wrapper);
    OH_LOG_INFO(LOG_APP, "Node-API napi_add_env_cleanup_hook ExternalFinalize");
}
// 在环境关闭时执行一些清理操作，如清理全局变量或其他需要在环境关闭时处理的资源
static void Cleanup(void *arg)
{
    // 执行清理操作
    OH_LOG_INFO(LOG_APP, "Node-API napi_add_env_cleanup_hook cleanuped: %{public}d", *(int *)(arg));
}
// 创建外部缓冲区并注册环境清理钩子函数
static napi_value NapiEnvCleanUpHook(napi_env env, napi_callback_info info)
{
    // 分配内存并复制字符串数据到内存中
    std::string str("Hello from Node-API!");
    Memory *wrapper = (Memory *)malloc(sizeof(Memory));
    wrapper->data = (char *)malloc(str.size());
    strcpy(wrapper->data, str.c_str());
    wrapper->size = str.size();
    // 创建外部缓冲区对象，并指定清理回调函数
    napi_value buffer = nullptr;
    napi_create_external_buffer(env, wrapper->size, (void *)wrapper->data, ExternalFinalize, wrapper, &buffer);
    // 静态变量作为钩子函数参数
    static int hookArg = 42;
    static int hookParameter = 1;
    // 注册环境清理钩子函数
    napi_status status = napi_add_env_cleanup_hook(env, Cleanup, &hookArg);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Test Node-API napi_add_env_cleanup_hook failed.");
        return nullptr;
    }
    // 注册环境清理钩子函数，此处不移除环境清理钩子，为了在Java环境被销毁时，这个钩子函数被调用，用来模拟执行一些清理操作，例如释放资源、关闭文件等。
    status = napi_add_env_cleanup_hook(env, Cleanup, &hookParameter);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "Test Node-API napi_add_env_cleanup_hook failed.");
        return nullptr;
    }
    // 立即移除环境清理钩子函数，确保不会在后续环境清理时被调用
    // 通常，当为其添加此钩子的资源无论如何都被拆除时调用这个接口
    napi_remove_env_cleanup_hook(env, Cleanup, &hookArg);
    // 返回创建的外部缓冲区对象
    return buffer;
}
```
#### **ANI 示例**
ArkTS 通常不需要手动注册环境清理钩子函数，因为资源管理由框架自动处理。
如果使用了原生资源或全局资源，可以通过组件的生命周期回调（如 aboutToDisappear）来手动清理资源。
对于普通的 JavaScript 对象，依赖垃圾回收机制即可。所以不存在这种接口。


### napi_add_async_cleanup_hook迁移示例
---
#### 代码示例对比
1.0 worker和taskpool中的env环境被清理，所有业务钩子的相关资源要释放。ani下taskpool也worker是我们管理的，不存在销毁环境但是业务不知道的情况。



### napi_remove_async_cleanup_hook迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_async_init迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_async_destroy迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



## 28. JS Feature

### napi_get_global迁移示例
---
#### 代码示例对比

#### **N-API 示例**
napi_get_global用于获取全局对象。
```C++
static napi_value GetGlobal(napi_env env, napi_callback_info info)
{
    napi_value global = nullptr;
    // 获取global对象
    napi_get_global(env, &global);
    return global;
}
```
#### **ANI 示例**
在 ArkTS中，没有直接的全局对象，当前 ANI 没有提供对应的接口。
替代方案：根本上是要解决 requireNapi 的名字空间和加载的问题，参考 napi_get_named_property 相关分析。
napi_has_named_property 在NAPI中是根据名字获取一个属性，这个属性可以是任何类型类型，在ANI中，我们应使用确定的类型。
在LoadSystemModuleByEngine的实现中，是在全局对象(Global中)获取 "constructor_xxx"这个属性，根据上下问题，这个constructor_xxx 是一个Function
结合 napi_get_global  中的描述所以在这里对应的ANI应该是 Class_FindMethod这个目前是支持的。
```C++
export class On {
    static{
        loadLibrary("uitest.z");
    }
    private nativeOn:String = '';
    public constructor(on:String) {
        if(this.nativeOn=='') {
            this.nativeOn = on;
        }
    }
    native static id(id: string, pattern?: MatchPattern):On;
}

ani:
static const char *className = "Luitest_ani/Driver;";
ani_class cls;
ani_ref nullref;
env->GetNull(&nullref);
if (ANI_OK != env->FindClass(className, &cls)) {
    std::cerr << "Not found '" << className << "'" << std::endl;
    return nullref;
}
ani_method ctor;
if(ANI_OK != env->Class_FindMethod(cls, "", "Lstd/core/String;:V", &ctor)){
    std::cerr << "Not found '" << "ctor" << "'" << std::endl;
    return nullref;
}
```


### napi_get_undefined迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```
napi_get_undefined 是 Node-API (N-API) 提供的一个 API，用于获取 JavaScript 的 undefined 值，并返回给 JavaScript 端。

napi_status napi_get_undefined(napi_env env, napi_value* result);
输入：
env: N-API 执行环境
result: 指向 napi_value 的指针，用于接收 undefined 值

返回值：
napi_ok（成功）
其他错误码（如果 env 或 result 无效）

示例：
#include <node_api.h>

napi_value GetUndefined(napi_env env, napi_callback_info info) {
    napi_value result;
    napi_status status = napi_get_undefined(env, &result);  // 获取 JavaScript 的 undefined
    if (status != napi_ok) return nullptr;  // 发生错误时返回 nullptr
    return result;  // 返回 undefined
}

napi_value Init(napi_env env, napi_value exports) {
    napi_property_descriptor desc = { "getUndefined", 0, GetUndefined, 0, 0, 0, napi_default, 0 };
    napi_define_properties(env, exports, 1, &desc);
    return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)

```

#### **ANI 示例**
```
ani对应的接口是GetUndefined，用来获取Undefined类型，并存储到ref里面。

sts侧：
function isUndefined(v: Object | null | undefined): boolean {
    return v === undefined;
}

cpp侧：
#include "ani_gtest.h"

namespace ark::ets::ani::testing {

class GetUndefinedTest : public AniTest {};

TEST_F(GetUndefinedTest, get_undefined)
{
    ani_ref ref;
    ASSERT_EQ(env_->GetUndefined(&ref), ANI_OK);

    auto isUndefined = CallEtsFunction<ani_boolean>("isUndefined", ref);
    ASSERT_EQ(isUndefined, ANI_TRUE);
}

TEST_F(GetUndefinedTest, invalid_argument)
{
    ASSERT_EQ(env_->GetUndefined(nullptr), ANI_INVALID_ARGS);
}

}  // namespace ark::ets::ani::testing
```


### napi_get_null迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```
napi_status napi_get_null(napi_env env, napi_value* result);
入参：
env: N-API 执行环境（必须的）
result: 指向 napi_value 的指针，用于接收 null 值

返回值：
napi_ok（成功），其他错误码（如果 env 或 result 无效）

示例：
#include <node_api.h>

napi_value GetNull(napi_env env, napi_callback_info info) {
    napi_value result;
    napi_status status = napi_get_null(env, &result);  // 获取 JavaScript 的 null
    if (status != napi_ok) return nullptr;  // 发生错误时返回 nullptr
    return result;  // 返回 null
}

napi_value Init(napi_env env, napi_value exports) {
    napi_property_descriptor desc = { "getNull", 0, GetNull, 0, 0, 0, napi_default, 0 };
    napi_define_properties(env, exports, 1, &desc);
    return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)

```

#### **ANI 示例**
```
ani侧对应的接口是GetNull，用来获取null类型，并存储到ref里面。

sts侧代码：
function isNull(v: Object | null | undefined): boolean {
    return v === null;
}

cpp侧：
#include "ani_gtest.h"

namespace ark::ets::ani::testing {

class GetNullTest : public AniTest {};

TEST_F(GetNullTest, get_null)
{
    ani_ref ref;
    ASSERT_EQ(env_->GetNull(&ref), ANI_OK);

    auto isNull = CallEtsFunction<ani_boolean>("isNull", ref);
    ASSERT_EQ(isNull, ANI_TRUE);
}

TEST_F(GetNullTest, invalid_argument)
{
    ASSERT_EQ(env_->GetNull(nullptr), ANI_INVALID_ARGS);
}

}  // namespace ark::ets::ani::testing

```

## 29. Ark Feature


### napi_create_ark_runtime迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_ark_runtime, 创建基础运行时环境

```cpp
// 1. 创建基础运行环境
napi_env env;
napi_status ret = napi_create_ark_runtime(&env);
if (ret != napi_ok) {
    return nullptr;
}
```

#### **ANI 示例**

```cpp
const char *stdlib = std::getenv("ARK_ETS_STDLIB_PATH");
ASSERT_NE(stdlib, nullptr);

const std::string optionPrefix = "--ext:";

// Create boot-panda-files options
std::string bootFileString = optionPrefix + "--boot-panda-files=" + stdlib;
const char *abcPath = std::getenv("ANI_GTEST_ABC_PATH");
if (abcPath != nullptr) {
    bootFileString += ":";
    bootFileString += abcPath;
}

ani_option bootFileOption = {bootFileString.data(), nullptr};

std::vector<ani_option> options;
options.push_back(bootFileOption);

ani_vm *vm_ = nullptr;
ani_options optionsPtr = {options.size(), options.data()};
ani_status result = ANI_CreateVM(&optionsPtr, ANI_VERSION_1, &vm_);
```

### napi_destroy_ark_runtime迁移示例
---

#### 代码示例对比

#### **N-API 示例**

napi_destroy_ark_runtime, 创建基础运行时环境

```cpp
// 4. 销毁ArkTS环境
ret = napi_destroy_ark_runtime(&env);
return nullptr;
```

#### **ANI 示例**

```cpp
ani_status result = vm_->DestroyVM();
```

### napi_serialize迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_deserialize迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_delete_serialization_data迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




## 30. VM Interface



### napi_create_limit_runtime迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```

### napi_create_runtime迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

```cpp
// 未使用不关注
```


### napi_get_stack_trace迁移示例
---

该NAPI的对应ANI处于设计规划中。
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**




## 31. Sendable Support

### napi_define_sendable_class迁移示例
---
#### 代码示例对比

#### **N-API 示例**
`Sendable`是一个机制，用于`JavaScript/TypeScript`与原生代码之间进行安全的交互。主要的主要功能有
1. 跨语言交互，使得`JavaScript/TypeScript`代码能够使用原生代码，原生代码能够使用`JavaScript/TypeScript`的功能。
2. 数据交互，能够使`JavaScript/TypeScript`代码与原生代码之间相互传递数据结构。
使用`napi_define_sendable_class`能够定义一个sendable类
```C
auto constructor = [](napi_env env, napi_callback_info info) -> napi_value {
    napi_value thisVar = nullptr;
    napi_get_cb_info(env, info, nullptr, nullptr, &thisVar, nullptr);
    return thisVar;
}

napi_value testClass = nullptr;
auto res = napi_define_sendable_class(env, "TestClass", NAPI_AUTO_LENGTH, constructor,
        nullptr, 0, nullptr, nullptr, &testClass);

if(res == napi_ok) {
    printf("Successfully create a sendable class");
} else {
    printf("Failed to create a sendable class");
}
```

#### **ANI 示例**
ArkTS 作为一种静态类型语言，天然的拥有sendable机制，允许与原生代码之间安全的进行数据交互。所以不存在这种接口。



### napi_is_sendable迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_is_sendable 判断给定ArkTS value是否是Sendable的。

```cpp
#include "napi/native_api.h"

static napi_value IsSendable(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args[1] = {nullptr};
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
    bool isSendable = false;
    napi_is_sendable(env, args[0], &isSendable);
    napi_value result;
    napi_get_boolean(env, isSendable, &result);
    return result;
}
```

#### **ANI 示例**

```cpp
// sendable 直接删除
```

### napi_create_sendable_object_with_properties迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_sendable_object_with_properties 使用给定的napi_property_descriptor创建一个sendable对象。

```cpp
#include "napi/native_api.h"

static napi_value GetSendableObject(napi_env env, napi_callback_info info) {
    napi_value val_true;
    napi_get_boolean(env, true, &val_true);
    napi_property_descriptor desc1[] = {
        {"x", nullptr, nullptr, nullptr, nullptr, val_true, napi_default_jsproperty, nullptr},
    };
    napi_value obj;
    napi_create_sendable_object_with_properties(env, 1, desc1, &obj);
    return obj;
}
```

#### **ANI 示例**

```cpp
// sendable 直接删除
```

### napi_wrap_sendable迁移示例
---
#### 代码示例对比

#### **N-API 示例**
`Sendable`是一个机制，用于`JavaScript/TypeScript`与原生代码之间进行安全的交互。主要的主要功能有
1. 跨语言交互，使得`JavaScript/TypeScript`代码能够使用原生代码，原生代码能够使用`JavaScript/TypeScript`的功能。
2. 数据交互，能够使`JavaScript/TypeScript`代码与原生代码之间相互传递数据结构。
使用`napi_wrap_sendable`能够将一个native实例包装到ArkTS对象中。
```C
constexpr int32_t TEST_INT = 32;

napi_value js_obj = nullptr;
auto status = napi_wrap_sendable(
    env, js_obj, (void*)TEST_INT, [](napi_env env, void* data, void* hint) {}, nullptr);

if(status == napi_ok) {
    printf("Success");
} else {
    printf("Failed");
}

```

#### **ANI 示例**
ArkTS 作为一种静态类型语言，天然的拥有sendable机制，允许与原生代码之间安全的进行数据交互。所以不存在这种接口。



### napi_wrap_sendable_with_size迁移示例
---
#### 代码示例对比

#### **N-API 示例**
`Sendable`是一个机制，用于`JavaScript/TypeScript`与原生代码之间进行安全的交互。主要的主要功能有
1. 跨语言交互，使得`JavaScript/TypeScript`代码能够使用原生代码，原生代码能够使用`JavaScript/TypeScript`的功能。
2. 数据交互，能够使`JavaScript/TypeScript`代码与原生代码之间相互传递数据结构。
使用`napi_wrap_sendable_with_size`能够将一个native实例包装到ArkTS对象中，并指定其大小。
```C
constexpr char TEST_STRING[5] = "test";

napi_value js_obj = nullptr;
napi_status status = napi_wrap_sendable_with_size(
    env, js_obj, (void*)TEST_STRING, [](napi_env env, void* data, void* hint) {}, nullptr, INT_ONE);

if(status == napi_ok) {
    printf("Success");
} else {
    printf("Failed");
}
```

#### **ANI 示例**
ArkTS 作为一种静态类型语言，天然的拥有sendable机制，允许与原生代码之间安全的进行数据交互。所以不存在这种接口。


### napi_unwrap_sendable迁移示例
---
#### 代码示例对比

#### **N-API 示例**
`Sendable`是一个机制，用于`JavaScript/TypeScript`与原生代码之间进行安全的交互。主要的主要功能有
1. 跨语言交互，使得`JavaScript/TypeScript`代码能够使用原生代码，原生代码能够使用`JavaScript/TypeScript`的功能。
2. 数据交互，能够使`JavaScript/TypeScript`代码与原生代码之间相互传递数据结构。
使用`napi_unwrap_sendable`能够从ArkTS对象中获取到已被包装的native实例。
```C
napi_value js_obj = nullptr;
void* result;
napi_status status = napi_unwrap_sendable(env, js_obj, &result);

if(status == napi_ok) {
    printf("Success");
} else {
    printf("Failed");
}
```

#### **ANI 示例**
ArkTS 作为一种静态类型语言，天然的拥有sendable机制，允许与原生代码之间安全的进行数据交互。所以不存在这种接口。


### napi_remove_wrap_sendable迁移示例
---
#### 代码示例对比

#### **N-API 示例**
`Sendable`是一个机制，用于`JavaScript/TypeScript`与原生代码之间进行安全的交互。主要的主要功能有
1. 跨语言交互，使得`JavaScript/TypeScript`代码能够使用原生代码，原生代码能够使用`JavaScript/TypeScript`的功能。
2. 数据交互，能够使`JavaScript/TypeScript`代码与原生代码之间相互传递数据结构。
使用`napi_remove_wrap_sendable`能够从ArkTS对象中获取到已被包装的native实例，并将该实例从ArkTS对象中移除。
```C
napi_value js_obj = nullptr;
void* result;
napi_status status = napi_remove_wrap_sendable(env, js_obj, &result);

if(status == napi_ok) {
    printf("Success");
} else {
    printf("Failed");
}
```

#### **ANI 示例**
ArkTS 作为一种静态类型语言，天然的拥有sendable机制，允许与原生代码之间安全的进行数据交互。所以不存在这种接口。


### napi_create_sendable_array迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_sendable_array 创建一个sendable数组。

```cpp
#include "napi/native_api.h"

static napi_value GetSendableArray(napi_env env, napi_callback_info info) {
    napi_value result = nullptr;
    napi_create_sendable_array(env, &result);
    return result;
}
```

#### **ANI 示例**

```cpp
// sendable 直接删除
```

### napi_create_sendable_array_with_length迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_sendable_array_with_length 创建一个指定长度的sendable数组。

```cpp
static napi_value GetSendableArrayWithLength(napi_env env, napi_callback_info info) {
    napi_value result = nullptr;
    napi_create_sendable_array_with_length(env, 1, &result);
    return result;
}
```

#### **ANI 示例**

```cpp
// sendable 直接删除
```

### napi_create_sendable_arraybuffer迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_sendable_arraybuffer 创建一个sendable ArrayBuffer。

```cpp
#include "napi/native_api.h"
#include "hilog/log.h"

static napi_value GetSendableArrayBuffer(napi_env env, napi_callback_info info) {
    static size_t LENGTH = 1024;
    void *data;
    napi_value result = nullptr;
    napi_create_sendable_arraybuffer(env, LENGTH, &data, &result);
    bool isArrayBuffer = false;
    napi_is_arraybuffer(env, result, &isArrayBuffer);
    OH_LOG_INFO(LOG_APP, "isArrayBuffer: %{public}d", isArrayBuffer);
    return result;
}
```

#### **ANI 示例**

```cpp
// sendable 直接删除
```

### napi_create_sendable_typedarray迁移示例
---
#### 代码示例对比

#### **N-API 示例**

napi_create_sendable_typedarray 创建一个sendable TypedArray。

```cpp
#include "napi/native_api.h"
#include "hilog/log.h"

static napi_value GetSendableTypedArray(napi_env env, napi_callback_info info) {
    static size_t LENGTH = 1024;
    static size_t OFFSET = 0;
    void *data;
    napi_value arraybuffer = nullptr;
    napi_create_sendable_arraybuffer(env, LENGTH, &data, &arraybuffer);

    napi_value result = nullptr;
    napi_create_sendable_typedarray(env, napi_uint8_array, LENGTH, arraybuffer, OFFSET, &result);
    bool isTypedArray = false;
    napi_is_typedarray(env, result, &isTypedArray);
    OH_LOG_INFO(LOG_APP, "isTypedArray: %{public}d", isTypedArray);
    return result;
}
```

#### **ANI 示例**

```cpp
// sendable 直接删除
```

## 32. Type in Infra

### napi_handle_scope迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**

### napi_property_descriptor迁移示例
---
在ANI中descriptor统一由string进行描述。
迁移方式参考napi_define_properties。

### napi_int32_array迁移示例
---
napi_int32_array生成出来的对象对应到ani_array_int。
#### 代码示例对比

#### **N-API 示例**
```CPP
napi_value arraybuffer = nullptr;
napi_create_typedarray(env, napi_int8_array, 0, arraybuffer, 0, nullptr);
```
#### **ANI 示例**
```CPP
ani_array_int array;
ASSERT_EQ(env_->Array_New_Int(5U, &array), ANI_OK);
```


### napi_init迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**



### napi_utils迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**
```
// 未使用不关注
```

### napi_adapter迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**
```
// 未使用不关注
```


### napi_valuetype迁移示例
---
#### 代码示例对比

#### **N-API 示例**

```
napi_valuetype定义了napi的类型。

typedef enum {
  // ES6 types (corresponds to typeof)
  napi_undefined,
  napi_null,
  napi_boolean,
  napi_number,
  napi_string,
  napi_symbol,
  napi_object,
  napi_function,
  napi_external,
  napi_bigint,
} napi_valuetype;
```


#### **ANI 示例**
```
对应到ANI，使用ani_kind定义了相应的类型。

typedef enum {
    ANI_KIND_BOOLEAN,
    ANI_KIND_CHAR,
    ANI_KIND_BYTE,
    ANI_KIND_SHORT,
    ANI_KIND_INT,
    ANI_KIND_LONG,
    ANI_KIND_FLOAT,
    ANI_KIND_DOUBLE,
    ANI_KIND_REF,
} ani_kind;
```


### napi_module迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```
napi_module 结构体在 N-API（Node.js 的 C API）中用于定义一个原生插件模块，使得 Node.js 可以识别并加载这个模块。它作为桥梁连接了用 C 或 C++ 编写的底层代码与 Node.js 的 JavaScript 运行时环境。通过 napi_module，开发者可以创建高性能的扩展或利用现有的 C/C++ 库的功能，并将其无缝地集成到 Node.js 应用程序中。

typedef struct napi_module {
  int nm_version;
  unsigned int nm_flags;
  const char* nm_filename;
  napi_addon_register_func nm_register_func;
  const char* nm_modname;
  void* nm_priv;
  void* reserved[4];
} napi_module;
```

#### **ANI 示例**
ANI加载模块主要使用env_->FindModule和ani_module来实现。

```
TEST_F(ModuleBindNativeFunctionsTest, bind_native_functions)
{
    ani_module module;
    ASSERT_EQ(env_->FindModule("L@abcModule/test;", &module), ANI_OK);
    ASSERT_NE(module, nullptr);

    const char *concatSignature = "Lstd/core/String;Lstd/core/String;:Lstd/core/String;";
    std::array functions = {
        ani_native_function {"sum", "II:I", reinterpret_cast<void *>(Sum)},
        ani_native_function {"concat", concatSignature, reinterpret_cast<void *>(Concat)},
    };
    ASSERT_EQ(env_->Module_BindNativeFunctions(module, functions.data(), functions.size()), ANI_OK);

    const char *className = "@abcModule/test/ETSGLOBAL";
    ASSERT_EQ(CallEtsClassStaticMethod<ani_boolean>(className, "checkSum"), ANI_TRUE);
    ASSERT_EQ(CallEtsClassStaticMethod<ani_boolean>(className, "checkConcat"), ANI_TRUE);
}
```


### napi_static迁移示例
---
#### 代码示例对比

#### **N-API 示例**
```
napi_property_attributes定义了属性的属性，定义了napi_static作为属性参数。

typedef enum {
  napi_default = 0,
  napi_writable = 1 << 0,
  napi_enumerable = 1 << 1,
  napi_configurable = 1 << 2,

  // Used with napi_define_class to distinguish static properties
  // from instance properties. Ignored by napi_define_properties.
  napi_static = 1 << 10,

#if NAPI_VERSION >= 8
  // Default for class methods.
  napi_default_method = napi_writable | napi_configurable,

  // Default for object properties, like in JS obj[prop].
  napi_default_jsproperty = napi_writable |
                            napi_enumerable |
                            napi_configurable,
#endif  // NAPI_VERSION >= 8
} napi_property_attributes;
```

#### **ANI 示例**

```
// ani是静态语言，未使用不关注
```


### napi_default迁移示例
---
#### 代码示例对比

#### **N-API 示例**

```
napi_property_attributes定义了属性的属性，定义了napi_default作为属性参数。

typedef enum {
  napi_default = 0,
  napi_writable = 1 << 0,
  napi_enumerable = 1 << 1,
  napi_configurable = 1 << 2,

  // Used with napi_define_class to distinguish static properties
  // from instance properties. Ignored by napi_define_properties.
  napi_static = 1 << 10,

#if NAPI_VERSION >= 8
  // Default for class methods.
  napi_default_method = napi_writable | napi_configurable,

  // Default for object properties, like in JS obj[prop].
  napi_default_jsproperty = napi_writable |
                            napi_enumerable |
                            napi_configurable,
#endif  // NAPI_VERSION >= 8
} napi_property_attributes;
```

#### **ANI 示例**
```
// 未使用不关注
```

### napi_handle_scope迁移示例
---
ANI中不需要napi_handle_scope对象，所有没有直接对应的类型。
见CreateLocalScope的用法。
#### 代码示例对比

#### **N-API 示例**
```CPP
napi_handle_scope scope = nullptr;
napi_open_handle_scope(env_, &scope);
// ...
napi_close_handle_scope(env_, scope);
```

#### **ANI 示例**
```CPP
// Passing SPECIFIED_CAPACITY as capacity should succeed and return ANI_OK
ASSERT_EQ(env_->CreateLocalScope(SPECIFIED_CAPACITY), ANI_OK);
ani_string string = nullptr;
// Create SPECIFIED_CAPACITY strings in the newly created local scope
for (ani_size i = 1; i <= SPECIFIED_CAPACITY; ++i) {
    // Construct a unique stringName for each iteration
    std::string stringName = "String_NewUTF8_" + std::to_string(i) + ";";

    // Attempt to create a new UTF8 string and check the result
    ASSERT_EQ(env_->String_NewUTF8(stringName.c_str(), stringName.size(), &string), ANI_OK);
    ASSERT_NE(string, nullptr);
}
// Destroy the local scope after string creation
ASSERT_EQ(env_->DestroyLocalScope(), ANI_OK);
```

### napi_ok迁移示例
---
ANI_OK与napi_ok在ANI中等效迁移。
#### 代码示例对比

#### **N-API 示例**
```CPP
napi_status napi_test(napi_env env, napi_env* result_env)
{
    CHECK_ENV(env);
    CHECK_ARG(env, result_env);
    return napi_ok;
}
```

#### **ANI 示例**
```CPP
ani_status ani_test(ani_env env, ani_int result){
    return ANI_OK;
}
```

### napi_status迁移示例
---

#### 代码示例对比

#### **N-API 示例**
```CPP
napi_status napi_test(napi_env env, napi_env* result_env)
{
    CHECK_ENV(env);
    CHECK_ARG(env, result_env);
    return napi_ok;
}
```

#### **ANI 示例**
```CPP
ani_status ani_test(ani_env env, ani_int result){
    return ANI_OK;
}
```


### napi_adapter迁移示例
---
#### 代码示例对比

#### **N-API 示例**

#### **ANI 示例**
```
// 未使用不关注
```



### napi_ref迁移示例
---
napi_ref对应到ani_ref。
#### 代码示例对比

#### **N-API 示例**
```CPP
napi_ref ref;
```
#### **ANI 示例**
```CPP
ani_ref ref;
```


### napi_value迁移示例
---
napi_value对应到ani_value。
#### 代码示例对比

#### **N-API 示例**
```CPP
napi_value val;
```
#### **ANI 示例**
```CPP
ani_value val;
```

### napi_number迁移示例
---
api_number对应到ani_double。
#### 代码示例对比

#### **N-API 示例**
```CPP
api_number val;
```
#### **ANI 示例**
```CPP
ani_double val;
```

### napi_env迁移示例
---
napi_env对应到ani_env。
#### 代码示例对比

#### **N-API 示例**
```CPP
napi_value ObjectGetAllPropertyNames(napi_env env, napi_callback_info info)
```
#### **ANI 示例**
```CPP
ani_boolean handleData(ani_env *env, [[maybe_unused]] ani_object obj, ani_object value)
```

### napi_set_stackinfo迁移示例
---
#### 代码示例对比

#### **N-API 示例**

```
// NOLINTNEXTLINE(readability-identifier-naming)
napi_status __attribute__((weak)) napi_set_stackinfo(napi_env env, napi_stack_info *info);
```

#### **ANI 示例**

ani不支持运行时改变对象信息，需要运行前静态设置对象属性，因此无需迁移。

### napi_get_stackinfo迁移示例
---
#### 代码示例对比

#### **N-API 示例**

```
// NOLINTNEXTLINE(readability-identifier-naming)
napi_status __attribute__((weak)) napi_get_stackinfo(napi_env env, napi_stack_info *result);
```

#### **ANI 示例**

ani不支持运行时改变对象信息，需要运行前静态设置对象属性，因此无需迁移。
