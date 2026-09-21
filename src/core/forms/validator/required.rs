pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        self.trim().is_empty()
    }
}

impl IsEmpty for bool {
    fn is_empty(&self) -> bool {
        !*self
    }
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

pub fn required<T: IsEmpty>(message: &'static str) -> impl Fn(T) -> Option<&'static str> + Copy {
    move |value: T| {
        if value.is_empty() {
            return Some(message);
        }

        None
    }
}
