use leptos::prelude::{Signal, WriteSignal};
use leptos_use::ColorMode;

#[derive(Debug, Clone, Copy)]
pub struct ThemeContext {
    pub mode: Signal<ColorMode>,
    pub mode_tx: WriteSignal<ColorMode>,
}
