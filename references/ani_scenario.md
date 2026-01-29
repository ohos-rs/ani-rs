# 资料索引

| 名称                      | 资源链接                                                                                                                                                                               | 主要用途                                     | 定位                         |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- | -------------------------- |
| ANI 上手教程（本文档）           | **[ani_scenario.md](https://gitee.com/liwentao_uiw/arkcompiler_runtime_core/blob/ani_spec/static_core/plugins/ets/runtime/ani/docs/ani_scenario.md)**                              | 提供 ANI 的全面资料，解决大多数 ANI 问题。               | 新手上手教程、新手必看、遇到 ANI 问题必查。   |
| NAPI 函数转 ANI 迁移示例       | **[napi2ani.md](https://gitee.com/liwentao_uiw/arkcompiler_runtime_core/blob/ani_spec/static_core/plugins/ets/runtime/ani/docs/napi2ani.md)**                                      | 提供 NAPI 到 ANI 的函数转换示例和处理方式，确保迁移正确性和兼容性。  | 工具书，不了解 NAPI 对应 ANI 写法时参考。 |
| ANI 使用示例 (ani_cookbook) | **[ani_cookbook](https://gitee.com/ironrain/ani_cookbook)**                                                                                                                        | 提供开箱即用的 ANI 函数示例代码，涵盖基础到复杂场景的应用。         | 新手上手、完整可用的代码用例。            |
| ANI 接口测试用例集             | **[ani/tests 测试用例文件夹](https://gitee.com/openharmony/arkcompiler_runtime_core/tree/OpenHarmony_feature_20241108/static_core/plugins/ets/tests/ani/tests)**                          | 包含所有已实现用例的功能验证，确保 ANI 函数可用性，代码必然可用。      | 工具书，验证 ANI 函数用法和功能时参考。     |
| ani.h 头文件               | **[ani.h](https://gitee.com/openharmony/arkcompiler_runtime_core/blob/OpenHarmony_feature_20241108/static_core/plugins/ets/runtime/ani/ani.h)**                                    | 核心定义文件，包含函数声明、参数说明、类型定义（如 ani_ref）及继承关系。 | 工具书，查阅 ANI 函数定义、类型和错误码时使用。 |
| 重要改动                    | **[更新日志](https://gitee.com/liwentao_uiw/arkcompiler_runtime_core/blob/ani_spec/static_core/plugins/ets/runtime/ani/docs/ani_scenario.md#23-%E6%94%B9%E5%8A%A8%E6%97%A5%E5%BF%97)** |                                          |                            |

**使用建议：**
在进行 NAPI 到 ANI 的函数迁移工作时，建议按照以下步骤利用上述资源:
1. 熟悉 ANI 函数规范: 首先仔细阅读 ani. h 头文件，理解 ANI 函数的参数定义、返回值类型以及各类 ani 类型的使用规范。这将为后续的函数迁移奠定坚实的基础。
2. 参考迁移示例: 结合 napi 2 ani. md 中的具体示例，学习如何将常见的 NAPI 函数逐步转换为 ANI 函数。注意观察示例中的代码结构、参数传递方式以及错误处理机制等细节。
3. 实践与验证: 利用 ani_cookbook 中的示例代码进行实际操作，尝试在自己的开发环境中运行这些示例，验证对 ANI 函数的理解和使用是否正确。通过实践不断加深对 ANI 函数的掌握程度。

# ANI 典型使用场景

## 1 loadLibrary

### 1.1 Native 函数绑定
Native 函数绑定分为几个步骤。
1. ets 文件中声明 native 函数。
2. ets 文件中的类声明 `static {loadLibrary("xxx")}` 静态代码块。
3. loadLibrary 中指向的 cpp 函数中声明 `ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)` 函数。步骤 2 中的静态代码块自动执行时将会触发该函数。
4. `ANI_Constructor` 中调用 Class_BindNativeMethods、Namespace_BindNativeFunctions、Module_BindNativeFunctions 几种 native 函数绑定方法。
5. 根据各自 native 函数声明的位置，需要调用不同的绑定函数。其中，class 函数绑定时，除了 ani_env，还**必须添加** ani_object/ani_class 参数，仅有 class 绑定需要如此，参考下述表格。

函数绑定时，使用不同 ANI 函数会有以下区别：

| 绑定目标类型               | 绑定函数                            | 需要补充的参数                 | 描述                    |
| -------------------- | ------------------------------- | ----------------------- | --------------------- |
| `class` 的非 static 方法 | `Class_BindNativeMethods`       | `ani_env`, `ani_object` | 需要`ani_object`来操作类实例。 |
| `class` 的static 方法   | `Class_BindNativeMethods`       | `ani_env`, `ani_class`  | 需要 `ani_class` 来操作类。  |
| `namespace`          | `Namespace_BindNativeFunctions` | `ani_env`               |                       |
| `module`             | `Module_BindNativeFunctions`    | `ani_env`               |                       |
注：**需要补充的参数**是指：如果调用绑定函数进行绑定，需要在 native 函数的参数开头添加这些参数。
如：
```cpp
// Class_BindNativeMethods绑定。ani_object也可以是ani_class取决于目标native函数ets声明是static还是非static
void nativeFoo(ani_env* env, ani_object obj, ...)

// Namespace_BindNativeFunctions/Module_BindNativeFunctions绑定
void nativeFooM(ani_env* env, ...)
```

注意点：
1. **被绑定的函数的入参都是伪类型，因为是通过 `void*` 传递出去的地址进行反向绑定得到固定地址，进行了类型擦除**，所声明的类型与真实类型可以不一致，运行时真正的类型取决于其真实传入的类型。但是编译时的调用和类型转换会受限于其声明的伪类型。
2. 由于 CPP 的 native 实现函数的入参是通过 `void*` 传递出去的地址进行反向绑定得到固定地址，所以其是一个多级指针，其内容会在每次函数调用，离开作用域后进行回收。所以不能直接将其存储到对象中。解决方法见《9. 节生命周期管理》
3. native 无法声明在 interface 中。
4. native 无法声明在 get/set 前，所以不能将 property 声明为 native 方法，但存在替代方法，见《1.1.5 节函数绑定的特殊场景》

#### 1.1.1 classname、identifier、symbol 查询
-----
参照反汇编工具 `ark_disasm` 的结果。反汇编工具构建方法见 《1.1.6 节 ABC》 文件反汇编

例如：有 `test_a.abc`, `test_b.abc`, `test_c.abc`, `test_d.abc` 链接成的 `aniSignature.abc`。`test_a.abc` 中存在 `Foo` 类

**对应的反汇编为**
```cpp
.record aniSignature.test_a.Foo <ets.extends=std.core.Object, access.record=public>
```
则对应的 classname 为 `LaniSignature/test_a/Foo;`

#### 1.1.2 class 中的函数绑定
----------------------
**ArkTS 代码**
```cpp
class PasteData{
    // libxxx.so
    static {loadLibrary("xxx")}
    native getRecordCount(a: int):int;
}
```
**abc 文件反汇编**
```cpp
// 文件名returnValue.abc
// native函数反汇编格式
.function i32 returnValue.PasteData.getRecordCount(returnValue.PasteData a0, i32 a1) <native, access.function=public>
```

**cpp 中对应绑定的 native 函数**
```cpp
static ani_int getRecordCount([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object/* 代表class自身，等效this指针 */, ani_int a);

std::array methods = {
        ani_native_function{
        "getRecordCount", // ets中的native修饰的函数的函数名
        "I:I", // ets中native修饰的函数的入参和返回值类型
        reinterpret_cast<void *>(getRecordCount) // cpp中要绑定的原生函数
        },
    };
// 对应的绑定函数
env->Class_BindNativeMethods(cls, methods.data(), methods.size());
```

`reinterpret_cast<void *>是在做类型擦除以进行绑定，永远只能用void*进行转换`

注：需要在 cpp 的 native 函数中添加参数 `[[maybe_unused]] ani_object object` 承接编译时附加的 object 参数，否则无法正确获取后续参数

`[[maybe_unused]] ani_object object/* 代表class自身，等效this指针 */` 也可以是

`[[maybe_unused]] ani_class object/* 静态函数绑定到的class */`

#### 1.1.3 namespace 中的函数绑定
--------------------------
```cpp
namespace PasteData{
    // libxxx.so
    loadLibrary("xxx");
    native function getRecordCount(a: int):int;
}
```
**abc 文件反汇编**
```cpp
// 文件名returnValue.abc
// native函数反汇编格式
.function i32 returnValue.PasteData.getRecordCount(i32 a0) <native, static, access.function=public>
```
**cpp 中对应绑定的 native 函数**
```cpp
// 不需要额外补充的ani_object
static ani_int getRecordCount([[maybe_unused]] ani_env *env, ani_int a);
```
**CPP 调用 ANI 将 native 函数实现绑定到ets的 native 声明上。**
```cpp
std::array methods = {
        ani_native_function{
        "getRecordCount", // ets中的native修饰的函数的函数名
        "I:I", // ets中native修饰的函数的入参和返回值类型
        reinterpret_cast<void *>(getRecordCount) // cpp中要绑定的原生函数
        },
    };
// 对应的绑定函数
env->Namespace_BindNativeFunctions(ns, methods.data(), methods.size());
```

`reinterpret_cast<void *>是在做类型擦除以进行绑定，永远只能用void*进行转换`

- **注意：不需要在 cpp 的 native 函数中添加额外参数声明！！！与反汇编保持一致即可。**

- **注意：loadLibrary 在 namespace 中时，由于没有 static 静态代码块，所以 namespace 必须被 import { PasteData }类似的形式被显示 import 时激活懒加载，参考《类加载失败定位 1.2.1.1 节》。**
因此如果该文件中的其他 native 修饰函数如果 namespace 不被显示 import 时将会导致这些函数没有被绑定实现。

> [! Warning] **注意**
> **namespace 绑定与 class 存在差异，不允许 class 方法与 namespace 方法混用。**
> 见更新日志

#### 1.1.4 module 函数绑定
-------------------
```cpp
// libxxx.so
loadLibrary("xxx");
namespace a{
}
class b{}
enum  COLORINT{
    REDINT = 5
}
native function processEnumInt(color : COLORINT) : void;
```
**abc 文件反汇编**
```cpp
// 文件名ani_enum.abc
// native函数反汇编格式
.function void ani_enum.ETSGLOBAL.processEnumInt(ani_enum.COLORINT a0) <native, static, access.function=public>
```
**CPP的 native 函数实现**
```cpp
// 不需要额外补充的ani_object
static void processEnumInt([[maybe_unused]] ani_env *env, ani_enum_item enumItem);
```
**CPP 调用 ANI 将 native 实现绑定到ets层的 native 声明上。**
```cpp
ani_module module;
env->FindModule("Lani_enum;", &module);
std::array methods = {
    ani_native_function {"processEnumInt", "Lani_enum/COLORINT;:V", 
    reinterpret_cast<void *>(processEnumInt)},
};
env->Module_BindNativeFunctions(module, methods.data(), methods.size());
```

`reinterpret_cast<void *>是在做类型擦除以进行绑定，永远只能用void*进行转换`

> [! Warning] **注意**
> **ETSGLOBAL 绑定已和 mdoule 函数绑定合并。不能使用 Class_BindNativeMethods 绑定 ETSGLOBAL 下的 native 方法**
> **见更新日志**

#### 1.1.5 函数绑定的特殊场景
- native 构造函数绑定时如果参数存在可选参数为重载实现，需要根据重载形式绑定多个 `ctor` 函数。建议反汇编查看具体内容。
- native 无法声明在 interface 中。
- native 无法声明在 get/set 前，所以无法声明一个 native 的 getter。但可以修改 get/set 内的实现，调用一个 native 方法，实现实际上的绑定。
```ts
class PersonInner {
    thisIsField: int = 3;
    _thisIsProperty:int = 10086
    native static createPerson():PersonInner;
    native thisIsPropertyGetter():int
    native thisIsPropertySetter(i:int):void
    get thisIsProperty():int{
        return this.thisIsPropertyGetter();
    }
    set thisIsProperty(i:int){
        this.thisIsPropertySetter(i);
        return;
    }
}

a.thisIsProperty = 1008611; // call `get thisIsProperty()`
console.log("thisIsProperty after change: " + a.thisIsProperty) // call `set thisIsProperty()`
```

#### 1.1.6 ABC 文件反汇编
abc 文件反汇编必须有 ark_disasm 二进制文件及其相关依赖。

ark_disasm 是通过 [arkcompiler_runtime_core仓](https://gitee.com/openharmony/arkcompiler_runtime_core/tree/OpenHarmony_feature_20241108/) 和 [arkcompiler_ets_frontend仓](https://gitee.com/openharmony/arkcompiler_ets_frontend/tree/OpenHarmony_feature_20241108/)进行编译得到的，[编译方法见这里，只需要看 1 和 2](https://gitee.com/JianfeiLee/arkcompiler_runtime_core/wikis/%E4%B8%8B%E8%BD%BD%E5%92%8C%E7%BC%96%E8%AF%91%E8%BF%90%E8%A1%8CArkTS%E6%BC%94%E8%BF%9B%E7%89%88%E4%BB%A3%E7%A0%81)。

这个不是通过编译 so 和 abc 文件的镜像命令生成的，不在全仓代码产物 out 目录中。原本已有的代码仓注意切换到 OpenHarmony_feature_20241108 分支。

反汇编命令：（这个不是生成镜像的命令生成的，也不在全仓目录下的 out 中）

`./out/bin/ark_disasm yourabcfile.abc dumpfile.txt`

如果反汇编失败，一般情况下是编译工具和反汇编工具版本差异过大，请更新代码和配套工具。如果使用最新工具链均不能解决问题，请咨询前端编译器同学。联系方式见提问群公告 wiki 最后一条“沟通地图”。


### 1.2 类加载失败问题定位
错误异常分两类：
1. LinkerUnresolvedClassError：是 Import 失败导致的，abc 加载没有成功或者 abc 导入不正确，还没有执行到 abc 的逻辑；编译没有报错，设备测试运行就报错，一般是这个原因。

类似下述关键字：
```
at std.core.LinkerUnresolvedClassError.<ctor> (<unknown>:36)
```
2. NoClassDefFoundError：import abc 成功了，但是调用的类，或者方法签名与声明不一致；需要查看对应反汇编字节码；

类似下述关键字：
```
[TID 007ecd] E/runtime:Unhandled exception: std.core.NoClassDefFoundError
```

#### 1.2.1 LinkerUnresolvedClassError 问题排查
---
##### 1.2.1.1 loadLibrary 写法不正确
ArkTS 1.2 默认懒加载，这里 loadLibrary 写在 export default fileIo 后；那么在执行 import fileIo 的时候，loadLibrary 不会执行，只有 import BussinessError 的操作，才会触发；

最好把 loadLibrary 写在 class 的 static 代码块保证能够执行。
```ts
export default fileIo;
loadLibrary("ani_fs_class.z")
export class BussinesError<T = void> {
//...
}
```

##### 1.2.1.2 设备缺少 abc 加载
1、查看程序的进程，确定程序运行是否加载所属的 abc 模块，如果没有执行步骤 2
```
cat /proc/pid/maps | grep abc
```
2、/system/framework 路径下是否有所需 abc 模块，如果有，操作步骤 3
3、/system/framework/bootpath.json 里是否有所需 abc 模块的加载路径，如果没有需要添加路径
```
hdc file recv /system/framework/bootpath.json ./
hdc file send bootpath.json /system/framework/bootpath.json
修改该文件需要重新设备!!!
```
4、bootpath.json 缺少模块 abc 加载路径，是由于模块打包的时候缺少配置，在 build. Gn 的 generate abc 里添加 `is_boot_abc="True"`

----
#### 1.2.2 NoClassDefFoundError 排查
1、查看定义的 abc 和调用 abc 的字节码

> [! Warning] **注意**
> **ETSGLOBAL 绑定后续将会和 mdoule 函数绑定合并。不能使用 Class_BindNativeMethods 绑定 ETSGLOBAL 下的 native 方法**
> **见更新日志**

下面的例子就是定义和调用的名字不匹配。Loadlibraries. ETSGLOBAL 定义的名字是

`@kolaui.interop.loadLibraries.ETSGLOBAL.loadNativeLibrary`

调用的时候是

`loadLibraries.ETSGLOBAL.loadNativeLibrary`

名字没有匹配；不匹配是不正确的；所以可能 arktsconfig 写法有问题，或者存在编译问题；

可以使用 ark_disasm 工具反汇编查看具体信息。
```
bin/ark_disasm xxx.abc xxx.txt
```

## 2 Mangling
通过字符编码区分重载函数，格式为 `参数类型:返回类型`，例如：
- `toInt(num: number): int` → `D:I`
- `toInt(str: string): int` → `Lstd/core/String;:I`

`Object_CallMethodByName_Int(obj, "toInt", "D:I", &result);` 

表示找到声明为 toInt (num: number): int 的函数。

`Object_CallMethodByName_Int(obj, "toInt", "Lstd/core/String;:I", &result);` 

表示找到声明为 toInt (num: string): int 的函数。

| **示例 ArkTS 类型** | **Mangling 示例** | **ANI 类型**   |
| --------------- | --------------- | ------------- |
| `boolean`       | `Z`             | `ani_boolean` |
| `byte`          | `B`             | `ani_byte`    |
| `char`          | `C`             | `ani_char`    |
| `short`         | `S`             | `ani_short`   |
| `int`           | `I`             | `ani_int`     |
| `long`          | `J`             | `ani_long`    |
| `float`         | `F`             | `ani_float`   |
| `double`        | `D`             | `ani_double`  |
| `number`        | `D`             | `ani_double`  |
| `void`          | `V`             | `void`        |

| **类型描述**  | **示例 ArkTS 类型**   | **Mangling 示例**                       | **备注**                                                                               |
| --------- | ----------------- | ------------------------------------- | ------------------------------------------------------------------------------------ |
| **对象类型**  |                   |                                       |                                                                                      |
| 类类型       | `class CustomCls` | `Lmodule1Name/module2Name/CustomCls;` | `L` 开头，类名和模块名用 `/` 分隔，以 `;` 结尾                                                       |
| 类类型       | `string`          | `Lstd/core/String;`                   | `L` 开头，类名和模块名用 `/` 分隔，以 `;` 结尾                                                       |
| 类类型       | `bigint`          | `Lescompat/BigInt;`                   | 在 escompat 包模块中声明                                                                    |
| 类类型       | `Array`           | `Lescompat/Array;`                    | `[即将过时]该类型不适配 Array_XXX 等接口，也并非 ani_array 类型。 [即将到来]为保持兼容性之后会变成ani_array，定长数组将有新类型 ` |
| 函数对象      | `()=>void`        | `Lstd/core/Function0;`                | 数字表示这个函数对象的的参数数量                                                                     |
| **数组类型**  |                   |                                       |                                                                                      |
| 一维数组      | `int[]`           | `[I`                                  | `[` 后跟元素类型                                                                           |
| 二维数组      | `int[][]`         | `[[I`                                 | 每增加一维，添加 `[`                                                                         |
| 对象数组      | `String[]`        | `[Lstd/core/String;`                  | 非基本类型以 `;` 结尾                                                                        |
| **空值**    |                   |                                       |                                                                                      |
| null      | `null`            | `Lstd/core/Object;`                   | GetNull 创建对象                                                                         |
| undefined | `undefined`       | `Lstd/core/Object;`                   | GetUndefined 创建对象                                                                    |
| **其他类型**  |                   |                                       |                                                                                      |
| 泛型        |                   |                                       | 见下面的总结和示例                                                                            |
| 联合类型      |                   |                                       | 见下面的总结和示例                                                                            |
| 可选参数      |                   |                                       | 见下面的总结和示例                                                                            |
| 默认参数      |                   |                                       | 同可选参数                                                                                |

**核心规则总结**
1. **参数与返回值分隔**  
   使用 `:` 分隔参数列表与返回值（如 `DD:I` 表示两个 double 参数，返回 int）。
2. **无参函数格式**  
   无参数且返回 void 的函数 Mangling 为 `:V`。没有声明返回值将会自动返回 void。没有参数可省略，返回值不可省略。
3. **参数组合规则**  
   - 基本类型连续排列（如 `II:V` 表示两个 int 参数）。
   - 非基本类型需用分号表示这是一个入参（如 `[Lstd/core/String;[I:V` 表示数组和 int 参数）。
1. **类与模块路径**  
   - 格式：`L<模块名>/<类名>;`
   - 所有类以 `L` 开头，分号结尾（如 `Lhello_ani/A;`）。
   - 未显式声明模块时，默认使用文件名作为模块名（如 `hello_ani.ets` 中的类 A 是 `Lhello_ani/A;`）。
   - 系统类模块名固定为 `std.core -> std/core` 或 `escompat`。
   - 模块名中存在 `.` 点符号时需要替换成 ` / ` 斜杠符号。
1. **数组表示法**  
   - 一维数组：`[` + 元素类型（如 `[I`）。
   - 多维数组：逐层添加 `[`（如 `[[I` 表示二维 int 数组）。
6. **泛型与联合类型**  
   - 统一映射为 `Lstd/core/Object;`。
   - `native function foo<T>():void` 中泛型参数不会改变 mangling，因为 native 函数内部的行为完全由 cpp 层自行决定。
7. **可选参数装箱**  
   - 基本类型可选参数会被装箱（如 `variable?:int` → `Lstd/core/Int;`），需通过类方法访问。
   - 非基本类型可选参数保持原类型。
8. **函数作为参数**  
   使用 `Lstd/core/FunctionN;`，`N` 表示参数数量（如 `Function2` 表示两个参数）。

“装箱”指变量的类型由基本类型 int、double、float 等变为 Int、Double、Float 等类的实例的过程，相应的会得到对应类的方法，不能直接将其作为 ani_int、ani_double、ani_float 等使用。具体调用方法见 interface/class->装箱类型一节。

**示例说明：**
```ts
// 以下均声明在hello_ani文件下，未做特别声明自动将文件名作为moduleName
class A {/*...*/}
class B {/*...*/}
namespace NS {
    class C {/*...*/}
}
function f():void // Mangling ":V"
function f(a:int):void // Mangling "I:V"
function f(a:int, b:int):void // Mangling "II:V"
function f(a:number, b:double):int // Mangling "DD:I" number是double的别名。

function f(a:Array<string>):void // Mangling "Lescompat/Array;:V"

// Mangling "ZBLstd/core/String;Lhello_ani/A;Lstd/core/Object;Lstd/core/Object;:V"
function f<T>(a:boolean, b:byte, c:string, d:A, f:A|B, e:T):void

// Mangling "ILstd/core/Int;Lstd/core/String;Lhello_ani/A;:V" 从第2个可选int被装箱为Int类型
function f(a:int, b?:int, c?:string, d?:A):void
// Mangling "ILstd/core/Int;[Lstd/core/String;[I:V"
function f(a:int[], b:string[], ...c:int[]):void
// Mangling "Lstd/core/Function;Lstd/core/Function;Lstd/core/Function;:V"
function f(a:()=>void,b:()=>string,c:(x:int):string):void
```

## 3 类型定义

### 3.1 原生类型/基本类型
```cpp
// ets基本类型的运行时类型
typedef uint8_t  ani_boolean;  // 布尔型 (1字节) ETS声明: boolean
typedef uint16_t ani_char;     // 字符型 (2字节) ETS声明: char
typedef int8_t   ani_byte;     // 字节型 (1字节) ETS声明: byte
typedef int16_t  ani_short;    // 短整型 (2字节) ETS声明: short
typedef int32_t  ani_int;      // 整型 (4字节)   ETS声明: int
typedef int64_t  ani_long;     // 长整型 (8字节) ETS声明: long
typedef float    ani_float;    // 单精度浮点     ETS声明: float
typedef double   ani_double;   // 双精度浮点     ETS声明: double/number
```
 number 是 double 类型的别名。

由于 ani_boolean 是 uint 8_t 而不是 bool，所以无法使用 cout 等 cpp 流进行直接输出，可以强制转换为 int 再进行输出。

### 3.2 扩展类型
| 类型定义              | 描述                  | 备注                                                                       |
| ----------------- | ------------------- | ------------------------------------------------------------------------ |
| `ani_ref`         | `ani_object` 的基类    |                                                                          |
| `ani_object`      | 任意非基本类型             | 不包括基本类型 ani_int 之类                                                       |
| `ani_error`       | `Error`             |                                                                          |
| `ani_fn_object`   | `Function/()=>void` |                                                                          |
| `ani_arraybuffer` | `ArrayBuffer`       |                                                                          |
| `ani_string`      | `string`            |                                                                          |
| `ani_array`       | `T[]`               | **与 `Array<T>` 不是同一个类型！！！**`Array<T>` 并不是 ani_array 类型，而是 ani_object 类型。 |
如果一个函数的入参要求 ani_ref 或者 ani_object，而需要传入一个 ani_int 类型，那么就需要将其进行装箱得到 ani_ref 见 4.1 节。
### 3.3 类型转换
下述链路上的类型，如果是“父子关系”可以通过 static_cast 进行类型转换，而不在这个链路上的则无法转换为 ani_ref 或者 ani_object。典型的如：ani_method。

但是以下转换必须保证其 ets 运行时层的可用性。例如：利用转换成 ani_ref，试图将 ani_string 转换成 ani_ref 当成 ani_array 使用，会运行时 crash 的。
```
ani_ref
├── ani_module
├── ani_namespace
├── ani_object
│   ├── ani_fn_object
│   ├── ani_enum_item
│   ├── ani_error
│   ├── ani_tuple_value
│   ├── ani_type
│   │   ├── ani_class
│   │   ├── ani_enum
│   │   └── ani_union
│   ├── ani_arraybuffer
│   ├── ani_string
│   └── ani_array
│       ├── ani_array_boolean
│       ├── ani_array_char
│       ├── ani_array_byte
│       ├── ani_array_short
│       ├── ani_array_int
│       ├── ani_array_long
│       ├── ani_array_float
│       ├── ani_array_double
│       └── ani_array_ref
```
eg: `auto str = static_cast<ani_string>(string_ref);`

- 如何将 ani_int/ani_double 等基本类型转换为 ani_ref？

参考 《4.1 节装箱》的步骤。

### 3.4 类型识别 Object_InstanceOf

ani_object 实际上在入参中代表所有类型，即如果存在自定义类型 A、B 的联合类型入参，必须使用 Object_InstanceOf 识别其类型。
```ts
type DataType = string | Object | ArrayBuffer
native function handleData(data: DataType):void  

function main(){
    loadLibrary("ani_union")
    handleData("hello") // Object is String Object Content:hello
    handleData(new ArrayBuffer(1024)) // Object is ArraryBuffer Lenght:1024
    handleData(new Array<int>) // Object is Other Class
}
```

```cpp
static void handleData_union(ani_env *env, ani_object obj, ani_object union_obj){
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
    std::cout << "Object is Other Class"<< std::endl;
	return;
}
```

## 4 出入参处理

### 4.1 装箱/拆箱

| 行为        | ETS 层 → C++ 层                       | C++ 层 → ETS 层                      |
| --------- | ----------------------------------- | ---------------------------------- |
| 装箱责任方     | ETS 层自动完成                           | C++ 层需手动调用装箱方法                     |
| ets 层行为   | 自动装箱                                | 自动拆箱                               |
| 数据转换方向    | 基本类型 → 装箱实例                         | 装箱实例 → 基本类型                        |
| cpp 层行为   | 手动拆箱                                | 手动装箱                               |
| 典型代码逻辑    | ETS直接传递基本类型，无需额外操作。CPP 层获取数据需要手动拆箱。 | CPP需显式调用 `Object_New()` 等方法生成装箱实例。 |
| ETS 层的透明性 | 对用户隐藏装箱细节                           | 对用户隐藏拆箱细节                          |

#### 4.1.1 装箱类型

| ets 类型    | ets 装箱类型  | Mangling             | binding/ANI 类型 | 备注         |
| --------- | --------- | -------------------- | -------------- | ---------- |
| `boolean` | `Boolean` | `Lstd/core/Boolean;` | `ani_object`   | 布尔型装箱类     |
| `byte`    | `Byte`    | `Lstd/core/Byte;`    | `ani_object`   | 字节型装箱类     |
| `char`    | `Char`    | `Lstd/core/Char;`    | `ani_object`   | 字符型装箱类     |
| `short`   | `Short`   | `Lstd/core/Short;`   | `ani_object`   | 短整型装箱类     |
| `int`     | `Int`     | `Lstd/core/Int;`     | `ani_object`   | 整型装箱类      |
| `long`    | `Long`    | `Lstd/core/Long;`    | `ani_object`   | 长整型装箱类     |
| `float`   | `Float`   | `Lstd/core/Float;`   | `ani_object`   | 单精度浮点装箱类   |
| `double`  | `Double`  | `Lstd/core/Double;`  | `ani_object`   | 双精度浮点装箱类   |
| `number`  | `Double`  | `Lstd/core/Double;`  | `ani_object`   | double 的别名 |
| `void`    | `Void`    | `Lstd/core/Void;`    | `ani_object`   | 通常无意义      |

这些类的可调用方法，可以参考《5.1 节系统类的方法查询》查询目标类的所属模块和其方法。

当函数入参的 mangling 为 `Lstd/core/Object;` 时，其入参只能是类的实例，典型的如 `Array<T>` 的方法 `$_set(i: int, val: T): void`，其 mangling 为 `ILstd/core/Object;:V`，第二个入参为一个类的实例对象。

在这种情况下，int、double 等基本类型的值需要转换成 Int、Double 等装箱类型的实例。根据《3.3 节类型转换》这样得到的 ani_object 也可以转换为 ani_ref。可以满足特定函数的 ani_ref 入参的要求。

同理 `[Lstd/core/Object;` 是类实例组成的数组。
#### 4.1.2 装箱
装箱的用途：
1. 满足函数入参的类型需求
2. 将基本类型的值转换为类对象，即 ani_int 这类基本类型转换为 ani_ref 这类动作。

装箱指类型为可选参数时，会自动将 int/double/float 等基本类型转换为 Int/Double/Float 等装箱类型。开发者缺省可选参数时等效于传入 undefined。

- 创建一个装箱对象
```ts
ani_object createDouble(ani_env *env){
    static const char *className = "Lstd/core/Double;";
    ani_class persion_cls;
    if (ANI_OK != env->FindClass(className, &persion_cls))
    {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return;
    }
    ani_method personInfoCtor;
    env->Class_FindMethod(persion_cls, "<ctor>","D:V", &personInfoCtor);
    ani_object personInfoObj;
    env->Object_New(persion_cls, personInfoCtor, &personInfoObj, ani_double(2.0));
    return personInfoObj;
}
```
可以参考《5.1 节系统类的方法查询》查询目标类的所属模块和其方法。

上述构造函数存在多个重载，可以自己选择需要的重载形式。

#### 4.1.3 拆箱
拆箱需要调用 `unboxed` 函数进行拆箱获取值。

`Object_CallMethodByName_Double (boxed_double_obj, "unboxed",":D" ,&unboxed_value)`

其中 `Object_CallMethodByName_Double` 中的 Double 是由返回值决定的，即认为这是一个 Double 的装箱类型的实例 unboxed 将会返回 double 基本数据，mangling 对应匹配为 `:D`，表示返回 double

示例：
```ts
function handleData(param: Double) // 大写的Double是一个类
```

```cpp
ani_double param_value;
env->Object_CallMethodByName_Double(static_cast<ani_object>(param_ref), "unboxed",":D" ,&param_value);
```

### 4.2 泛型参数 generic
当 mangling 为 `Lstd/core/Object;` 时，需要传入基本类型，请参考《4.1 节装箱/拆箱》，cpp 层传入参数时需要装箱，cpp 获取函数返回值时需要拆箱。

| ets 类型       | 实际场景         | Mangling                             | binding/ANI 类型 | 备注      |
| ------------ | ------------ | ------------------------------------ | -------------- | ------- |
| `a: T, b: R` | `f(1,"str")` | `Lstd/core/Object;Lstd/core/Object;` | `ani_object`   |         |
| `Array<T>`   | `Array<int>` | `Lstd/core/Array;`                   | `ani_object`   |         |
| `T[]`        | `int[]`      | `[I`                                 | `ani_array`    | 非标准泛型场景 |

1. 泛型参数不影响 Mangling 和 ANI 类型。
2. 需要泛型参数的类在 ANI 通过 Object_New 创建时会根据入参自动配置。


### 4.3 联合类型  union 
联合类型无影响，均为 `Lstd/core/Object` 对应 ANI 类型 ani_object。

**注：仅与 undefined 联合时转换成可选参数场景。请看《4.4 节可选参数》。**

当 mangling 为 `Lstd/core/Object;` 时，需要传入基本类型，请参考《4.1 节装箱/拆箱》，cpp 层传入参数时需要装箱，cpp 获取函数返回值时需要拆箱。

| ets 类型           | Mangling            | binding/ANI 类型 | 备注       |
| ---------------- | ------------------- | -------------- | -------- |
| a: double \| int | `Lstd/core/Object;` | `ani_object`   | 基本类型需要装箱 |

**实参处理**：

需要根据具体的实参调用 `Object_InstanceOf` 判断属于哪个具体的类型。根据实参的真实类型进行需要的处理。

示例跳转：
```ts
type DataType = string | Object | ArrayBuffer
native function handleData(data: DataType):void  

function main(){
    loadLibrary("ani_union")
    handleData("hello") // Object is String Object Content:hello
    handleData(new ArrayBuffer(1024)) // Object is ArraryBuffer Lenght:1024
    handleData(new Array<int>) // Object is Other Class
}
```

```cpp
static void handleData_union(ani_env *env, ani_object obj, ani_object union_obj){
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
    std::cout << "Object is Other Class"<< std::endl;
	return;
}
```
[ani_union/ani_union.cpp · ironrain/ani_cookbook - 码云 - 开源中国](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_union/ani_union.cpp)

### 4.4 可选参数
**Mangling**:
1. 基本类型的场景：int、double 等将会自动在前端转换成 Int、Double 等类对象。
	`Lstd/core/Int;` 、`Lstd/core/Double;`
2. 非基本类型的场景：mangling 保持不变。
3. 可选参数的类型超过 2 个 (? 号是联合 undefined 的语法糖)，例如 a?:number|int 等效 a:number|int|undefined，转换为联合类型场景。
4. 构造函数中存在可选参数时为函数重载实现，将在 abc 文件中存在多个构造函数，需要逐个绑定。

当 mangling 为 `Lstd/core/Object;` 时，需要传入基本类型，请参考《4.1 节装箱/拆箱》，cpp 层传入参数时需要装箱，cpp 获取函数返回值时需要拆箱。

| ets 类型            | Mangling                 | binding/ANI 类型 | 备注                      |
| ----------------- | ------------------------ | -------------- | ----------------------- |
| `a: int`          | `I`                      | `ani_int`      |                         |
| `a?: int`         | `Lstd/core/Int;`         | `ani_object`   | 基本类型可选参数，发生装箱行为         |
| `a: Int`          | `Lstd/core/Int;`         | `ani_object`   |                         |
| `a?: Int`         | `Lstd/core/Int;`         | `ani_object`   | 非基本类型的场景：mangling 保持不变。 |
| `a?: customeCls`  | `LmoduleName/CustomCls;` | `ani_object`   | 非基本类型的场景：mangling 保持不变。 |
| `a?: number\|int` | `Lstd/core/Object;`      | `ani_object`   | 联合类型                    |

**实参处理：**
如果无法确认实参传入的对象的真实类型，请先判空。

`Reference_IsUndefined (optional_obj, &isUndefined)`

示例跳转：

[ani_optional_parameter/ani_optional_parameter.cpp · ironrain/ani_cookbook - 码云 - 开源中国](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_optional_parameter/ani_optional_parameter.cpp)
### 4.5 默认参数
参考可选参数场景 1 与场景 2：
1. 基本类型的默认参数会进行自动装箱，
2. 非基本类型的场景：mangling 保持不变。
```ts
// ZLstd/core/Boolean;:V
function foo(a:boolean, b:boolean = False):void{}
```
装箱的场景在 ets 层调用，可以传入基本类型，在 cpp 层获取时得到自动装箱后的装箱实例；在 cpp 层调用时，需要手动装箱，传入装箱后的实例，ets 层会自动拆箱得到基本类型。

### 4.6 可变参数/剩余参数
可变参数即数组。这在 ets 中属于一个前端语法糖，自动将后续的参数组合生成成数组。
```ts
// I[I:V
function foo(a:int, ...b:int[]):void
```

## 5 对象创建

### 5.1 系统类的方法查询

在 `openharmony/arkcompiler/runtime_core/static_core/plugins/ets/stdlib/` 这个路径下查询需要的标准库函数。如果不知道自己需要查询哪个标准库的类，请反汇编确认其名称。

例 1：

`ArrayBuffer`

关键词"class ArrayBuffer" vscode 搜索路径限制为 `stdlib/**/*.ets`，找到 ArrayBuffer.ets 文件的 50 行

代码片段：
```ts
export class ArrayBuffer extends Buffer
{
	public constructor(length: int, maxByteLength?: int)
	public constructor(length: number, maxByteLength?: number)
    public static isView(obj: Object): boolean
	get byteLength(): number 
	private data: byte[] | undefined
```
从上面的代码段落中，可以看到 Array 对象中存在
1. 两个构造函数，其函数名在 ANI 接口识别时为字符串 `<ctor>`，第一个函数的 mangling 可以写成 `ILstd/core/Int;:V`。**禁止使用 nullptr 代替构造函数搜索时的 signature。**
2. 名为 `isView` 的方法，参数为 Object，返回值为 boolean，其 mangling 可以写成 `Lstd/core/Object;:Z`
3. 名为 byteLength 的 property
4. 名为 data 的 filed

### 5.2 自定义类的方法查询

### 5.3 非标准类的方法查询
典型的如 enum，其定义由前端编译器自行在 abc 中进行实现。其具体方法需要在 abc 文件中进行反汇编获取。

### 5.4 类实例创建
1. FindClass 找到需要创建的类。
2. Class_FindMethod 找到需要创建的类的构造函数，根据 mangling 将会匹配指定的构造函数。**禁止使用 nullptr 此时代替 mangling，会导致非常多种的问题。**
3. Object_New 创建对象。
例如：
```ts
// filename:ani_hello.ets
// moduleName：ani_hello
// 由于不做配置模块名默认为文件名

// 自定义一个类
class Point {
x:int
y:int
constructor(x:int,y:int){
this.x = x; this.y = y;
}
constructor(x:number,y:number){
this.x = (int)x; this.y = (int)y;
}
}
```

 1. FindClass
 ```cpp
 ani_class cls;
 env->FindClass("Lani_hello/Point;", &cls);
 ```
2. Class_FindMethod
```cpp
ani_method ctor1;
env->Class_FindMethod(cls, "<ctor>", "II:V", &ctor1);
ani_method ctor2;
env->Class_FindMethod(cls, "<ctor>", "DD:V", &ctor1);
```
上述例子中 

Mangling : `II:V` 将会匹配到 `constructor(x:int,y:int)`；

Mangling : `DD:V` 将会匹配到 `constructor(x:double,y:double)`；

**禁止使用 nullptr 代替构造函数搜索时的 signature。

3. Object_New
```cpp
ani_object obj1;
env->Object_New(cls, ctor1, &obj1, ani_int(1), ani_int(2));
ani_object obj2;
env->Object_New(cls, ctor2, &obj1, ani_double(1), ani_double(2));
```
上述例子中
创建了两个对象，`obj1` 和 `obj2 `，`obj1` 是调用 `constructor(x:int,y:int)` 创建出来的，`obj2` 是调用 `constructor(x:double,y:double)` 创建出来的。

**对于 ArkTS 标准库中的类创建实例**
1. FindClass 需要在 `arkcompiler/runtime_core/static_core/plugins/ets/stdlib` 这个路径下查找所需的类的类名。其类名声明的文件会声明所在的 package，一般为 `std.core` 或者 `escompat`，对应的 mangling 为 `Lstd/core/<ClassName>;` 或 `Lescompat/<ClassName>;`。
2. Class_FindMethod 找到目标类之后，查看其构造函数，根据需要的构造函数描述 Mangling 匹配目标构造函数。
3. Object_New 调用该函数创建即可。

ANI 中创建对象需要注意 ANI 层中无法用 Object_New 创建以下类型：
- Interface 接口类
- Abstract 抽象类
- String 字符串类（专门的 ANI 接口创建）
- `T[]` 变长数组类（专门的 ANI 接口创建）


## 6 interface/class
Interface 无法在 ANI 层直接创建对象，需要先实现（class implements interface）后，可以调用。

Interface 中的字段自动声明为 property。Class 实现其 interface 后，来自 interface 的属性也为 property。
```TS
interface PointI {
x:int // property
y:int // property
}
class Point implements PointI {
x:int // property
y:int // property
z:int // field
}
```

```cpp
ani_class clsPointI; ani_method ctorPointI; // 假设已赋值
ani_class clsPoint; ani_method ctorPoint; //   假设已赋值
ani_object objPointI; ani_object objPoint;
env->Object_New(clsPointI, ctorPointI, &objPointI); // fail
env->Object_New(clsPoint, ctorPoint, &objPoint); //    success
```
直接创建 interface 对象失败。

直接创建 class 对象成功。

ANI 中，class 对象创建不依赖与 interface，但 interface 对象创建需要实现 class。
ets 层中可以通过下面的方法创建 interface 对象。其行为是在前端隐式创建一个匿名类帮助其进行构建。
```ts
let p: PointI = { x:1 , y:1};
```
有兴趣可以反汇编 abc 文件查看前端转换的结果。

### 6.1 property&field
- Property: 含有 get/set 方法的类中属性 property
- Field: 没有 get/set 方法的相关属性即为 field
- **interface 中声明的属性均为 property，实现（class implements interface）之后，class 中 interface 已声明的属性均为 property。**

**property 与 field 相关方法不能混用！Property 与 field 相关方法不能混用！！！！！！！！**

ANI 接口中含有 field 相关字符的函数不能被 property 调用。例如：`Class_FindField`

ANI 接口中含有 property 相关字符的函数不能被 field 调用。例如：`Object_GetPropertyByName_Boolean`

```ts
interface PointI {
x:int // property
y:int // property
}

class Point implements PointI {
x:int // property
y:int // property
z:int // field
}
```

### 6.2 访问属性
访问属性有多种途径。
#### 6.2.1 访问、赋值 property
| 函数名称                             | 动作  | 注释                        |
| -------------------------------- | --- | ------------------------- |
| **Object_GetPropertyByName_XXX** | 取值  | 获取整数类型属性，返回 `int` 值到 ret 对象中。 |
| **Object_SetPropertyByName_XXX** | 赋值  | 设置浮点数类型属性，`double` 值。      |

Object_GetPropertyByName_XXX 中 xxx 可以为 Int、Double 等，表示将会获取得到一个 int、double 等的基本类型数据返回值。如果返回值是一个类型的对象，请使用 Ref。

Object_SetPropertyByName_XXX 中 xxx 可以为 Int、Double 等，表示将会给一个 int、double 等的基本类型属性赋值。如果属性的值应该是一个类型的对象，请使用 Ref。
```ts
interface Person {
    name: string;
    age: int;
}

class PersonInner implements Person{
    name: string = "";
    age: int = 2;
}
```

```cpp
std::string name = "Goose";
ani_string name_string{};
env->String_NewUTF8(name.c_str(), name.size(), &name_string);
ani_int age_value(42);

env->Object_SetPropertyByName_Int(person_obj, "age", age_value);
env->Object_SetPropertyByName_Ref(person_obj, "name", name_string);

ani_int age_value_ret;
ani_ref name_string_ret;
env->Object_GetPropertyByName_Int(person_obj, "age", &age_value_ret);
env->Object_GetPropertyByName_Ref(person_obj, "name", &name_string_ret);
```

多种示例参考：
[property的多种示例](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_property/ani_property.cpp)

native 无法声明在 get/set 前，所以不能将 property 声明为 native 方法，但存在替代方法，见《1.1.5 节函数绑定的特殊场景》
#### 6.2.2 访问、赋值 field

| 函数名称                        | 动作  | 注释                        |
| --------------------------- | --- | ------------------------- |
| `Object_GetFieldByName_XXX` | 取值  | 获取整数类型属性，返回 `int` 值到 ret 对象中。 |
| `Object_SetFieldByName_XXX` | 赋值  | 设置浮点数类型属性，`double` 值。      |

Object_GetFieldByName_XXX 中 xxx 可以为 Int、Double 等，表示将会获取得到一个 int、double 等的基本类型数据返回值。如果返回值是一个类型的对象，请使用 Ref。

Object_SetFieldByName_XXX 中 xxx 可以为 Int、Double 等，表示将会给一个 int、double 等的基本类型属性赋值。如果属性的值应该是一个类型的对象，请使用 Ref。
```ts
class Person{ // 没有实现自interface所以name和age并非property而是field
    name: string = "";
    age: int = 2;
    // 
    thisIsField: int = 3;
}
```

```cpp
std::string name = "Goose";
ani_string name_string{};
env->String_NewUTF8(name.c_str(), name.size(), &name_string);
ani_int age_value(42);

env->Object_SetFieldByName_Int(person_obj, "age", age_value);
env->Object_SetFieldByName_Ref(person_obj, "name", name_string);

ani_int age_value_ret;
ani_ref name_string_ret;
env->Object_GetFieldByName_Int(person_obj, "age", &age_value_ret);
env->Object_GetFieldByName_Ref(person_obj, "name", &name_string_ret);
```

#### 6.2.3 访问、赋值 static field

| 函数名称                             | 动作  | 注释                            |
| -------------------------------- | --- | ----------------------------- |
| `Class_GetStaticFieldByName_XXX` | 取值  | 获取整数类型属性，返回 `int` 值到 ret 对象中。 |
| `Class_SetStaticFieldByName_XXX` | 赋值  | 设置浮点数类型属性，`double` 值。         |

Class_GetStaticFieldByName_XXX 中 xxx 可以为 Int、Double 等，表示将会获取得到一个 int、double 等的基本类型数据返回值。如果返回值是一个类型的对象，请使用 Ref。

Class_SetStaticFieldByName_XXX 中 xxx 可以为 Int、Double 等，表示将会给一个 int、double 等的基本类型属性赋值。如果属性的值应该是一个类型的对象，请使用 Ref。

### 6.3 调用方法
方法 1：Object_CallMethod_XXX

其中 xxx 可以为 Int、Double 等，表示将会获取得到一个 int、double 等的基本类型数据返回值。如果返回值是一个类型的对象，请使用 Ref。
```ts
class Foo {
   
    native NativeFunc():void;
    ManagedFunc():void {
        console.println("Print in ManagedFunc");
    }
}
```

```cpp
static void NativeFunc(ani_env *env, ani_object obj)
{
    ani_method managedMethod;
    ani_type result;
    
    static const char *className = "Lani_call_method/Foo;";
    ani_class cls;
    if (ANI_OK != env->FindClass(className, &cls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return ;
    }
    if(ANI_OK != env->Class_FindMethod(cls, "ManagedFunc", ":V", &managedMethod)){
        std::cerr << "Class_FindMethod Faild" << std::endl;
        return ;
    }
    std::cout << "Print in Native Func" << std::endl;
    env->Object_CallMethod_Void(obj, managedMethod);
}
```

示例跳转： [ani_call_method/ani_call_method.cpp](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_call_method/ani_call_method.cpp)

### 6.4 可选方法转可选参数
ArkTS1.2 不支持可选方法，使用可选参数进行替代。
```ts
class OptionalClass {
    // fn是函数对象，ani中是ani_fn_obj
    fn?:(a:double, b:double)=>double = (a:double, b:double)=>{return a + b}
}
console.log(CallFn(1.0, 2.0));
let obj = new OptionalClass();
console.log(obj.fn(1.0, 2.0));
```

```cpp
ani_double CallFn(ani_env *env, ani_double val_double1, ani_double val_double2)
{
    ani_object classObj = {};
    static const char *className = "Lani_fn_object/OptionalClass;";
    ani_class cls;
    if (ANI_OK != env->FindClass(className, &cls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
    }

    ani_method ctor;
    if (ANI_OK != env->Class_FindMethod(cls, "<ctor>", nullptr, &ctor)){
        std::cerr << "get ctor Failed'" << className << "'" << std::endl;
    }

    //创建一个实例
    if (ANI_OK != env->Object_New(cls, ctor, &classObj)){
        std::cerr << "Create Object Failed'" << className << "'" << std::endl;
    }

    ani_ref fn_ref;
    env->Object_GetFieldByName_Ref(classObj, "fn", &fn_ref);

    // 构造入参数组
    std::vector<ani_ref> vec;
    // 基本类型转换为ani_ref需要装箱。
    auto val1 = createDouble(env, val_double1);
    auto val2 = createDouble(env, val_double2);
    vec.push_back(val1);
    vec.push_back(val2);

    ani_ref fnReturnVal;
    env->FunctionalObject_Call(static_cast<ani_fn_object>(fn_ref), vec.size(), vec.data(), &fnReturnVal);

    // 返回值类型如果要求是基本类型如ani_int、ani_double需要解包。
    ani_double sumRs;
    env->Object_CallMethodByName_Double(static_cast<ani_object>(fnReturnVal), "unboxed", ":D", &sumRs);

    return sumRs;
}
```
### 6.5 wrap&unwrap
示例跳转： [ani_wrap_native_ptr/ani_wrap_native_ptr.ets · ironrain/ani_cookbook](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_wrap_native_ptr/ani_wrap_native_ptr.ets)


## 7 回调 Function/Lambda 函数对象

函数对象，回调示例： [ani_fn_object/ani_fn_object.ets · ironrain/ani_cookbook](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_fn_object/ani_fn_object.ets)

```cpp
ani_status FunctionalObject_Call(ani_env *env, ani_fn_object fn, ani_size argc, ani_ref *argv, ani_ref *result)
```
函数要求入参为 ani_ref 数组，则如果入参为 int、double 等基本类型，需要转换其为 Int、Double 等装箱类型的实例，组成数组转换成数组指针。
```
ani_object createDouble(ani_double doubleVal){...} //返回一个Double类型，实现见《4.1.2 装箱》
```

```
std::vecotr<ani_ref> vec;
vec.push_back(createDouble(ani_double(2)));
vec.push_back(createDouble(ani_double(4)));
ani_ref* result;
env->FunctionalObject_Call(show_every, ani_size(2), vec.data(),  result);
```

## 8 异步

异步示例： [ani_async_wrapper/ani_async_wrapper.ets · ironrain/ani_cookbook](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_async_wrapper/ani_async_wrapper.ets)

> [! Warning] **当前可能存在问题**
> 如果在异步中抛出异常可能导致程序卡死，无法正常结束。如果配置了执行超时限制，到时将会自行 crash。
> 请尝试在 then 后添加 catch 然后 reject 对象，看是否能够解决。
> 如果无法解决，请查看 wiki 中的沟通地图，联系运行时解决。

## 9 生命周期管理
所有在 CPP native 侧创建的对象，在脱离其作用域之后不再可用。即使存入其他全局 CPP对象也会被回收。必须通过 GlobalReference_Create 等方法进行存储，或者将其与 ets 层的正确声明周期的对象进行绑定。
```cpp
vector<ani_object> vec;
void BoundNativeFunc(ani_env* env, ani_object param) // param也是创建的对象。
{
ani_object newObj;
Object_New(cls, ctor, newObj, ...);
vec.push_back(newObj); // 这个动作是UB
}
```

### 9.1 存储临时对象 Reference
CPP 中的临时对象：
1. cpp 层的 native 函数的入参
2. ANI 函数创建的相关对象，包括但不限于 Object_New、CallMethod_Ref、Array_New_XXX、String_NewXXX 等行为返回的对象。

对于这些临时对象，如果希望在其作用域外继续使用，可以通过GlobalReference_Create 存储之后，用其返回的 ref 视作原对象的真实地址使用。在所有的使用结束后必须进行销毁，否则导致内存泄漏。

- **Reference 可以使用 GlobalReference_Create 对于 native 函数的入参进行创建缓存。**

**cpp 层的 native 函数的所有入参都是固定间隔进行申请的固定地址的多级指针。任何情况下都不要存储其参数到 native 函数作用域外使用，这都是 UB（未定义行为），会发生返回奇怪的错误码、crash 等问题。**

如果不使用 XXXReference_Create 的形式进行创建，实际保存的是临时多级指针，将会被下一次调用时的对象刷新指针内容（这是UB，不要依赖这个行为，可能会crash），最后一次调用之后，将在随机的时机进行释放。

正确用法：使用一个变量存储 GlobalReference_Create 创建出来的 ani_ref。
```cpp
ani_ref savePtr;
int cnt = 0;
void nativeFunc_RunTwice(ani_env env, ani_object obj, ani_object paramObj) {
    // 普通调用
    env->Object_CallMethodByName_Void(paramObj, "show", nullptr);
	
    // 存储到作用域外
	if(cnt == 0){
		env->GlobalReference_Create(reinterpret_cast<ani_ref>(paramObj), &savePtr);
		cnt += 1;
	}
	// 读取全局变量中存储的结果
	// 第二次执行时，显示结果为第一次执行时保存的入参的show函数。
	env->Object_CallMethodByName_Void(static_cast<ani_object>(savePtr), "show", nullptr);
}
```

错误示例：
```cpp
ani_ref savePtr;
int cnt = 0;
// 绑定到ets的cpp native函数
void nativeFunc_RunTwice(ani_env env, ani_object obj, ani_object paramObj) {
    // 普通调用
    env->Object_CallMethodByName_Void(paramObj, "show", nullptr);

    // 存储到作用域外
	if(cnt == 0){
		savePtr = static_cast<ani_ref>(paramObj);
		cnt += 1;
	}
	// 读取全局变量中存储的结果
	// 第二次执行时，显示结果为第二次执行时的入参的show函数。（不要依赖这个UB行为，可能导致crash）
	env->Object_CallMethodByName_Void(static_cast<ani_object>(savePtr), "show", nullptr);
}
```

GlobalReference_Create 等 Create 出来的 ani_ref 视同原对象。参考 《3.3 节的类型定义》中的类型继承树中的关系图，ani_ref 是基类。

- **创建出来的 Reference 在调用结束之后需要用 GlobalReference_Delete 进行销毁，否则会造成内存泄漏**
```cpp
ani_ref objectRef;
env_->String_NewUTF8("x", 1, reinterpret_cast<ani_string *>(&objectRef));
ani_ref objectGRef;
env_->GlobalReference_Create(objectRef, &objectGRef);

env_->GlobalReference_Delete(objectGRef);
```

### 9.2 VM 与 env 的生命周期
vm 是在 app 创建时元能力生成的，通常与应用的生命周期一致。

vm 中可以有多个 env，一般来说，一个线程对应一个 env。

**当前线程的 env 只能在自己的线程中使用。**

也就是说，如果当前线程结束了，那么 env 就会被自动销毁，其 ani_env* env 会变成悬空指针。

解决方案：

从当前线程的 env 获取 vm。
```cpp
ani_vm *vm = nullptr;
env_->GetVM(&vm);
```

传递 vm，然后从 vm 中获取 env。

在另一个线程中，需要 AttachCurrentThread 获取 env。使用 AttachCurrentThread 结束之后用 DetachCurrentThread 进行销毁。

如果之前已经有函数 AttachCurrentThread 了当前线程，AttachCurrentThread会失败。

此时可以 GetEnv 获取引用返回的 env。

1. vm->AttachCurrentThread
```cpp
// 得到env
ani_env *workerEnv = nullptr;
ani_options aniArgs {0, nullptr};
auto status = vm_->AttachCurrentThread(&aniArgs, ANI_VERSION_1, &workerEnv);
...
// env使用结束后分离线程附加。env失效。
status = vm_->DetachCurrentThread();


// 启动互操作的版本
ani_env *etsEnv {nullptr};// 得到env
ani_option interopEnabled {"--interop=enable", nullptr}; // 如果需要启动互操作
ani_options aniArgs {1, &interopEnabled};
auto status = etsVM->AttachCurrentThread(&aniArgs, ANI_VERSION_1, &etsEnv);
...
// env使用结束后分离线程附加。env失效。
status = vm_->DetachCurrentThread();
```

2. vm->GetEnv
```cpp
ani_env *env;
auto status = vm->GetEnv(ANI_VERSION_1, &env);
```

## 10 多线程
ANI 中暂时没有专用多线程 API，可以自行使用 thread 等标准库进行多线程执行。但是需要注意 **env 无法跨线程使用**，因此捕获 env 的行为将造成空指针异常。

重点参考《 9 节生命周期》。

1. lambda 函数将在其他线程中执行时，定义时应该捕获 vm 而不是 env。参考《9.2 节》



## 11 变长数组 Array\<T\>和T\[\]
**新改动：**

当前 `Array<T>` 被扩展到变长数组范围，可以使用 Array_Set_XXX 系列的。

**建议使用 Array 创建数组**，可以调用相关 Array 函数，如 `push`，`$_set` 等函数

ani_array 指 `Array<T>` 和 `T[]` 两种类型。

ANI 中相关 ani_array、ani_array_xxx 等类型都是指变长数组。
```cpp
// 创建Array<T>
env->Array_New_Ref(stringCls, strings.size(), undefinedRef, &array )

// 遍历赋值
ani_size index = 0;
for(auto string:strings){
    ani_string ani_str;
    env->String_NewUTF8(string.c_str(), string.size(), &ani_str)
    env->Array_Set_Ref(array, index, ani_str)
    index ++;
} 
```

## 12 定长数组 

API 待实现定义
 
## 13 ArrayBuffer
ArrayBuffer 类型支持这两个接口 CreateArrayBuffer、ArrayBuffer_GetInfo。

见示例
```ts
loadLibrary("ani_arraybuffer");

native function handleData(buffer:ArrayBuffer):void

function main(){
    // 写入混合类型数据
    const buffer = new ArrayBuffer(4);
    const uint8View = new Uint8Array(buffer);

    uint8View[0] = 1; // 1*1
    uint8View[1] = 2; // 2*256
    uint8View[2] = 0;

    // 读取数据
    console.log(uint8View);
    handleData(buffer); // 1*1 +2*256 = 513
    console.log("1*1 +2*256 = 513")
}
```

```cpp
// 这个类型不是ani_array
static void handleData(ani_env *env, [[maybe_unused]] ani_object obj, ani_arraybuffer arraybuffer) // native函数
{
    void* resultData;
    ani_size resultSize;
    env->ArrayBuffer_GetInfo(arraybuffer, &resultData, & resultSize);
    // 注意解指针uint32_t*
    std::cout << *static_cast<uint32_t*>(resultData) << std::endl;
}
```
完整示例： [ani_arraybuffer/ani_arraybuffer.cpp · ironrain/ani_cookbook - 码云 - 开源中国](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_arraybuffer/ani_arraybuffer.cpp)


## 14 Enum 枚举
Enum 的声明是一个 class（3/15 之后的版本），可以使用 Enum 系列相关的方法。

| 标识符        | ani 类型           | Mangling 名称          |
| ---------- | --------------- | ------------------- |
| COLOR      | `ani_enum`      | `LmoduleName/COLOR` |
| COLOR.Blue | `ani_enum_item` | `Lstd/core/Object;` |

示例跳转： [ani_enum/ani_enum.cpp · ironrain/ani_cookbook - 码云 - 开源中国](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_enum/ani_enum.cpp)
## 15 Error 异常
### 15.1 抛出异常

异常的基类是 Error，其 Mangling 为 `Lescompat/Error`，如果需要抛出自定义异常，通常自定义的异常需要继承自该基类。

特殊情况：当前系统内隐式声明了 BussinessError，可能会造成无法绑定 native 构造函数的情况。

例：创建 Error 并抛出。

```cpp
ani_class errCls;
// 可以查找更多Error类型
char* className = "Lescompat/Error;";
if (ANI_OK != env->FindClass(className, &errCls)) {
    std::cerr << "Not found '"  << className << std::endl;
    return ANI_ERROR;
}

ani_object errObj;
// 有构造函数重载的情况下，请准确根据需要的函数，传入函数签名的Mangling。
ani_method errCtor;
if (ANI_OK != env->Class_FindMethod(errCls, "<ctor>", "Lstd/core/String;Lescompat/ErrorOptions;:V", &errCtor)) {
    std::cerr << "get errCtor Failed'" << className << "'" << std::endl;
    return ANI_ERROR;
}

std::string name = "This will show message!";
ani_string result_string{};
env->String_NewUTF8(name.c_str(), name.size(), &result_string);

//创建一个Error的实例
if (ANI_OK != env->Object_New(errCls, errCtor, &errObj, result_string)) {
    std::cerr << "Create Object Failed'" << className << "'" << std::endl;
    return ANI_ERROR;
}

// 抛出异常
env->ThrowError(static_cast<ani_error>(errObj));
```

### 15.2 捕获异常
1. 异常存在之后可以在 ets 层使用 try-catch 进行捕获。
```ts
try {
  nativeThrowError();
}
catch (e: Error) {
  console.log(e.message)
}
```
2. 异步时需要使用.catch() 进行捕获

## 16 String 字符串
创建字符串且将 std::string 转换为 ani_string
```cpp
ani_string ANIUtils_StdStringToANIString(ani_env *env, std::string str)
{
    ani_string result_string{};
    env->String_NewUTF8(str.c_str(), str.size(), &result_string);
    return result_string;
}
```
Ani_string 转换为 std::string 
```cpp
std::string ANIUtils_ANIStringToStdString(ani_env *env, ani_string ani_str)
{
    ani_size strSize;
    env->String_GetUTF8Size(ani_str, &strSize);

    std::vector<char> buffer(strSize + 1); // +1 for null terminator
    char *utf8_buffer = buffer.data();

    ani_size bytes_written = 0;
    env->String_GetUTF8(ani_str, utf8_buffer, strSize + 1, &bytes_written);

    utf8_buffer[bytes_written] = '\0';
    std::string content = std::string(utf8_buffer);
    return content;
}
```

## 17 BigInt 大数
```cpp
//ets
class Foo{
    static { loadLibrary("ani_bigint");}
    native testBigInt(num:bigint):void;
} 

function main(){
    const f = new Foo;
    let n : bigint = 11223344n;
    f.testBigInt(n);
}

```
```cpp
#include <ani.h>
#include <array>
#include <iostream>

static void testBigInt([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object, ani_object num)
{
    ani_class bigIntCls;
    const char * className = "Lescompat/BigInt;";
    if (ANI_OK != env->FindClass(className, &bigIntCls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return ;
    }
    ani_method getLongMethod;
    if (ANI_OK != env->Class_FindMethod(bigIntCls, "getLong", ":J", &getLongMethod)){
        std::cerr << "Class_GetMethod Failed '" << className << "'" << std::endl;
        return ;
    }

    ani_long longnum;
    if (ANI_OK != env->Object_CallMethod_Long(num, getLongMethod, &longnum)){
        std::cerr << "Object_CallMethod_Long '" << "getLongMethod" << "'" << std::endl;
        return ;
    }
    std::cout << "num value is : '" << longnum << "'" << std::endl;
    return;
}

ANI_EXPORT ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    ani_env *env;
    if (ANI_OK != vm->GetEnv(ANI_VERSION_1, &env)) {
        std::cerr << "Unsupported ANI_VERSION_1" << std::endl;
        return ANI_ERROR;
    }

    static const char *className = "Lani_bigint/Foo;";
    ani_class cls;
    if (ANI_OK != env->FindClass(className, &cls)) {
        std::cerr << "Not found '" << className << "'" << std::endl;
        return ANI_ERROR;
    }

    std::array methods = {
        ani_native_function {"testBigInt", "Lescompat/BigInt;:V", reinterpret_cast<void *>(testBigInt)},
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

## 18 其他容器类使用方法
1. 通过 FindClass 根据类名获取目标容器类
2. 通过 Class_FindMethod 与 Object_CallMethod 系列函数、或者 Object_CallMethodByName 系列函数来根据 builtin 函数名调用容器类里的 builtin 方法
3. 对出入参进行类型转化，如果是复杂对象需要进一步调用方法提取基本类型

### 18.1 Record: "Lescompat/Record;"
Record 的 builtin 函数列表如下（@标准库提供完整列表）

| 函数名     | Signature                           | 描述               |
| ------- | ----------------------------------- | ---------------- |
| \<ctor> | Lstd/core/Object;:V                 | 构造函数             |
| $_get   | Lstd/core/Object;:Lstd/core/Object; | 获取 Key 对应的 Value |
| $_set   | Lstd/core/Object;:V                 | 添加 Key-Value 键值对 |
| keys    |                                     |                  |

代码示例
```ts
// .ets
class PersonInfo {
  name: string = ""
  age: number = 0
}
// ...
native callWithRecord(entry: Record<string, PersonInfo>):void;
// ...
```

```cpp
// .cpp
void callWithRecord([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object, ani_object record)
{
    // 通过class name "Lescompat/Record;"找到Record类
    ani_class recordCls;
    const char * recordClassName = "Lescompat/Record;";
    if (ANI_OK != env->FindClass(recordClassName, &recordCls)) {
        std::cerr << "Not found '" << recordClassName << "'" << std::endl;
        return ;
    }

    // 将字符串"Chloe"处理为ani_string
    ani_string ani_name;
    std::string name = "Chloe";
    if (ANI_OK !=env->String_NewUTF8(name.c_str(), name.length(), &ani_name)){
        std::cerr << "String_NewUTF8 Failed '" << "Chloe" << "'" << std::endl;
        return ;
    }

    // 使用Object_CallMethodByName_Ref调用$_get方法，获取Key为"Chloe"时对应的Value: person，函数签名Lstd/core/Object;:Lstd/core/Object;
    ani_ref person;
    if (ANI_OK != env->Object_CallMethodByName_Ref(record, "$_get", nullptr, &person, ani_name)){
        std::cerr << "Object_CallMethodByName_Ref  $_get Faild" << std::endl;
        return ;
    }

    // 根据est代码里的定义，$_get方法获取的Value: person为自定义类型PersonInfo, 需要调用Object_GetFieldByName_Ref获取person的name和age字段
    // 注意person原先为ani_ref，此处强转为ani_object，因为ani_ref为所有ani数据类型的父类，在明确对象类型的情况下可以强转为子类
    ani_ref person_name;
    if (ANI_OK != env->Object_GetFieldByName_Ref(static_cast<ani_object>(person), "name", &person_name)){
        std::cerr << "Object_GetFieldByName_Ref Faild" << std::endl;
        return ;
    }

    ani_double person_age;
    if (ANI_OK != env->Object_GetFieldByName_Double(static_cast<ani_object>(person), "age", &person_age)){
        std::cerr << "Object_GetFieldByName_Ref Faild" << std::endl;
        return ;
    }

    // ...
}
// ...
```
### 18.2 List
### 18.3 Tuple 元组

## 19 错误码分析
```cpp
// ani.h
typedef enum {
    ANI_OK,
    ANI_ERROR,
    ANI_INVALID_ARGS,
    ANI_INVALID_TYPE,
    ANI_INVALID_DESCRIPTOR,
    ANI_INCORRECT_REF,
    ANI_PENDING_ERROR,
    ANI_NOT_FOUND,
    ANI_ALREADY_BINDED,
    ANI_OUT_OF_REF,
    ANI_OUT_OF_MEMORY,
    ANI_OUT_OF_RANGE,
    ANI_BUFFER_TO_SMALL,
    // NOTE: Add necessary status codes
} ani_status;
```

| 枚举值                    | 值   | 说明          | 备注                        |
| ---------------------- | --- | ----------- | ------------------------- |
| ANI_OK                 | 0   | 操作成功        |                           |
| ANI_ERROR              | 1   | 通用错误        | 函数执行失败，需要具体分析             |
| ANI_INVALID_ARGS       | 2   | 无效参数        | 函数入参中存在非法 nullptr         |
| ANI_INVALID_TYPE       | 3   | 无效类型        | 函数实际返回类型与函数后缀不一致          |
| ANI_INVALID_DESCRIPTOR | 4   | 无效描述符       | Mangling 不符合规范            |
| ANI_INCORRECT_REF      | 5   | 引用不正确       |                           |
| ANI_PENDING_ERROR      | 6   | ArkTS 抛出异常  | ETS 层抛出了一个异常，具体见下文        |
| ANI_NOT_FOUND          | 7   | 未找到         | FindXXX 函数未找到目标           |
| ANI_ALREADY_BINDED     | 8   | native 已经绑定 | 重复绑定                      |
| ANI_OUT_OF_REF         | 9   | 引用超出范围      | 调用数组超界                    |
| ANI_OUT_OF_MEMORY      | 10  | 内存不足        |                           |
| ANI_OUT_OF_RANGE       | 11  | 超出范围        |                           |
| ANI_BUFFER_TO_SMALL    | 12  | 缓冲区太小       |                           |
| ANI_INVALID_VERSION    | 13  | 非法版本号       | 常见于创建 VM 相关的地方            |
| ANI_AMBIGUOUS          | 14  | 存在歧义        | 不要使用 nullptr 替代 signature |
### 19.1 错误码 2 ANI_INVALID_ARGS
此错误码表示，你的入参存在非法的 nullptr 参数。
例如：
```cpp
Object_CallMethodByName_Boolean(nullptr, ...)
```
比如将第一个入参配置为 nullptr 就是非法参数。

### 19.2 错误码 6 ANI_PENDING_ERROR
此报错信息表示，现在 ArkTS 运行时抛出了一个异常。
在 native 的 cpp 侧可以用下面的方法捕获到这个异常信息进行解析。
```cpp
#include <sstream>
ani_boolean errorExists;
// ADD_LOG替换成自己的日志函数和宏
ADD_LOG(/* 此处添加日志表示进入到处理流程 */);
env->ExistUnhandledError(&errorExists);

/* 会报错误码6的那行代码 */

std::ostringstream buffer;
std::streambuf *oldStderr = std::cerr.rdbuf(buffer.rdbuf());
ani_status status = env->DescribeError();
std::cerr.rdbuf(oldStderr);
std::string output = buffer.str();
env->ExistUnhandledError(&errorExists);
ADD_LOG(/* 此处添加日志打印 `output` */);
```

### 19.3 错误码 7 ANI_NOT_FOUND
1. 必须保证 .d.ets 文件和 .ets 的声明完全一致，否则在真机中运行永远返回 7。
```ts
// .d.ets
class A {
foo(i:int):void // 两个文件不一致，将返回7
}

// .ets
class A {
foo():void // 两个文件不一致，将返回7
}
```

### 19.4 错误码 14 ANI_AMBIGUOUS
1. 函数存在重载时，使用 nullptr 将会造成歧义，无法确定具体需要的目标函数。
```cpp
// ets
function foo(i:int):void
function foo(d:number):void

// CPP
FindMethod(cls, "foo", nullptr, &method); // ERROR
FindMethod(cls, "foo", "D:V", &method); //   OK
```

注：Record 的 keys 函数当前存在问题，会报告错误码 14，补充 signature ": Lescompat/IterableIterator;"可解决。

## 20 常见问题 FAQ

- BussinesError 无法创建、行为异常

检查是否存在 BussinessError$partial 字样，构建 abc 文件存在冲突，系统已内置 BussinessError，继续声明会导致冲突，请联系前端和标准库解决该问题。

- Object_New 导致程序 crash

常见原因为入参类型与实际所需类型不一致。强烈建议不要使用 nullptr 获取构造函数。因为构造函数的场景是特殊的，例如构造函数的入参为可选参数时是通过重载实现的，这会导致 nullptr 获取到随机的构造函数。

- 声明文件必须与实现文件的定义一致
否则可能导致无法找到实现所声明的字段。
```ts
// .d.ets
interface Point{
x?:number
}

// .ets
interface Point {
x:number // 如果这样实现，可能导致无法找到number的值，此时他既不是field也不是property。
}
```

- 声明文件必须与实现文件的定义一致

否则可能导致索引错误的属性、函数等

```ts
// .d.ets
interface Point{
x:number
}

// .ets
class Point {
x:number // 如果这样实现，那么访问x时，会提示get/set x不存在。
}
```

- ani_boolean 无法输出

由于 ani_boolean 是 uint 8_t 而不是 bool，所以无法使用 cout 等 cpp 流进行直接输出，可以强制转换为 int 再进行输出。

- 怎么将 ani_int/ani_double 等对象转换成 ani_ref？

将这些类型进行装箱，得到 ani_object，参考 《3.3 节类型转换》，可以将 ani_object 转换为 ani_ref。常见于需要使用 FunctionObject_Call 的场景，其默认参数数组的元素必须为 ani_ref 类型。

- 如何注册析构函数，在ets侧对象析构时调用native的析构释放资源？

当前不支持自动析构，只能在ets侧手动调用gc触发。
示例跳转：

[ani_wrap_native_ptr/ani_wrap_native_ptr.cpp · ironrain/ani_cookbook - 码云 - 开源中国](https://gitee.com/ironrain/ani_cookbook/blob/master/ani_wrap_native_ptr/ani_wrap_native_ptr.cpp)


- 在ets传入两个不同的function，native获取到的object地址一样？

cpp层绑定到ets层的native标识函数的时候其入参在cpp侧的地址都是被自动管理的，会申请一个地址，随后每次调用都是被复用的。可以理解为绑定到native表示的ets函数对应的CPP函数，他的入参都是固定地址的多级指针，保存这个多级指针并不能保证指向的底层地址不会发生变化。所以他们的地址是相同的，每个入参都是按固定间隔进行申请。

- 同一个native函数，通过ets调用两次，函数的入参为function，怎么在native判断这两个function是否相同？

需要使用GlobalReference_Create创建一个持有状态的ref引用，而不是函数入参的地址。
```ts
ani_ref savePtr = nullptr;
void SubscribeState(ani_env *env, [[maybe_unused]] ani_object object, ani_string type, ani_object callback)
{
    env->GlobalReference_Create(reinterpret_cast<ani_ref>(callback), &savePtr);
}
```
这样savePtr保存下来的就是一个可用的ani_ref对象，ani_ref可以Reference_StrictEquals直接进行对比。也可以强制类型转换转换为ani_object执行ANI函数调用。

- 获取两个内容相同的对象的ref，调用接口判断是否相等，调用Reference_StrictEquals期望返回 false，但是实际返回true，是否正确？

Reference_StrictEquals对于String，Number这些obj会基于内容进行比较，例如：
```ts
ets侧
function GetObject(): Object {
    return new String("Hello World!");
}
cpp侧
auto objectRef1 = CallEtsFunction<ani_ref>("GetObject");
auto objectRef2 = CallEtsFunction<ani_ref>("GetObject");
ani_boolean isEquals;
ASSERT_EQ(env_->Reference_StrictEquals(objectRef1, objectRef2, &isEquals), ANI_OK);
ASSERT_EQ(isEquals, ANI_TRUE);
```
若是用户自定义的class，则会基于地址比较。例如：
```ts
ani_wref wref;
ASSERT_EQ(env_->c_api->WeakReference_Create(env_, refa, &wref), ANI_OK);

ani_wref wrefere;
ASSERT_EQ(env_->c_api->WeakReference_Create(env_, refb, &wrefere), ANI_OK);

ani_boolean wasReleased;
ani_ref ref;
ASSERT_EQ(env_->c_api->WeakReference_GetReference(env_, wref, &wasReleased, &ref), ANI_OK);

ani_ref refere;
ASSERT_EQ(env_->c_api->WeakReference_GetReference(env_, wrefere, &wasReleased, &refere), ANI_OK);

ani_boolean isEquals;

ASSERT_EQ(env_->c_api->Reference_StrictEquals(env_, ref, refere, &isEquals), ANI_OK);
ASSERT_EQ(isEquals, ANI_FALSE);
```

- String_GetUTF8SubString 获取字符串，当截取长度不足一个编码后的字符时，是否会截断输出？
String_GetUTF8SubString 边界值测试，测试步骤如下：
1.用字符串"example，世界"，创建UTF-8字符串 String_NewUTF8
2.偏移量为9，取子字符串大小为5的字符串 String_GetUTF8SubString
3.取到子字符串大小的数据
实际结果：取出的字符串大小为3，是字符"世"字的UTF-8格式

这是正常情况，UTF8字符占3个字节。这是在获取UTF8编码字符串，因此当截取长度不足一个编码后的字符时，截断输出。

- 调用env->Object_New，报错返回ANI_INVALID_TYPE。

在ets中加自己的参数为long的构造函数，Object_New时将第四个参数（生成的对象）转为ani_long类型。参考如下示例：
```ts
ani_object context_object;
if(ANI_OK !=env->Object_New(cls, ctor, &context_object, reinterpret_cast<ani_long>(nativeContext)))
{   
    std::cerr << "New Context Fail" << std::endl;
}
```

- 接口返回值为 `Promise<void>`的情况下，接口参数resolve和reject应该怎么写

暂时不能用promise void，尝试换成有返回值的写法。

- 如何在手机上执行编译好的.abc，和怎样测试API接口

需要构建目标平台的运行时产物ark，然后使用ark执行，细节见见GN构建文档。

`export LD_LIBRARY_PATH=.; /path/to/runtime_core/static_core/build/bin/ark  --boot-panda-files=/path/to/runtime_core/static_core/build/plugins/ets/etsstdlib.abc --load-runtimes=ets run.abc run.ETSGLOBAL::main`

- 构造函数不支持可选参数吗？

可以参数在构造函数中是以重载函数的形式实现的，需要绑定多个native构造函数。

- ets有namespace，可选参数无法获取

Namespace_BindNativeFunctions绑定时不需要第二个参数指向类自身。

`static ani_object springMotion([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object, ani_object response, ani_object dampingFraction, ani_object overlapDuration)`

修改成

`static ani_object springMotion([[maybe_unused]] ani_env *env, ani_object response, ani_object dampingFraction, ani_object overlapDuration)`

- namespace里有联合类型时，ets层给对的类型，cpp层获取联合类型内容时会有问题。

Namespace_BindNativeFunctions绑定时不需要第二个参数指向类自身。

`static void handleData_union(ani_env *env, ani_object obj, ani_string type, ani_object union_obj)`

修改成

`static void handleData_union(ani_env *env, ani_string type, ani_object union_obj)`

- 声明了入参，确实传入了实参，为什么 cpp 层获取到的对象是 nullptr

参考《1.1 Native 函数绑定》是否存在绑定错误的问题。

- 调用 Record 的 keys 函数，报告错误码 14

Record 的 keys 函数当前存在问题，会报告错误码 14，补充 signature ": Lescompat/IterableIterator;"可解决。


## 21 ANI 性能分析
待补充

## 22 更新日志
- **Array 适配变长数组**

推荐统一使用 Array 作为变长数组。（PR 已合入 [Implement Array mirror class and use in ANI API · Pull Request !3789 · OpenHarmony/arkcompiler_runtime_core - Gitee.com](https://gitee.com/openharmony/arkcompiler_runtime_core/pulls/3789)）

```ts
// beforce change
const fixedArray = Array<string>()
const array = string[]
// after change
const fixedArray = FixedArray<string>() // 暂未合入，标准库与API规划设计中。
const array = Array<string>()
```

- **ETSGLOBAL 绑定移除**

 ETSGLOBAL 绑定后续将会和 module 相关函数绑定合并。
 
 因此也无法使用 Class_BindNativeMethods 的方法绑定 module 中的函数，需要使用 FindModule 找到原 ETSGLOBAL 再用 Module_BindNativeFunctions 绑定。
 
 **切勿使用 Class_BindNativeMethods 绑定 ETSGLOBAL 下的 native 方法，之后会存在不兼容问题！！！**
```ts
// ets 示例代码
native function newArray():string[] // 声明在top-level，不在namespace或class中
```
整改前：
```cpp
static ani_ref newArray(ani_env *env, ani_object object) {...}
static const char *className = "Lani_array/ETSGLOBAL;";

ani_class cls;
env->FindClass(className, &cls);
...
env->Class_BindNativeMethods(cls, methods.data(), methods.size());
```
整改后
```cpp
static ani_ref newArray(ani_env *env) {...}

static const char *moduleName = "Lani_array;";
ani_module module;
env->FindModule(moduleName, &module);
...
env->Module_BindNativeFunctions(module, methods.data(), methods.size());
```

- **禁止使用 FindClass 进行搜索 namespace、module，反之亦然**

注意：所有 namespace 都必须使用 namespace 相关方法进行搜索，切勿使用 FindClass 相关函数进行搜索 namespace，存在不兼容问题！！！

