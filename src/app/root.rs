use leptos::prelude::*;
use leptos_meta::Html;
use leptos_router::components::Outlet;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use leptos_use::{UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options};

use crate::features::users;
use crate::state::ThemeContext;
use crate::{
    features::{auth, home},
    layout, pages,
};

#[component]
pub fn App() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default());

    provide_context(ThemeContext {
        mode,
        mode_tx: set_mode,
    });

    view! {
        <Html {..} class=move || mode.get().to_string()/>

        <Router>
            <Routes fallback=pages::NotFoundPage>
                <Route path=path!("/login") view=auth::pages::LoginPage />

                <ParentRoute path=path!("/") view=move || view!{ <layout::ProtectedRoutes /> }>
                    <Route path=path!("/") view=home::pages::HomePage />

                    <ParentRoute path=path!("/users") view=Outlet>
                        <Route path=path!("") view=users::pages::UserListPage />
                        <Route path=path!("/create") view=users::pages::UserCreatePage />
                        <Route path=path!("/:user_id") view=users::pages::UserDetailPage />
                    </ParentRoute>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
