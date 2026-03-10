use ani_derive::ani;
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Mutex, OnceLock,
};

pub struct Widget;

static WIDGET_NAME: OnceLock<Mutex<String>> = OnceLock::new();
static WIDGET_COUNT: AtomicI32 = AtomicI32::new(0);

fn widget_name_store() -> &'static Mutex<String> {
    WIDGET_NAME.get_or_init(|| Mutex::new(String::new()))
}

#[ani(class = "Widget")]
impl Widget {
    #[ani(constructor)]
    pub fn new(name: String, count: i32) {
        WIDGET_COUNT.store(count, Ordering::SeqCst);
        if let Ok(mut slot) = widget_name_store().lock() {
            *slot = name;
        }
    }

    #[ani(getter)]
    pub fn get_name() -> String {
        widget_name_store()
            .lock()
            .map(|s| s.clone())
            .unwrap_or_default()
    }

    #[ani(getter)]
    pub fn get_count() -> i32 {
        WIDGET_COUNT.load(Ordering::SeqCst)
    }

    #[ani(setter)]
    pub fn set_count(count: i32) {
        WIDGET_COUNT.store(count, Ordering::SeqCst);
    }

    #[ani]
    pub fn describe() -> String {
        format!("Widget({}, {})", Self::get_name(), Self::get_count())
    }

    #[ani(static, name = "sum")]
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::Widget;

    #[test]
    fn widget_impl_methods_work() {
        Widget::new("demo".to_string(), 3);
        assert_eq!(Widget::get_name(), "demo");
        assert_eq!(Widget::get_count(), 3);
        Widget::set_count(5);
        assert_eq!(Widget::get_count(), 5);
        assert_eq!(Widget::describe(), "Widget(demo, 5)");
        assert_eq!(Widget::add(2, 4), 6);
    }
}
