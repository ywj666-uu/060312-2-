use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::*;
use crate::components::navbar::Navbar;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/leaderboard")]
    Leaderboard,
    #[at("/projects")]
    Projects,
    #[at("/profile/:id")]
    Profile { id: i64 },
    #[at("/admin")]
    Admin,
    #[at("/settings")]
    Settings,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <home::HomePage /> },
        Route::Leaderboard => html! { <leaderboard_page::LeaderboardPage /> },
        Route::Projects => html! { <projects_page::ProjectsPage /> },
        Route::Profile { id } => html! { <profile_page::ProfilePage user_id={id} /> },
        Route::Admin => html! { <admin_page::AdminPage /> },
        Route::Settings => html! { <settings_page::SettingsPage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <main class="container">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}
