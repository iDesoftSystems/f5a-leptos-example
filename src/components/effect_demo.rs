use leptos::{logging, prelude::*};

#[component]
pub fn ThemeConfig() -> impl IntoView {
    let (dark_mode, dark_mode_tx) = signal(false);

    Effect::new(move || {
        let current_state = dark_mode.get();

        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme-preference", &current_state.to_string());
            }
        }

        logging::log!("theme saved to localstorage: {}", current_state);
    });

    let on_toggle_dark_mode = move |_| {
        dark_mode_tx.update(|mode| {
            *mode = !*mode;
        });
    };

    view! {
        <button
            on:click=on_toggle_dark_mode
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Toggle Dark Mode"
        </button>
    }
}

#[component]
pub fn Notifications() -> impl IntoView {
    let (notifications, notifications_tx) = signal(0);

    Effect::new(move |_| {
        let notifications_counter = notifications.get();

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if notifications_counter == 0 {
                    document.set_title("Inbox");
                } else {
                    document.set_title(&format!("({}) new messages", notifications_counter));
                }
            }
        }
    });

    let on_receive_message = move |_| {
        notifications_tx.update(|n| *n += 1);
    };

    let on_mark_as_read = move |_| {
        notifications_tx.set(0);
    };

    view! {
        <button
            on:click=on_receive_message
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Receive message"
        </button>

        <button
            on:click=on_mark_as_read
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Mark as read"
        </button>
    }
}
