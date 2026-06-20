use leptos::{logging, prelude::*};

#[component]
pub fn SimpleDerived() -> impl IntoView {
    let (count, counter_tx): (ReadSignal<i32>, WriteSignal<i32>) = signal(0);
    // let count_plus_ten = move || count.get() + 10;
    let count_plus_ten: Memo<i32> = Memo::new(move |_| count.get() + 10);

    view! {
        <button on:click=move |_| counter_tx.set(count.get() + 1)>
            "Count is: " {count_plus_ten}
        </button>
    }
}

#[component]
pub fn PasswordValidator() -> impl IntoView {
    let (password, password_tx) = signal(String::new());

    let password_strength = move || {
        logging::log!("[PasswordValidator] checking password strength");

        let password_value = password.get();

        if password_value.is_empty() {
            return "Empty";
        }

        let has_metrics = password_value.len() > 8
            && password_value.chars().any(|char| char.is_numeric())
            && password_value.chars().any(|char| char.is_uppercase());

        if has_metrics {
            return "Strong";
        }

        if password_value.len() > 4 {
            return "Medium";
        }

        "Weak"
    };

    let password_strength_class = move || {
        logging::log!("[PasswordValidator] processing class for badge");

        let password_strength = password_strength();

        match password_strength {
            "Empty" => "min-h-4 min-w-4 bg-slate-500 rounded px-1",
            "Strong" => "min-h-4 min-w-4 bg-green-500 rounded px-1",
            "Medium" => "min-h-4 min-w-4 bg-yellow-500 rounded px-1",
            "Weak" => "min-h-4 min-w-4 bg-red-500 rounded px-1",
            _ => "",
        }
    };

    let on_input_password = move |ev| {
        password_tx.set(event_target_value(&ev));
    };

    view! {
        <div class="flex flex-col gap-y-2">
            <label for="password">Password</label>
            <input
                on:input=on_input_password
                id="password"
                type="password"
                class="px-2 py-2 rounded border border-slate-400" />
            <div class=password_strength_class>
                {password_strength}
            </div>
        </div>
    }
}

#[component]
pub fn PasswordValidatorMemoized() -> impl IntoView {
    let (password, password_tx) = signal(String::new());

    let password_strength = Memo::new(move |_| {
        logging::log!("[PasswordValidatorMemoized] checking password strength");

        let password_value = password.get();

        if password_value.is_empty() {
            return "Empty";
        }

        let has_metrics = password_value.len() > 8
            && password_value.chars().any(|char| char.is_numeric())
            && password_value.chars().any(|char| char.is_uppercase());

        if has_metrics {
            return "Strong";
        }

        if password_value.len() > 4 {
            return "Medium";
        }

        "Weak"
    });

    let password_strength_class = move || {
        logging::log!("[PasswordValidatorMemoized] processing class for badge");
        let password_strength = password_strength.get();

        match password_strength {
            "Empty" => "min-h-4 min-w-4 bg-slate-500 rounded px-1",
            "Strong" => "min-h-4 min-w-4 bg-green-500 rounded px-1",
            "Medium" => "min-h-4 min-w-4 bg-yellow-500 rounded px-1",
            "Weak" => "min-h-4 min-w-4 bg-red-500 rounded px-1",
            _ => "",
        }
    };

    let on_input_password = move |ev| {
        password_tx.set(event_target_value(&ev));
    };

    view! {
        <div class="flex flex-col gap-y-2">
            <label for="password">Password with memo</label>
            <input
                on:input=on_input_password
                id="password"
                type="password"
                class="px-2 py-2 rounded border border-slate-400" />
            <div class=password_strength_class>
                {password_strength}
            </div>
        </div>
    }
}
