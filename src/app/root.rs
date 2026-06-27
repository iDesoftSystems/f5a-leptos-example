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

                    <ParentRoute path=path!("/users") view=Outlet>
                        <Route path=path!("") view=users::pages::UsersPage />
                        <Route path=path!(":user_id") view=users::pages::UserDetailPage />
                    </ParentRoute>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
