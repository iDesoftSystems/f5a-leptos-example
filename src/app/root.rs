use crate::features::{auth, home, users};
use crate::{layouts, pages};
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=pages::NotFoundPage>
                <Route path=path!("/login") view=auth::pages::LoginPage />

                <ParentRoute path=path!("/") view=layouts::ProtectedRoutes>
                    <Route path=path!("/") view=home::pages::HomePage />
                    <Route path=path!("/users") view=users::pages::UsersPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
