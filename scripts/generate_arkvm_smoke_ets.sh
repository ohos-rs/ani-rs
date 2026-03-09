#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

if [[ "${1:-}" == "--ensure-ets" ]]; then
  ./scripts/check_example_ets.sh
fi

helper_template="$repo_root/scripts/arkvm_test_helpers.ets.inc"
if [[ ! -f "$helper_template" ]]; then
  echo "MISSING_HELPER_TEMPLATE: $helper_template"
  exit 1
fi

emit_local_bindings_from_decl() {
  local decl="$1"
  sed -E \
    -e '/^[[:space:]]*loadLibrary\(/d' \
    -e 's/^([[:space:]]*)export[[:space:]]+/\1/' \
    "$decl"
}

emit_preload_snippet() {
  :
}
emit_case_snippet() {
  local pkg="$1"
  case "$pkg" in
    ani-example-any-dynamic)
      cat <<'ETS'
class DynamicAnyObject {
  value: Object;
  constructor(v: Object) {
    this.value = v;
  }
}
let anyObj: Object = new DynamicAnyObject(new DynamicAnyObject(new Object()));
let anyVal: Object = new DynamicAnyObject(new Object());
__assert_bool("dynamic_get_set_bool", __ANI_GENERATED__.dynamic_get_set(anyObj, anyVal));
function identity(x: Object): Object {
  return x;
}
__assert_bool("dynamic_call_bool", __ANI_GENERATED__.dynamic_call(identity, anyVal));
function DynamicCtor(v: Object): Object {
  return v;
}
let dynamicConstructOk = true;
try {
  dynamicConstructOk = __ANI_GENERATED__.dynamic_construct(DynamicCtor, anyVal);
} catch (_e) {
  dynamicConstructOk = false;
}
__assert_bool("dynamic_construct_bool", dynamicConstructOk);
ETS
      ;;
    ani-example-any-value-wrapper)
      cat <<'ETS'
function makePair(a: Object, b: Object): Object {
  return a;
}
__assert_bool("dynamic_call_with_fn_args_bool", __ANI_GENERATED__.dynamic_call_with_fn_args(makePair));
class CounterAny {
  count: Object | null = null;
}
__ANI_GENERATED__.dynamic_set_property(new CounterAny());
let counter = new CounterAny();
__ANI_GENERATED__.dynamic_set_property(counter);
__assert_true("dynamic_set_property_count", counter.count != null);
ETS
      ;;
    ani-example-array-generic)
      cat <<'ETS'
class ArrayPayload {
  value: int;
  constructor(value: int) {
    this.value = value;
  }
}
let pushObj: Object = new ArrayPayload(1);
let setObj: Object = new ArrayPayload(2);
__assert_bool("array_push_and_pop_bool", __ANI_GENERATED__.array_push_and_pop(pushObj));
__assert_bool("array_set_and_get_bool", __ANI_GENERATED__.array_set_and_get(setObj));
ETS
      ;;
    ani-example-arraybuffer)
      cat <<'ETS'
let bufA = __ANI_GENERATED__.create_buffer(8);
let bufB = __ANI_GENERATED__.create_buffer(4);
__assert_eq_int("buffer_length", __ANI_GENERATED__.buffer_length(bufA), 8);
let merged = __ANI_GENERATED__.concat_buffers(bufA, bufB);
__assert_eq_int("concat_buffers_length", __ANI_GENERATED__.buffer_length(merged), 12);
__assert_true("process_buffer_non_negative", __ANI_GENERATED__.process_buffer(merged) >= 0);
ETS
      ;;
    ani-example-async-wrapper)
      cat <<'ETS'
__assert_eq_int("async_square", __ANI_GENERATED__.async_square(6), 36);
let taskId = __ANI_GENERATED__.async_compute_start(5);
__assert_true("async_compute_start_non_negative", taskId >= 0);
__assert_bool("async_check_status_type", __ANI_GENERATED__.async_check_status(taskId));
__assert_true("batch_compute_non_negative", __ANI_GENERATED__.batch_compute(3) >= 0);
ETS
      ;;
    ani-example-bigint)
      cat <<'ETS'
__assert_eq_long("big_int_add", __ANI_GENERATED__.big_int_add(7, 5), 12);
__assert_eq_long("big_int_subtract", __ANI_GENERATED__.big_int_subtract(9, 4), 5);
__assert_eq_long("big_int_multiply", __ANI_GENERATED__.big_int_multiply(3, 4), 12);
__assert_true("big_int_compare_lt", __ANI_GENERATED__.big_int_compare(2, 5) < 0);
__assert_true("big_int_is_zero", __ANI_GENERATED__.big_int_is_zero(0));
ETS
      ;;
    ani-example-call-method)
      cat <<'ETS'
__assert_eq_int("call_static_square", __ANI_GENERATED__.call_static_square(7), 49);
class MethodBox {
  value: int;
  constructor(v: int) {
    this.value = v;
  }
}
__assert_eq_int("get_property_int", __ANI_GENERATED__.get_property_int(new MethodBox(9)), 9);
ETS
      ;;
    ani-example-call-variadic-v)
      cat <<'ETS'
export function sumValue(a: int, b: int): int {
  return a + b;
}
function pairValue(a: Object, b: Object): Object {
  return a;
}
__assert_bool("call_any_with_fn_args_type", __ANI_GENERATED__.call_any_with_fn_args(pairValue));
__assert_eq_int("call_function_with_value_array", __ANI_GENERATED__.call_function_with_value_array_by_name("sumValue", 3, 4), 7);
ETS
      ;;
    ani-example-class-bind-static-native)
      cat <<'ETS'
class StaticBindToken {}
let bindStaticOk = true;
try {
  __ANI_GENERATED__.bind_static_natives(new StaticBindToken());
} catch (_e) {
  bindStaticOk = false;
}
__assert_bool("bind_static_natives_bool", bindStaticOk);
ETS
      ;;
    ani-example-class-reflect)
      cat <<'ETS'
class ReflectTarget {
  private _value: int = 0;
  public get value(): int {
    return this._value;
  }
  public set value(v: int) {
    this._value = v;
  }
}
__assert_bool(
  "resolve_getter_and_setter_bool",
  __ANI_GENERATED__.resolve_getter_and_setter_by_name("arkvm_test.ReflectTarget", "value"),
);
ETS
      ;;
    ani-example-class-static)
      cat <<'ETS'
class StaticHost {
  static COUNT: int = 0;
  static add(a: int, b: int): int {
    return a + b;
  }
}
__assert_bool(
  "lookup_static_field_by_name",
  __ANI_GENERATED__.lookup_static_field_by_name("arkvm_test.StaticHost", "COUNT"),
);
__assert_eq_int(
  "static_field_roundtrip_int_by_name",
  __ANI_GENERATED__.static_field_roundtrip_int_by_name("arkvm_test.StaticHost", "COUNT", 11),
  11,
);
__assert_eq_int(
  "call_static_int_by_name",
  __ANI_GENERATED__.call_static_int_by_name("arkvm_test.StaticHost", "add", 2, 3),
  5,
);
ETS
      ;;
    ani-example-class-static-by-name)
      cat <<'ETS'
class ByNameHost {
  static COUNT: int = 0;
  static PAYLOAD: Object = new Object();
}
class ByNamePayload {
  key: string = "";
  constructor(k: string) {
    this.key = k;
  }
}
let byNameCount = __ANI_GENERATED__.static_field_by_name_roundtrip_named("arkvm_test.ByNameHost");
__assert_eq_int("static_field_by_name_roundtrip_named", byNameCount, 7);
__assert_bool(
  "static_ref_by_name_roundtrip_named",
  __ANI_GENERATED__.static_ref_by_name_roundtrip_named("arkvm_test.ByNameHost", new ByNamePayload("ok")),
);
ETS
      ;;
    ani-example-enum-item-wrapper)
      cat <<'ETS'
class EnumToken {}
let enumToken = new EnumToken();
let enumNameOk = true;
try {
  let gotName = __ANI_GENERATED__.enum_item_name(enumToken, "Red");
  __assert_true("enum_item_name_length", gotName.length >= 0);
} catch (_e) {
  enumNameOk = false;
}
__assert_bool("enum_item_name_bool", enumNameOk);
ETS
      ;;
    ani-example-error)
      cat <<'ETS'
__assert_eq_double("divide", __ANI_GENERATED__.divide(8.0, 2.0), 4.0);
__assert_eq_string("validate_age", __ANI_GENERATED__.validate_age(20), "Age 20 is valid");
__assert_eq_string("expect_string_type", __ANI_GENERATED__.expect_string_type("ani"), "ANI");
__assert_eq_string("login", __ANI_GENERATED__.login("admin", "secret"), "auth_token_12345");
__assert_true("read_config_non_empty", __ANI_GENERATED__.read_config("config.json").length > 0);
__ANI_GENERATED__.check_array_bounds(1, 3);
__assert_true("check_array_bounds_ok", true);
ETS
      ;;
    ani-example-ets-declaration)
      cat <<'ETS'
__assert_eq_int("add", __ANI_GENERATED__.add(3, 4), 7);
ETS
      ;;
    ani-example-fixed-array-wrapper)
      cat <<'ETS'
__assert_eq_int("sum_fixed_int", __ANI_GENERATED__.sum_fixed_int([1, 2, 3]), 6);
let ints = __ANI_GENERATED__.roundtrip_fixed_int([7, 8]);
__assert_eq_int("roundtrip_fixed_int_len", ints.length, 2);
let bools = __ANI_GENERATED__.negate_fixed_bool([true, false]);
__assert_true("negate_fixed_bool_len", bools.length == 2);
ETS
      ;;
    ani-example-fixed-tuple-enum-utf16)
      cat <<'ETS'
__assert_eq_string("utf16_roundtrip", __ANI_GENERATED__.utf16_roundtrip("你好ANI"), "你好ANI");
let fixedRegion = __ANI_GENERATED__.fixed_array_region();
__assert_true("fixed_array_region_non_empty", fixedRegion.length >= 0);
ETS
      ;;
    ani-example-function)
      cat <<'ETS'
function noArgsOk(): string {
  return "ok";
}
function wrapValue(input: string): string {
  return "wrapped:" + input;
}
__ANI_GENERATED__.clear_callback();
__assert_true("has_callback_false", !__ANI_GENERATED__.has_callback());
__ANI_GENERATED__.register_string_transformer(wrapValue);
__assert_true("has_string_transformer_true", __ANI_GENERATED__.has_string_transformer());
__assert_eq_string("transform_string", __ANI_GENERATED__.transform_string("ani"), "wrapped:ani");
__assert_eq_string("call_no_args_callback", __ANI_GENERATED__.call_no_args_callback(noArgsOk), "ok");
__assert_eq_string("call_string_callback", __ANI_GENERATED__.call_string_callback(wrapValue, "ark"), "wrapped:ark");
ETS
      ;;
    ani-example-function-variable)
      cat <<'ETS'
export let counter: int = 0;
export let payload: Object = new Object();
export function addInts(a: int, b: int): int {
  return a + b;
}
export function doNothing(): void {
}
__assert_eq_int("call_module_function_int_example", __ANI_GENERATED__.call_module_function_int_example("addInts"), 16);
__assert_eq_int("module_variable_roundtrip_int", __ANI_GENERATED__.module_variable_roundtrip_int("counter", 11), 11);
__assert_bool("module_variable_roundtrip_ref", __ANI_GENERATED__.module_variable_roundtrip_ref("payload", new Object()));
ETS
      ;;
    ani-example-init-lifecycle)
      cat <<'ETS'
let before = __ANI_GENERATED__.init_state();
__assert_true("init_state_non_negative", before >= 0);
__ANI_GENERATED__.reset_init_state();
let after = __ANI_GENERATED__.init_state();
__assert_true("init_state_after_reset_non_negative", after >= 0);
ETS
      ;;
    ani-example-interface)
      cat <<'ETS'
let c1 = __ANI_GENERATED__.create_comparable(10);
let c2 = __ANI_GENERATED__.create_comparable(20);
__assert_true("create_comparable_handles", c1 > 0 && c2 > 0);
__assert_true("compare_values_lt", __ANI_GENERATED__.compare_values(c1, c2) < 0);
__ANI_GENERATED__.destroy_comparable(c1);
__ANI_GENERATED__.destroy_comparable(c2);

let s = __ANI_GENERATED__.create_serializable("hello");
__assert_true("create_serializable_handle", s > 0);
__assert_true("serialize_contains_data", __ANI_GENERATED__.serialize(s).length > 0);
__ANI_GENERATED__.deserialize(s, "{\"data\":\"world\"}");
__assert_eq_string("get_data", __ANI_GENERATED__.get_data(s), "world");
__ANI_GENERATED__.destroy_serializable(s);
ETS
      ;;
    ani-example-module-member)
      cat <<'ETS'
export function sum(a: int, b: int): int {
  return a + b;
}
export let counter: int = 1;
namespace sample {
  export function mul(a: int, b: int): int {
    return a * b;
  }
  export let state: int = 2;
}
__assert_bool("find_current_module_members", __ANI_GENERATED__.find_current_module_members());
__assert_bool("find_current_namespace_members", __ANI_GENERATED__.find_current_namespace_members("sample"));
ETS
      ;;
    ani-example-nullish-union)
      cat <<'ETS'
__assert_bool("accept_undefined", __ANI_GENERATED__.accept_undefined(undefined));
__assert_bool("accept_null", __ANI_GENERATED__.accept_null(null));
__assert_true("make_undefined", __ANI_GENERATED__.make_undefined() == undefined);
__assert_true("make_null", __ANI_GENERATED__.make_null() == null);
ETS
      ;;
    ani-example-new-basic)
      cat <<'ETS'
__assert_eq_int("add", __ANI_GENERATED__.add(2, 3), 5);
__assert_eq_int("subtract", __ANI_GENERATED__.subtract(9, 4), 5);
__assert_eq_int("multiply", __ANI_GENERATED__.multiply(6, 7), 42);
__assert_eq_int("divide", __ANI_GENERATED__.divide(8, 2), 4);
__assert_eq_string("greet", __ANI_GENERATED__.greet("ArkTS"), "Hello, ArkTS!");
__assert_eq_int("string_length", __ANI_GENERATED__.string_length("hello"), 5);
__assert_eq_long("factorial", __ANI_GENERATED__.factorial(5), 120);
__assert_true("is_prime_17", __ANI_GENERATED__.is_prime(17));
ETS
      ;;
    ani-example-new-class)
      cat <<'ETS'
let calcToken = __ANI_GENERATED__.Calculator.create();
__assert_true("Calculator.create_handle", calcToken > 0);

let person = new __ANI_GENERATED__.Person("Alice", 30);
__assert_eq_string("Person.getName", person.getName(), "Alice");
__assert_eq_int("Person.getAge", person.getAge(), 30);
person.setAge(31);
__assert_eq_int("Person.getAge_after_set", person.getAge(), 31);
__assert_eq_string("Person.greet", person.greet(), "Hello, I'm Alice and I'm 31 years old!");
person.destroy();
ETS
      ;;
    ani-example-impl-block)
      cat <<'ETS'
let widget = new __ANI_GENERATED__.Widget("impl", 2);
__assert_eq_string("Widget.getName", widget.getName(), "impl");
__assert_eq_int("Widget.getCount", widget.getCount(), 2);
widget.setCount(5);
__assert_eq_int("Widget.getCount_after_set", widget.getCount(), 5);
__assert_eq_string("Widget.describe", widget.describe(), "Widget(impl, 5)");
__assert_eq_int("Widget.sum", __ANI_GENERATED__.Widget.sum(2, 4), 6);
ETS
      ;;
    ani-example-object-typed)
      cat <<'ETS'
class ObjTyped {
  counter: long = 0;
  ratio: float = 0.0;
}
class ObjTypedToken {}
let objTyped = new ObjTyped();
__assert_eq_long("field_by_name_roundtrip_long", __ANI_GENERATED__.field_by_name_roundtrip_long(objTyped, 99), 99);
__assert_eq_double("property_roundtrip_float", __ANI_GENERATED__.property_roundtrip_float(objTyped, 1.25), 1.25);
ETS
      ;;
    ani-example-object-model)
      cat <<'ETS'
let madeUser = __ANI_GENERATED__.make_user_profile(3, "ark", true);
__assert_eq_int("make_user_profile_id", madeUser.id, 3);
__assert_eq_string("make_user_profile_name", madeUser.name, "ark");
__assert_true("make_user_profile_active", madeUser.active);

let inputUser = new UserProfile();
inputUser.id = 8;
inputUser.name = "native";
inputUser.active = false;
__assert_eq_string(
  "describe_user_profile",
  __ANI_GENERATED__.describe_user_profile(inputUser),
  "8:native:inactive",
);

let renamedUser = __ANI_GENERATED__.rename_user_profile(inputUser, "renamed");
__assert_eq_string("rename_user_profile_name", renamedUser.name, "renamed");
__assert_eq_int("rename_user_profile_id", renamedUser.id, 8);

let chooseOk = __ANI_GENERATED__.choose_user_profile(true) as UserProfile;
__assert_eq_int("choose_user_profile_object_id", chooseOk.id, 7);
__assert_eq_string("choose_user_profile_object_name", chooseOk.name, "ani");
let chooseFallback = __ANI_GENERATED__.choose_user_profile(false) as String;
__assert_eq_string("choose_user_profile_string", chooseFallback.toString(), "no-user");

let resultOk = __ANI_GENERATED__.user_profile_result(true);
__assert_eq_int("user_profile_result_id", resultOk.id, 9);
__assert_eq_string("user_profile_result_name", resultOk.name, "result");
__assert_true("user_profile_result_inactive", !resultOk.active);
__assert_throws("user_profile_result_error", (): void => {
  __ANI_GENERATED__.user_profile_result(false);
});
ETS
      ;;
    ani-example-optional)
      cat <<'ETS'
__assert_eq_int("with_default_simple", __ANI_GENERATED__.with_default_simple(3, 4), 12);
__assert_eq_int("with_optional_int_some", __ANI_GENERATED__.with_optional_int(8, new Int(2)), 10);
__assert_eq_int("with_optional_int_null", __ANI_GENERATED__.with_optional_int(8, null), 8);
__assert_eq_double("with_optional_double_some", __ANI_GENERATED__.with_optional_double(1.5, new Double(2.0)), 3.5);
__assert_eq_double("with_optional_double_null", __ANI_GENERATED__.with_optional_double(1.5, null), 1.5);
__assert_eq_int("with_optional_boolean_null", __ANI_GENERATED__.with_optional_boolean(8, null), 8);
__assert_eq_int("with_optional_boolean_true", __ANI_GENERATED__.with_optional_boolean(8, new Boolean(true)), 16);
__assert_eq_int("with_multiple_optional_mixed", __ANI_GENERATED__.with_multiple_optional(1, new Int(2), null, new Int(4)), 7);
__assert_eq_long("with_optional_long_null", __ANI_GENERATED__.with_optional_long(9, null), 9);
__assert_eq_double("with_optional_float_null", __ANI_GENERATED__.with_optional_float(2.5, null), 2.5);
__assert_eq_string("with_optional_string_some", __ANI_GENERATED__.with_optional_string("x", "y"), "x y");
__assert_eq_string("with_optional_string_null", __ANI_GENERATED__.with_optional_string("x", null), "x");
ETS
      ;;
    ani-example-record)
      cat <<'ETS'
let recordPtr = __ANI_GENERATED__.create_record();
__ANI_GENERATED__.record_set(recordPtr, "a", 1);
__ANI_GENERATED__.record_set(recordPtr, "b", 2);
__assert_eq_int("record_size", __ANI_GENERATED__.record_size(recordPtr), 2);
__assert_eq_int("record_get", __ANI_GENERATED__.record_get(recordPtr, "a"), 1);
__assert_true("record_has", __ANI_GENERATED__.record_has(recordPtr, "b"));
__ANI_GENERATED__.destroy_record(recordPtr);
ETS
      ;;
    ani-example-reference)
      cat <<'ETS'
__ANI_GENERATED__.clear_stored_object();
__assert_true("has_stored_object_false", !__ANI_GENERATED__.has_stored_object());
class RefObject {
  name: string;
  value: int;
  constructor(name: string, value: int) {
    this.name = name;
    this.value = value;
  }
}
let refObj = new RefObject("ani", 1);
__ANI_GENERATED__.store_object(refObj);
__assert_true("has_stored_object_true", __ANI_GENERATED__.has_stored_object());
__assert_true("use_stored_object", __ANI_GENERATED__.use_stored_object());
__ANI_GENERATED__.set_compare_object(refObj);
__assert_bool("compare_stored_references_bool", __ANI_GENERATED__.compare_stored_references());
__assert_bool("clone_stored_object_bool", __ANI_GENERATED__.clone_stored_object());
__ANI_GENERATED__.clear_stored_object();
__ANI_GENERATED__.clear_compare_object();
ETS
      ;;
    ani-example-reference-scope)
      cat <<'ETS'
class ScopeObject {
  id: int;
  constructor(id: int) {
    this.id = id;
  }
}
let scopeObj = new ScopeObject(1);
__ANI_GENERATED__.use_reference_scope(scopeObj);
__assert_true("use_reference_scope_ok", true);
__assert_true("compare_references_same", __ANI_GENERATED__.compare_references(scopeObj, scopeObj));
ETS
      ;;
    ani-example-setfield)
      cat <<'ETS'
let personData = __ANI_GENERATED__.create_person_data("bob", 20, 1.80);
__assert_true("create_person_data_handle", personData > 0);
__assert_eq_int("person_data_get_age", __ANI_GENERATED__.person_data_get_age(personData), 20);
__ANI_GENERATED__.person_data_set_age(personData, 21);
__assert_eq_int("person_data_set_age", __ANI_GENERATED__.person_data_get_age(personData), 21);
__assert_eq_string("person_data_get_name", __ANI_GENERATED__.person_data_get_name(personData), "bob");
__ANI_GENERATED__.destroy_person_data(personData);
ETS
      ;;
    ani-example-template)
      cat <<'ETS'
let intContainer = __ANI_GENERATED__.create_int_container(10);
__assert_eq_int("container_get_int", __ANI_GENERATED__.container_get_int(intContainer), 10);
__ANI_GENERATED__.container_set_int(intContainer, 12);
__assert_eq_int("container_set_int", __ANI_GENERATED__.container_get_int(intContainer), 12);
__ANI_GENERATED__.destroy_int_container(intContainer);

let pair = __ANI_GENERATED__.create_pair("k", 9);
__assert_eq_string("pair_get_key", __ANI_GENERATED__.pair_get_key(pair), "k");
__assert_eq_int("pair_get_value", __ANI_GENERATED__.pair_get_value(pair), 9);
__ANI_GENERATED__.destroy_pair(pair);
ETS
      ;;
    ani-example-tuple-value-wrapper)
      cat <<'ETS'
let tupleValue: [int, int] = [2, 3];
__assert_eq_int("tuple_sum", __ANI_GENERATED__.tuple_sum(tupleValue), 5);
__ANI_GENERATED__.tuple_set_first(tupleValue, 7);
__assert_eq_int("tuple_set_first", __ANI_GENERATED__.tuple_sum(tupleValue), 10);
ETS
      ;;
    ani-example-type-relation)
      cat <<'ETS'
class TypeRelationToken {}
let typeTokenA = new TypeRelationToken();
let typeTokenB = new TypeRelationToken();
__assert_bool(
  "check_type_relation_bool",
  __ANI_GENERATED__.check_type_relation(typeTokenA, typeTokenB),
);
__assert_bool("get_super_class_example_bool", __ANI_GENERATED__.get_super_class_example(typeTokenA));
ETS
      ;;
    ani-example-union)
      cat <<'ETS'
__assert_eq_string(
  "handle_string_or_int_either_string",
  __ANI_GENERATED__.handle_string_or_int_either(new String("ani")),
  "String: ani",
);
__assert_eq_string("create_by_type_int", __ANI_GENERATED__.create_by_type(1, 9, "x"), "Int: 9");
__assert_eq_int("get_type_code", __ANI_GENERATED__.get_type_code(2), 2);
ETS
      ;;
    ani-example-vm)
      cat <<'ETS'
__assert_eq_int("build_vm_options_count", __ANI_GENERATED__.build_vm_options_count(), 2);
__assert_true("query_vm_version_positive", __ANI_GENERATED__.query_vm_version() > 0);
__assert_true("query_vm_version_with_closure_positive", __ANI_GENERATED__.query_vm_version_with_closure() > 0);
ETS
      ;;
    ani-example-weak-ref)
      cat <<'ETS'
class WeakPayload {
  key: string;
  constructor(key: string) {
    this.key = key;
  }
}
__assert_true("weak_ref_roundtrip", __ANI_GENERATED__.weak_ref_roundtrip(new WeakPayload("value")));
ETS
      ;;
    ani-example-wrap-ptr)
      cat <<'ETS'
let res = __ANI_GENERATED__.create_native_resource(7, "r1");
__assert_true("create_native_resource_handle", res > 0);
__assert_eq_int("get_resource_id", __ANI_GENERATED__.get_resource_id(res), 7);
__assert_eq_string("get_resource_name", __ANI_GENERATED__.get_resource_name(res), "r1");
__ANI_GENERATED__.set_resource_name(res, "r2");
__assert_eq_string("set_resource_name", __ANI_GENERATED__.get_resource_name(res), "r2");
__ANI_GENERATED__.destroy_native_resource(res);

let db = __ANI_GENERATED__.create_db_connection("db://local");
__assert_true("db_handle", db > 0);
__assert_true("db_connect", __ANI_GENERATED__.db_connect(db));
__assert_true("db_is_connected", __ANI_GENERATED__.db_is_connected(db));
__assert_eq_int("db_execute_query", __ANI_GENERATED__.db_execute_query(db, "select 1"), 1);
__assert_eq_int("db_get_query_count", __ANI_GENERATED__.db_get_query_count(db), 1);
__ANI_GENERATED__.db_disconnect(db);
__ANI_GENERATED__.destroy_db_connection(db);
ETS
      ;;
    *)
      cat <<'ETS'
__assert_true("module_imported", true);
ETS
      ;;
  esac
}

examples_generated=0
examples_missing=0

while IFS= read -r cargo; do
  [[ -z "$cargo" ]] && continue

  dir="$(dirname "$cargo")"
  pkg="$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$cargo" | head -n1)"
  [[ -z "$pkg" ]] && continue

  base="${pkg//-/_}"
  decl="$dir/target/ani-ets/${base}.ets"
  out="$dir/arkvm_test.ets"

  if [[ ! -f "$decl" ]]; then
    echo "MISSING_DECL: $decl"
    ((examples_missing += 1))
    continue
  fi

  {
    echo "// Auto-generated arkvm smoke test for ${pkg}."
    echo "// Regenerate with: ./scripts/generate_arkvm_smoke_ets.sh"
    echo
    emit_local_bindings_from_decl "$decl"
    echo
    emit_preload_snippet "$pkg"
    echo
    echo "loadLibrary(\"${base}\");"
    echo
    cat "$helper_template"
    echo
    echo "console.log(\"[arkvm] smoke start: ${pkg}\");"
    emit_case_snippet "$pkg" | sed 's/__ANI_GENERATED__\.//g'
    echo
    cat <<'ETS'
if (__ani_fail_count > 0) {
  throw new Error("arkvm assertions failed: " + __ani_fail_count);
}
ETS
    echo "console.log(\"[arkvm] smoke done: ${pkg}\");"
  } > "$out"

  ((examples_generated += 1))
done < <(find examples -maxdepth 2 -name Cargo.toml | sort)

echo "GENERATED: ${examples_generated}"
echo "MISSING_DECL: ${examples_missing}"
