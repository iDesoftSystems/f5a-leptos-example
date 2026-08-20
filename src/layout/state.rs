use leptos::prelude::RwSignal;

#[derive(Debug, Copy, Clone)]
pub struct SidebarContext(pub RwSignal<bool>);
