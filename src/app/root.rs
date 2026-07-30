use leptos::prelude::*;
use leptos_meta::Html;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use leptos_router::components::Outlet;
use leptos_use::{UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options};

use crate::{
    features::{auth, home},
    layout, pages,
};
use crate::features::users;

#[component]
pub fn App() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default());

    view! {
        <Html {..} class=move || mode.get().to_string()/>

        <Router>
            <Routes fallback=pages::NotFoundPage>
                <Route path=path!("/login") view=auth::pages::LoginPage />

                <ParentRoute path=path!("/") view=move || view!{ <layout::ProtectedRoutes mode=mode mode_tx=set_mode /> }>
                    <Route path=path!("/") view=home::pages::HomePage />

                    <ParentRoute path=path!("/users") view=Outlet>
                        <Route path=path!("") view=users::pages::UserListPage />
                        <Route path=path!("/create") view=users::pages::UserCreatePage />
                    </ParentRoute>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
