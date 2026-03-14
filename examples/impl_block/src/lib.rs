use ani::conversions::Either;
use ani::prelude::*;
use ani_derive::{ani, AniClass};
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(AniClass)]
#[ani(class = "Widget")]
pub struct Widget {
    pub _name: String,
    pub _count: i32,
}

#[derive(Debug, PartialEq, Eq, AniClass)]
#[ani(class = "WidgetSnapshot")]
pub struct WidgetSnapshot {
    pub label: String,
    pub total: i32,
}

static WIDGET_REVISION: AtomicI32 = AtomicI32::new(1);

#[ani(class = "Widget")]
impl Widget {
    #[ani(constructor)]
    pub fn new(env: &Env<'_>, this: &AniObject<'_>, name: String, count: i32) -> Result<()> {
        Widget {
            _name: name,
            _count: count,
        }
        .write_back_to_ani_object(env, this)
    }

    #[ani(getter)]
    pub fn get_name(&self) -> String {
        self._name.clone()
    }

    #[ani(getter)]
    pub fn get_count(&self) -> i32 {
        self._count
    }

    #[ani(setter)]
    pub fn set_count(&mut self, count: i32) {
        self._count = count;
    }

    #[ani]
    pub fn describe(&self) -> String {
        format!("Widget({}, {})", self._name, self._count)
    }

    #[ani]
    pub fn snapshot(&self) -> WidgetSnapshot {
        WidgetSnapshot {
            label: self._name.clone(),
            total: self._count,
        }
    }

    #[ani]
    pub fn maybe_snapshot(&self, include: bool) -> Option<WidgetSnapshot> {
        if include {
            Some(self.snapshot())
        } else {
            None
        }
    }

    #[ani]
    pub fn checked_snapshot(&self, min_count: i32) -> Result<WidgetSnapshot> {
        if self._count < min_count {
            return Err(Error::new(
                Status::InvalidArgs,
                format!("count {} is below {}", self._count, min_count),
            ));
        }
        Ok(self.snapshot())
    }

    #[ani]
    pub fn maybe_checked_snapshot(
        &self,
        include: bool,
        min_count: i32,
    ) -> Result<Option<WidgetSnapshot>> {
        if !include {
            return Ok(None);
        }
        Ok(Some(self.checked_snapshot(min_count)?))
    }

    #[ani]
    pub fn choose_snapshot(&self, detailed: bool) -> Either<WidgetSnapshot, String> {
        if detailed {
            Either::A(self.snapshot())
        } else {
            Either::B(self.describe())
        }
    }

    #[ani]
    pub fn merge_snapshot_input(&self, value: Either<WidgetSnapshot, String>) -> String {
        match value {
            Either::A(snapshot) => format!(
                "{}:{}+{}",
                self._name,
                snapshot.label,
                self._count + snapshot.total,
            ),
            Either::B(text) => format!("{}:{}", self._name, text),
        }
    }

    #[ani]
    pub fn maybe_name(&self, include: bool) -> Option<String> {
        if include {
            Some(self._name.clone())
        } else {
            None
        }
    }

    #[ani]
    pub fn rename(&mut self, name: Option<String>) -> Result<Option<String>> {
        let Some(next_name) = name else {
            return Err(Error::new(Status::InvalidArgs, "name is required"));
        };

        let previous = if self._name.is_empty() {
            None
        } else {
            Some(self._name.clone())
        };
        self._name = next_name;
        Ok(previous)
    }

    #[ani]
    pub fn bump(&mut self, delta: i32) -> i32 {
        self._count += delta;
        self._count
    }

    #[ani(static, name = "sum")]
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[ani(static, getter = "revision")]
    pub fn revision() -> i32 {
        WIDGET_REVISION.load(Ordering::SeqCst)
    }

    #[ani(static, setter = "revision")]
    pub fn set_revision(value: i32) {
        WIDGET_REVISION.store(value, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::{Widget, WidgetSnapshot};
    use ani::conversions::Either;

    #[test]
    fn widget_impl_methods_work() {
        let mut widget = Widget {
            _name: "demo".to_string(),
            _count: 3,
        };
        assert_eq!(widget.get_name(), "demo");
        assert_eq!(widget.get_count(), 3);
        widget.set_count(5);
        assert_eq!(widget.get_count(), 5);
        assert_eq!(widget.bump(2), 7);
        assert_eq!(widget.describe(), "Widget(demo, 7)");

        let snapshot = widget.snapshot();
        assert_eq!(snapshot.label, "demo");
        assert_eq!(snapshot.total, 7);
        assert_eq!(widget.maybe_snapshot(true).unwrap().label, "demo");
        assert!(widget.maybe_snapshot(false).is_none());
        assert_eq!(widget.checked_snapshot(7).unwrap().total, 7);
        assert!(widget.checked_snapshot(8).is_err());
        assert_eq!(widget.maybe_checked_snapshot(false, 1).unwrap(), None);
        assert_eq!(
            widget
                .maybe_checked_snapshot(true, 7)
                .unwrap()
                .unwrap()
                .total,
            7
        );
        assert!(widget.maybe_checked_snapshot(true, 8).is_err());
        match widget.choose_snapshot(true) {
            Either::A(snapshot) => assert_eq!(snapshot.total, 7),
            Either::B(_) => panic!("expected snapshot"),
        }
        match widget.choose_snapshot(false) {
            Either::A(_) => panic!("expected string"),
            Either::B(text) => assert_eq!(text, "Widget(demo, 7)"),
        }
        assert_eq!(
            widget.merge_snapshot_input(Either::A(WidgetSnapshot {
                label: "child".to_string(),
                total: 2,
            })),
            "demo:child+9"
        );
        assert_eq!(
            widget.merge_snapshot_input(Either::B("plain".to_string())),
            "demo:plain"
        );

        assert_eq!(widget.maybe_name(true).as_deref(), Some("demo"));
        assert_eq!(widget.maybe_name(false), None);
        assert_eq!(
            widget.rename(Some("next".to_string())).unwrap().as_deref(),
            Some("demo")
        );
        assert_eq!(widget.get_name(), "next");
        assert!(widget.rename(None).is_err());

        let snapshot = WidgetSnapshot {
            label: "ok".to_string(),
            total: 9,
        };
        assert_eq!(snapshot.label, "ok");
        assert_eq!(snapshot.total, 9);
        assert_eq!(Widget::add(2, 4), 6);
        assert_eq!(Widget::revision(), 1);
        Widget::set_revision(9);
        assert_eq!(Widget::revision(), 9);
    }
}
