use leptos::{
    callback::Callback,
    reactive::{computed::Memo, signal::RwSignal, traits::Get, traits::Set},
};

#[derive(Clone, Copy)]
pub struct FormField<T> {
    pub value: RwSignal<T>,
    pub touched: RwSignal<bool>,
    pub is_dirty: Memo<bool>,
    pub is_pristine: Memo<bool>,
    pub error: Memo<Option<&'static str>>,
    pub show_error: Memo<bool>,
    pub mark_touched: Callback<web_sys::FocusEvent>,
}

#[derive(Clone, Copy)]
pub enum ShowRule {
    TouchedOrSubmitted,
    DirtyOrSubmitted,
    SubmittedOnly,
    Always,
}

impl ShowRule {
    fn should_show(self, touched: bool, dirty: bool, submitted: bool) -> bool {
        match self {
            ShowRule::TouchedOrSubmitted => touched || submitted,
            ShowRule::DirtyOrSubmitted => dirty || submitted,
            ShowRule::SubmittedOnly => submitted,
            ShowRule::Always => true,
        }
    }
}

pub fn use_form_field<T>(
    initial: T,
    validate: impl Fn(T) -> Option<&'static str> + Copy + Send + Sync + 'static,
    submitted: RwSignal<bool>,
    rule: ShowRule,
) -> FormField<T>
where
    T: PartialEq + Clone + Send + Sync + 'static,
{
    let value = RwSignal::new(initial.clone());
    let touched = RwSignal::new(false);

    let is_dirty = Memo::new(move |_| value.get() != initial);
    let is_pristine = Memo::new(move |_| !is_dirty.get());

    let error = Memo::new(move |_| validate(value.get()));
    let show_error = Memo::new(move |_| {
        rule.should_show(touched.get(), is_dirty.get(), submitted.get()) && error.get().is_some()
    });

    let mark_touched = Callback::new(move |_: web_sys::FocusEvent| touched.set(true));

    FormField {
        value,
        touched,
        is_dirty,
        is_pristine,
        error,
        show_error,
        mark_touched,
    }
}
