use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="navbar-brand">
                <Link<Route> to={Route::Home} classes="brand-link">
                    {"🏆 开源贡献者激励平台"}
                </Link<Route>>
            </div>
            <div class="navbar-menu">
                <Link<Route> to={Route::Leaderboard} classes="nav-link">{"排行榜"}</Link<Route>>
                <Link<Route> to={Route::Projects} classes="nav-link">{"项目"}</Link<Route>>
                <Link<Route> to={Route::Admin} classes="nav-link">{"管理"}</Link<Route>>
                <Link<Route> to={Route::Settings} classes="nav-link">{"设置"}</Link<Route>>
            </div>
        </nav>
    }
}
