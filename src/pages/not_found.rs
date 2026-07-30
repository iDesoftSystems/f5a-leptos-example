use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="min-h-screen flex flex-col items-center justify-center gap-y-2">
            <h1 class="text-3xl sm:text-4xl font-bold">Not Found</h1>
            <p class="text-lg sm:text-xl">The page you are looking for does not exist.</p>
            <a href="/"
                class="px-8 py-2 border text-base font-semibold rounded-md hover:bg-slate-50">
                Go back
            </a>
        </section>
    }
}
