use ani_derive::ani;

#[ani(finalize)]
fn invalid_finalize(value: i32) {
    let _ = value;
}

fn main() {}
