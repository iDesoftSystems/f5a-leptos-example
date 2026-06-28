use leptos::{attr::Attribute, view};

pub fn clickable_icon_styles_attrs() -> impl Attribute {
    view! {
        <{..} class="rounded-md p-1 text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700" />
    }
}
