use leptos::{attr::Attribute, view};

pub fn clickable_icon_styles_attrs(extra_class: impl Into<String>) -> impl Attribute {
    let full_class = format!(
        "rounded-md p-1 text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700 {}",
        extra_class.into()
    );
    view! {
        <{..} class=full_class />
    }
}
