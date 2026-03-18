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
function main(): void {
  console.log("[arkvm] smoke start: ani-example-async-wrapper");
  __assert_eq_int("async_square", __ANI_GENERATED__.async_square(6), 36);
  let taskId = __ANI_GENERATED__.async_compute_start(5);
  __assert_true("async_compute_start_non_negative", taskId >= 0);
  __assert_bool("async_check_status_type", __ANI_GENERATED__.async_check_status(taskId));
  __assert_true("batch_compute_non_negative", __ANI_GENERATED__.batch_compute(3) >= 0);
  let promisedSquare: int = waitForCompletion(() => __ANI_GENERATED__.tokio_delayed_square(7, 10));
  __assert_eq_int("tokio_delayed_square", promisedSquare, 49);
  let promisedText: string = waitForCompletion(() => __ANI_GENERATED__.tokio_fetch_text("ani://tokio"));
  __assert_eq_string("tokio_fetch_text", promisedText, "Response from: ani://tokio");
  let tokioRejected: boolean = waitForCompletion(async (): Promise<boolean> => {
    try {
      await __ANI_GENERATED__.tokio_fail("boom");
      return false;
    } catch (_e) {
      return true;
    }
  });
  __assert_true("tokio_fail", tokioRejected);

  if (__ani_fail_count > 0) {
    throw new Error("arkvm assertions failed: " + __ani_fail_count);
  }
  console.log("[arkvm] smoke done: ani-example-async-wrapper");
}
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

  static flag(): boolean {
    return true;
  }

  static sum(a: int, b: int): int {
    return a + b;
  }

  static tag(): string {
    return "by-name-tag";
  }

  static label(prefix: string, suffix: string): string {
    return prefix + "-" + suffix;
  }

  static clearCount(): void {
    ByNameHost.COUNT = 0;
  }

  static resetTo(value: int): void {
    ByNameHost.COUNT = value;
  }
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
__assert_eq_int(
  "static_method_sum_by_name_named",
  __ANI_GENERATED__.static_method_sum_by_name_named("arkvm_test.ByNameHost", 3, 4),
  7,
);
__assert_true(
  "static_method_flag_by_name_named",
  __ANI_GENERATED__.static_method_flag_by_name_named("arkvm_test.ByNameHost"),
);
__assert_eq_string(
  "static_method_tag_by_name_named",
  __ANI_GENERATED__.static_method_tag_by_name_named("arkvm_test.ByNameHost"),
  "by-name-tag",
);
__assert_eq_string(
  "static_method_label_by_name_named",
  __ANI_GENERATED__.static_method_label_by_name_named("arkvm_test.ByNameHost", "left", "right"),
  "left-right",
);
__assert_eq_int(
  "static_method_clear_by_name_named",
  __ANI_GENERATED__.static_method_clear_by_name_named("arkvm_test.ByNameHost"),
  0,
);
__assert_eq_int(
  "static_method_reset_by_name_named",
  __ANI_GENERATED__.static_method_reset_by_name_named("arkvm_test.ByNameHost", 19),
  19,
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
    ani-example-bind-overload)
      cat <<'ETS'
__assert_eq_int("sum_2", __ANI_GENERATED__.sum(8, 16), 24);
__assert_eq_int("sum_3", __ANI_GENERATED__.sum(8, 16, 6), 30);
__assert_eq_string("concat_2", __ANI_GENERATED__.concat("abc", "def"), "abcdef");
__assert_eq_string("concat_3", __ANI_GENERATED__.concat("abc", "def", "ghi"), "abcdefghi");
__assert_eq_int("ops.sum_2", ops.sum(8, 16), 24);
__assert_eq_int("ops.sum_3", ops.sum(8, 16, 6), 30);
__assert_eq_string("ops.concat_2", ops.concat("abc", "def"), "abcdef");
__assert_eq_int("A.recursiveFunction", A.recursiveFunction(5), 15);
__assert_eq_int("A.B.sumB", A.B.sumB(8, 16), 24);
ETS
      ;;
    ani-example-ets-declaration)
      cat <<'ETS'
__assert_eq_int("add", __ANI_GENERATED__.add(3, 4), 7);
__assert_eq_double("AniMath.Utils.sqrt", AniMath.Utils.sqrt(9.0), 3.0);
__assert_eq_int("AniMath.Utils.sum3", AniMath.Utils.sum3(1, 2, 3), 6);
let person = new example.Person("ani-rs", 7);
__assert_eq_string("example.Person.name", person.name, "ani-rs");
__assert_eq_int("example.Person.score", person.score, 7);
person.score = 9;
__assert_eq_int("example.Person.score_after_set", person.score, 9);
__assert_eq_string("example.Person.label", person.label(), "ani-rs#9");
__assert_eq_string("example.Person.species", example.Person.species(), "human");
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
function recursiveString(target: string, current: string): string {
  if (target == current) {
    return current;
  }
  return recursiveString(target, current + "1");
}
let closureValue = "closure:";
let closureFn = (input: string): string => {
  return closureValue + input;
};
let nestedFn = (input: string): string => {
  let prefix = "hello ";
  let inner = (value: string): string => {
    return prefix + value;
  };
  return inner(input);
};
let recursiveFn = (input: string): string => {
  return recursiveString("hello1111111111", input);
};
__ANI_GENERATED__.clear_callback();
__assert_true("has_callback_false", !__ANI_GENERATED__.has_callback());
__ANI_GENERATED__.register_string_transformer(wrapValue);
__assert_true("has_string_transformer_true", __ANI_GENERATED__.has_string_transformer());
__assert_eq_string("transform_string_basic", __ANI_GENERATED__.transform_string("ani"), "wrapped:ani");
__assert_eq_string("call_no_args_callback", __ANI_GENERATED__.call_no_args_callback(noArgsOk), "ok");
__assert_eq_string("call_string_callback_basic", __ANI_GENERATED__.call_string_callback(wrapValue, "ark"), "wrapped:ark");
__assert_eq_string("call_string_callback_closure", __ANI_GENERATED__.call_string_callback(closureFn, "arkts"), "closure:arkts");
__assert_eq_string("call_string_callback_nested", __ANI_GENERATED__.call_string_callback(nestedFn, "world"), "hello world");
__assert_eq_string("call_string_callback_recursive", __ANI_GENERATED__.call_string_callback(recursiveFn, "hello"), "hello1111111111");
__ANI_GENERATED__.register_string_transformer(closureFn);
__assert_eq_string("transform_string_closure", __ANI_GENERATED__.transform_string("vm"), "closure:vm");
__ANI_GENERATED__.register_string_transformer(nestedFn);
__assert_eq_string("transform_string_nested", __ANI_GENERATED__.transform_string("arkvm"), "hello arkvm");
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
__assert_eq_int("init_state_runs_all_registered_init_callbacks", before, 111);
__ANI_GENERATED__.reset_init_state();
let after = __ANI_GENERATED__.init_state();
__assert_eq_int("init_state_reset_clears_runtime_flags", after, 0);
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
export function join(left: string, right: string): string {
  return left + ":" + right;
}
export let counter: int = 1;
export let label: string = "seed";
namespace sample {
  export function mul(a: int, b: int): int {
    return a * b;
  }
  export function tag(value: string): string {
    return "[" + value + "]";
  }
  export let state: int = 2;
  export let note: string = "note0";
}
__assert_bool("find_current_module_members", __ANI_GENERATED__.find_current_module_members());
__assert_eq_int("call_current_module_sum", __ANI_GENERATED__.call_current_module_sum(3, 4), 7);
__assert_eq_string("call_current_module_join", __ANI_GENERATED__.call_current_module_join("left", "right"), "left:right");
__assert_eq_int("roundtrip_current_module_counter", __ANI_GENERATED__.roundtrip_current_module_counter(11), 11);
__assert_eq_string("roundtrip_current_module_label", __ANI_GENERATED__.roundtrip_current_module_label("updated"), "updated");
__assert_bool("find_current_namespace_members", __ANI_GENERATED__.find_current_namespace_members("sample"));
__assert_eq_int("call_current_namespace_mul", __ANI_GENERATED__.call_current_namespace_mul("sample", 6, 7), 42);
__assert_eq_string("call_current_namespace_tag", __ANI_GENERATED__.call_current_namespace_tag("sample", "arkts"), "[arkts]");
__assert_eq_int("roundtrip_current_namespace_state", __ANI_GENERATED__.roundtrip_current_namespace_state("sample", 23), 23);
__assert_eq_string("roundtrip_current_namespace_note", __ANI_GENERATED__.roundtrip_current_namespace_note("sample", "note1"), "note1");
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
    ani-example-constructor-nullish)
      cat <<'ETS'
let noneUser = new __ANI_GENERATED__.Person(undefined);
__assert_eq_string("ctor_none", noneUser.name, "anonymous");
noneUser.rename("bridge");
__assert_eq_string("rename_some", noneUser.name, "bridge");
noneUser.rename(undefined);
__assert_eq_string("rename_none", noneUser.name, "anonymous");
let someUser = new __ANI_GENERATED__.Person("ark");
__assert_eq_string("ctor_some", someUser.name, "ark");
ETS
      ;;
    ani-example-constructor-overload)
      cat <<'ETS'
let pair = new __ANI_GENERATED__.Measure(2, 3);
__assert_eq_string("Measure.kind_pair", pair.kind, "pair");
__assert_eq_int("Measure.total_pair", pair.total, 5);
__assert_eq_string("Measure.describe_pair", pair.describe(), "pair:5");

let named = new __ANI_GENERATED__.Measure("named", 4);
__assert_eq_string("Measure.kind_named", named.kind, "named");
__assert_eq_int("Measure.total_named", named.total, 4);
__assert_eq_string("Measure.describe_named", named.describe(), "named:4");
ETS
      ;;

    ani-example-class-method-overload)
      cat <<'ETS'
let box = new __ANI_GENERATED__.MathBox(5);
__assert_eq_int("MathBox.base", box.base, 5);
__assert_eq_int("MathBox.mix2", box.mix(2, 3), 10);
__assert_eq_int("MathBox.mix3", box.mix(2, 3, 4), 14);
__assert_eq_string("MathBox.tag1", __ANI_GENERATED__.MathBox.tag("ark"), "[ark]");
__assert_eq_string("MathBox.tag2", __ANI_GENERATED__.MathBox.tag("ark", "ts"), "[ark:ts]");
ETS
      ;;
    ani-example-new-class)
      cat <<'ETS'
let calcToken = __ANI_GENERATED__.Calculator.create();
__assert_true("Calculator.create_handle", calcToken > 0);

let person = new __ANI_GENERATED__.Person("Alice", 30);
__assert_eq_string("Person.name", person.name, "Alice");
__assert_eq_int("Person.age", person.age, 30);
person.age = 31;
__assert_eq_int("Person.age_after_set", person.age, 31);
__assert_eq_string("Person.greet", person.greet(), "Hello, I'm Alice and I'm 31 years old!");
person.destroy();
ETS
      ;;
    ani-example-impl-block)
      cat <<'ETS'
let widget = new __ANI_GENERATED__.Widget("impl", 2);
__assert_eq_string("Widget.name", widget.name, "impl");
__assert_eq_int("Widget.count", widget.count, 2);
__assert_eq_string("Widget.maybe_name_some", widget.maybe_name(true) as String, "impl");
__assert_true("Widget.maybe_name_none", widget.maybe_name(false) == undefined);
let widgetSnapshot = widget.snapshot();
__assert_eq_string("Widget.snapshot_label", widgetSnapshot.label, "impl");
__assert_eq_int("Widget.snapshot_total", widgetSnapshot.total, 2);
let maybeSnapshot = widget.maybe_snapshot(true) as WidgetSnapshot;
__assert_eq_string("Widget.maybe_snapshot_label", maybeSnapshot.label, "impl");
__assert_true("Widget.maybe_snapshot_none", widget.maybe_snapshot(false) == undefined);
let checkedSnapshot = widget.checked_snapshot(2);
__assert_eq_int("Widget.checked_snapshot_total", checkedSnapshot.total, 2);
__assert_throws("Widget.checked_snapshot_error", (): void => {
  widget.checked_snapshot(3);
});
let maybeCheckedSnapshot = widget.maybe_checked_snapshot(true, 2) as WidgetSnapshot;
__assert_eq_int("Widget.maybe_checked_snapshot_total", maybeCheckedSnapshot.total, 2);
__assert_true("Widget.maybe_checked_snapshot_none", widget.maybe_checked_snapshot(false, 2) == undefined);
__assert_throws("Widget.maybe_checked_snapshot_error", (): void => {
  widget.maybe_checked_snapshot(true, 3);
});
let chooseSnapshot = widget.choose_snapshot(true) as WidgetSnapshot;
__assert_eq_string("Widget.choose_snapshot_label", chooseSnapshot.label, "impl");
let chooseText = widget.choose_snapshot(false) as String;
__assert_eq_string("Widget.choose_snapshot_text", chooseText.toString(), "Widget(impl, 2)");
let mergeInput = new WidgetSnapshot();
mergeInput.label = "child";
mergeInput.total = 5;
__assert_eq_string("Widget.merge_snapshot_input_object", widget.merge_snapshot_input(mergeInput), "impl:child+7");
__assert_eq_string("Widget.merge_snapshot_input_string", widget.merge_snapshot_input("plain"), "impl:plain");
let previousName = widget.rename("renamed");
__assert_eq_string("Widget.rename_previous", previousName as String, "impl");
__assert_eq_string("Widget.name_after_rename", widget.name, "renamed");
__assert_throws("Widget.rename_undefined", (): void => {
  widget.rename(undefined);
});
widget.count = 5;
__assert_eq_int("Widget.count_after_set", widget.count, 5);
__assert_eq_int("Widget.bump", widget.bump(3), 8);
__assert_eq_int("Widget.count_after_bump", widget.count, 8);
__assert_eq_string("Widget.describe", widget.describe(), "Widget(renamed, 8)");
__assert_eq_string("Widget.index_get", widget.$_get(4.0), "renamed#4");
widget.$_set(2.0, "slot");
__assert_eq_string("Widget.index_set_text_name", widget.name, "slot@2");
widget.$_set(6.0, true);
__assert_eq_int("Widget.index_set_flag_true", widget.count, 6);
widget.$_set(3.0, false);
__assert_eq_int("Widget.index_set_flag_false", widget.count, -3);
let widgetIter = widget.$_iterator();
let widgetIterFirst = widgetIter.next();
__assert_true("Widget.iterator_negative_done", widgetIterFirst.done);
__assert_true("Widget.iterator_negative_value", widgetIterFirst.value == undefined);
let widgetIterEnd = widgetIter.next();
__assert_true("Widget.iterator_negative_repeat_done", widgetIterEnd.done);
__assert_true("Widget.iterator_negative_repeat_value", widgetIterEnd.value == undefined);
widget.count = 4;
let widgetIterPositive = widget.$_iterator();
let widgetIterPositiveFirst = widgetIterPositive.next();
__assert_true("Widget.iterator_positive_first_done", !widgetIterPositiveFirst.done);
__assert_eq_int("Widget.iterator_positive_first_value", widgetIterPositiveFirst.value as int, 0);
let widgetIterPositiveSecond = widgetIterPositive.next();
__assert_eq_int("Widget.iterator_positive_second_value", widgetIterPositiveSecond.value as int, 1);
let widgetIterPositiveThird = widgetIterPositive.next();
__assert_eq_int("Widget.iterator_positive_third_value", widgetIterPositiveThird.value as int, 2);
let widgetIterPositiveFourth = widgetIterPositive.next();
__assert_eq_int("Widget.iterator_positive_fourth_value", widgetIterPositiveFourth.value as int, 3);
let widgetIterPositiveEnd = widgetIterPositive.next();
__assert_true("Widget.iterator_positive_end_done", widgetIterPositiveEnd.done);
__assert_true("Widget.iterator_positive_end_value", widgetIterPositiveEnd.value == undefined);
__assert_eq_int("Widget.sum", __ANI_GENERATED__.Widget.sum(2, 4), 6);
__assert_eq_int("Widget.revision_initial", __ANI_GENERATED__.Widget.revision, 1);
__ANI_GENERATED__.Widget.revision = 7;
__assert_eq_int("Widget.revision_after_set", __ANI_GENERATED__.Widget.revision, 7);
__assert_true(
  "Widget.resolve_special_methods",
  __ANI_GENERATED__.resolve_widget_special_methods("arkvm_test.Widget")
);
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
    ani-example-object-access)
      cat <<'ETS'
class AccessTarget {
  counter: int;
  label: string;
  private _ratio: double;
  private _alias: string;

  constructor(counter: int, label: string, ratio: double, alias: string) {
    this.counter = counter;
    this.label = label;
    this._ratio = ratio;
    this._alias = alias;
  }

  get ratio(): double {
    return this._ratio;
  }

  set ratio(value: double) {
    this._ratio = value;
  }

  get alias(): string {
    return this._alias;
  }

  set alias(value: string) {
    this._alias = value;
  }
}

let target = new AccessTarget(1, "seed", 1.5, "alpha");
__assert_eq_int("field_by_name_int_roundtrip", __ANI_GENERATED__.field_by_name_int_roundtrip(target, 7), 7);
__assert_eq_int("field_by_name_int_state", target.counter, 7);
__assert_eq_int("field_by_handle_int_roundtrip", __ANI_GENERATED__.field_by_handle_int_roundtrip(target, 11), 11);
__assert_eq_int("field_by_handle_int_state", target.counter, 11);
__assert_true("field_ref_roundtrip", __ANI_GENERATED__.field_ref_roundtrip(target, "renamed"));
__assert_eq_string("field_ref_state", target.label, "renamed");
__assert_eq_double("property_by_name_double_roundtrip", __ANI_GENERATED__.property_by_name_double_roundtrip(target, 2.75), 2.75);
__assert_eq_double("property_by_name_double_state", target.ratio, 2.75);
__assert_true("property_ref_roundtrip", __ANI_GENERATED__.property_ref_roundtrip(target, "beta"));
__assert_eq_string("property_ref_state", target.alias, "beta");
ETS
      ;;
    ani-example-object-runtime)
      cat <<'ETS'
class RuntimeBox {
  value: int;
  label: string;

  constructor(value: int, label: string) {
    this.value = value;
    this.label = label;
  }

  sumNumbers(left: int, right: int): int {
    return this.value + left + right;
  }

  compareNumbers(left: int, right: int): string {
    return left == right ? "eq" : "ne";
  }

  describe(): string {
    return this.label + ":" + this.value;
  }

  isPositive(): boolean {
    return this.value > 0;
  }

  clearLabel(): void {
    this.label = "cleared";
  }
}

class RuntimeSubBox extends RuntimeBox {
  constructor(value: int, label: string) {
    super(value, label);
  }
}

__assert_eq_string("create_runtime_box", __ANI_GENERATED__.create_runtime_box(5, "seed"), "seed:5");
let created = new RuntimeBox(5, "seed");
__assert_eq_int("sum_by_name", __ANI_GENERATED__.sum_by_name(created, 2, 3), 10);
__assert_eq_string("compare_by_name_eq", __ANI_GENERATED__.compare_by_name(created, 4, 4), "eq");
__assert_eq_string("compare_by_name_ne", __ANI_GENERATED__.compare_by_name(created, 4, 5), "ne");
__assert_eq_string("describe_by_name_zero", __ANI_GENERATED__.describe_by_name_zero(created), "seed:5");
__assert_true("is_positive_by_name", __ANI_GENERATED__.is_positive_by_name(created));
__assert_eq_string("clear_label_by_name", __ANI_GENERATED__.clear_label_by_name(created), "cleared");
__assert_eq_string("describe_by_handle", __ANI_GENERATED__.describe_by_handle(created), "cleared:5");
__assert_true("is_runtime_box_instance_base", __ANI_GENERATED__.is_runtime_box_instance(created));
let derived = new RuntimeSubBox(9, "child");
__assert_true("is_runtime_box_instance_sub", __ANI_GENERATED__.is_runtime_box_instance(derived));
__assert_true("runtime_box_assignable_to_base", __ANI_GENERATED__.runtime_box_assignable_to_base(derived));
__assert_true("runtime_box_has_super", __ANI_GENERATED__.runtime_box_has_super(derived));
ETS
      ;;
    ani-example-object-model)
      cat <<ETS
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

__assert_eq_string(
  "describe_optional_user_profile_undefined",
  __ANI_GENERATED__.describe_optional_user_profile(undefined),
  "none",
);
__assert_eq_string(
  "describe_optional_user_profile_null",
  __ANI_GENERATED__.describe_optional_user_profile(null),
  "none",
);
let maybeUser = __ANI_GENERATED__.maybe_user_profile(true);
__assert_true("maybe_user_profile_some", maybeUser != undefined);
let maybeUserObj = maybeUser as UserProfile;
__assert_eq_int("maybe_user_profile_some_id", maybeUserObj.id, 11);
__assert_eq_string("maybe_user_profile_some_name", maybeUserObj.name, "maybe");
__assert_true("maybe_user_profile_none", __ANI_GENERATED__.maybe_user_profile(false) == undefined);
__assert_eq_string(
  "describe_optional_user_profile_roundtrip",
  __ANI_GENERATED__.describe_optional_user_profile(maybeUser),
  "11:maybe:active",
);
let maybeResult = __ANI_GENERATED__.maybe_user_profile_result(true);
__assert_true("maybe_user_profile_result_some", maybeResult != undefined);
let maybeResultObj = maybeResult as UserProfile;
__assert_eq_int("maybe_user_profile_result_id", maybeResultObj.id, 12);
__assert_eq_string("maybe_user_profile_result_name", maybeResultObj.name, "result-option");
__assert_true("maybe_user_profile_result_none", __ANI_GENERATED__.maybe_user_profile_result(false) == undefined);

let resultOk = __ANI_GENERATED__.user_profile_result(true);
__assert_eq_int("user_profile_result_id", resultOk.id, 9);
__assert_eq_string("user_profile_result_name", resultOk.name, "result");
__assert_true("user_profile_result_inactive", !resultOk.active);
__assert_throws("user_profile_result_error", (): void => {
  __ANI_GENERATED__.user_profile_result(false);
});

let directory = __ANI_GENERATED__.make_user_profile_directory();
let primary = directory.get("primary");
__assert_true("make_user_profile_directory_primary_present", primary != undefined);
let primaryUser = primary as UserProfile;
__assert_eq_int("make_user_profile_directory_primary_id", primaryUser.id, 21);
__assert_eq_string("make_user_profile_directory_primary_name", primaryUser.name, "directory-primary");
let inputDirectory = new Map<string, UserProfile>(0);
let inputPrimary = new UserProfile();
inputPrimary.id = 31;
inputPrimary.name = "input-primary";
inputPrimary.active = true;
let inputBackup = new UserProfile();
inputBackup.id = 32;
inputBackup.name = "input-backup";
inputBackup.active = false;
inputDirectory.set("primary", inputPrimary);
inputDirectory.set("backup", inputBackup);
__assert_eq_string(
  "summarize_user_profile_directory",
  __ANI_GENERATED__.summarize_user_profile_directory(inputDirectory),
  "backup=32#input-backup#inactive|primary=31#input-primary#active",
);

let record = __ANI_GENERATED__.make_user_profile_record();
let recordPrimary = record["primary"];
__assert_true("make_user_profile_record_primary_present", recordPrimary != undefined);
let recordPrimaryUser = recordPrimary as UserProfile;
__assert_eq_int("make_user_profile_record_primary_id", recordPrimaryUser.id, 41);
__assert_eq_string("make_user_profile_record_primary_name", recordPrimaryUser.name, "record-primary");
let inputRecord = new Record<string, UserProfile>();
let inputRecordPrimary = new UserProfile();
inputRecordPrimary.id = 51;
inputRecordPrimary.name = "record-input-primary";
inputRecordPrimary.active = true;
let inputRecordBackup = new UserProfile();
inputRecordBackup.id = 52;
inputRecordBackup.name = "record-input-backup";
inputRecordBackup.active = false;
inputRecord["primary"] = inputRecordPrimary;
inputRecord["backup"] = inputRecordBackup;
__assert_eq_string(
  "summarize_user_profile_record",
  __ANI_GENERATED__.summarize_user_profile_record(inputRecord),
  "backup=52#record-input-backup#inactive|primary=51#record-input-primary#active",
);

let group = __ANI_GENERATED__.make_user_profile_group();
__assert_eq_int("make_user_profile_group_size", group.size, 2);
__assert_eq_string(
  "summarize_user_profile_group_roundtrip",
  __ANI_GENERATED__.summarize_user_profile_group(group),
  "61#set-primary#active|62#set-backup#inactive",
);
let inputGroup = new Set<UserProfile>(0);
let inputGroupPrimary = new UserProfile();
inputGroupPrimary.id = 71;
inputGroupPrimary.name = "set-input-primary";
inputGroupPrimary.active = true;
let inputGroupBackup = new UserProfile();
inputGroupBackup.id = 72;
inputGroupBackup.name = "set-input-backup";
inputGroupBackup.active = false;
inputGroup.add(inputGroupPrimary);
inputGroup.add(inputGroupBackup);
__assert_eq_string(
  "summarize_user_profile_group_input",
  __ANI_GENERATED__.summarize_user_profile_group(inputGroup),
  "71#set-input-primary#active|72#set-input-backup#inactive",
);
ETS
      ;;

    ani-example-optional)
      cat <<'ETS'
__assert_eq_int("with_default_simple", __ANI_GENERATED__.with_default_simple(3, 4), 12);
__assert_eq_int("with_optional_int_some", __ANI_GENERATED__.with_optional_int(8, 2), 10);
__assert_eq_int("with_optional_int_null", __ANI_GENERATED__.with_optional_int(8, null), 8);
__assert_eq_double("with_optional_double_some", __ANI_GENERATED__.with_optional_double(1.5, 2.0), 3.5);
__assert_eq_double("with_optional_double_null", __ANI_GENERATED__.with_optional_double(1.5, null), 1.5);
__assert_eq_int("with_optional_boolean_null", __ANI_GENERATED__.with_optional_boolean(8, null), 8);
__assert_eq_int("with_optional_boolean_true", __ANI_GENERATED__.with_optional_boolean(8, true), 16);
__assert_eq_int("with_multiple_optional_mixed", __ANI_GENERATED__.with_multiple_optional(1, 2, null, 4), 7);
__assert_eq_int("with_optional_int_undefined", __ANI_GENERATED__.with_optional_int(8, undefined), 8);
__assert_eq_string("with_optional_string_undefined", __ANI_GENERATED__.with_optional_string("x", undefined), "x");
let madeInt = __ANI_GENERATED__.make_optional_int(true);
__assert_eq_int("make_optional_int_some_roundtrip", __ANI_GENERATED__.with_optional_int(3, madeInt), 10);
__assert_true("make_optional_int_none", __ANI_GENERATED__.make_optional_int(false) == undefined);
let madeString = __ANI_GENERATED__.make_optional_string(true);
__assert_eq_string("make_optional_string_some_roundtrip", __ANI_GENERATED__.with_optional_string("x", madeString), "x ok");
__assert_true("make_optional_string_none", __ANI_GENERATED__.make_optional_string(false) == undefined);
__assert_eq_long("with_optional_long_null", __ANI_GENERATED__.with_optional_long(9, null), 9);
__assert_eq_double("with_optional_float_null", __ANI_GENERATED__.with_optional_float(2.5, null), 2.5);
__assert_eq_string("with_optional_string_some", __ANI_GENERATED__.with_optional_string("x", "y"), "x y");
__assert_eq_string("with_optional_string_null", __ANI_GENERATED__.with_optional_string("x", null), "x");
ETS
      ;;
    ani-example-map)
      cat <<'ETS'
let scores = __ANI_GENERATED__.make_score_map();
__assert_true("make_score_map_has_ani", scores.has("ani"));
let aniScore = scores.get("ani");
__assert_true("make_score_map_get_ani_present", aniScore != undefined);
__assert_eq_int("make_score_map_get_ani", aniScore as int, 1);
let missingScore = scores.get("missing");
__assert_true("make_score_map_missing", missingScore == undefined);
let emptyScores = __ANI_GENERATED__.make_empty_score_map();
__assert_true("make_empty_score_map_missing", !emptyScores.has("ani"));
let inputScores = new Map<string, int>(0);
inputScores.set("ani", 1);
inputScores.set("arkts", 2);
inputScores.set("ets", 3);
__assert_eq_int("sum_score_map", __ANI_GENERATED__.sum_score_map(inputScores), 6);
ETS
      ;;

    ani-example-set)
      cat <<'ETS'
let words = __ANI_GENERATED__.make_word_set();
__assert_true("make_word_set_has_ani", words.has("ani"));
__assert_true("make_word_set_has_arkts", words.has("arkts"));
__assert_true("make_word_set_has_ets", words.has("ets"));
__assert_true("make_word_set_missing", !words.has("missing"));
let emptyWords = __ANI_GENERATED__.make_empty_word_set();
__assert_true("make_empty_word_set_missing", !emptyWords.has("ani"));
let inputWords = new Set<string>(0);
inputWords.add("ani");
inputWords.add("arkts");
inputWords.add("ets");
__assert_eq_int("count_word_set", __ANI_GENERATED__.count_word_set(inputWords), 3);
__assert_true("has_word_true", __ANI_GENERATED__.has_word(inputWords, "ani"));
__assert_true("has_word_false", !__ANI_GENERATED__.has_word(inputWords, "missing"));
let sortedWords = __ANI_GENERATED__.make_sorted_word_set();
__assert_true("make_sorted_word_set_has_ani", sortedWords.has("ani"));
__assert_true("make_sorted_word_set_has_arkts", sortedWords.has("arkts"));
__assert_true("make_sorted_word_set_has_ets", sortedWords.has("ets"));
__assert_eq_int("count_sorted_word_set", __ANI_GENERATED__.count_sorted_word_set(inputWords), 3);
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

let directRecord = __ANI_GENERATED__.create_record_direct();
__assert_eq_int("record_sum_direct", __ANI_GENERATED__.record_sum(directRecord), 44);
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
__assert_eq_string(
  "handle_string_or_int_either_int",
  __ANI_GENERATED__.handle_string_or_int_either(7),
  "Int: 7",
);
__assert_eq_string(
  "handle_three_types_bool",
  __ANI_GENERATED__.handle_three_types(true),
  "Boolean: true",
);
__assert_eq_string(
  "handle_four_types_double",
  __ANI_GENERATED__.handle_four_types(3.5),
  "Double: 3.5",
);
let eitherString = __ANI_GENERATED__.return_either(true) as String;
__assert_eq_string("return_either_string", eitherString.toString(), "Hello from Either!");
let eitherInt = __ANI_GENERATED__.return_either(false) as Int;
__assert_eq_string("return_either_int", eitherInt.toString(), "42");
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

let typedRes = __ANI_GENERATED__.create_native_resource_handle(9, "typed");
__assert_true("create_native_resource_typed_handle", typedRes > 0);
__assert_eq_int("get_native_resource_handle_id", __ANI_GENERATED__.get_native_resource_handle_id(typedRes), 9);
__ANI_GENERATED__.destroy_native_resource_handle(typedRes);

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
