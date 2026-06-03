use yew::prelude::*;
use crate::api;
use crate::types::*;
use crate::components::leaderboard::LeaderboardTable;
use crate::components::user_card::UserCard;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let users = use_state(|| Vec::<UserResponse>::new());
    let leaderboard = use_state(|| Vec::<LeaderboardEntry>::new());
    let sync_status = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let users = users.clone();
        let leaderboard = leaderboard.clone();
        let sync_status = sync_status.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(u) = api::list_users().await {
                    users.set(u);
                }
                if let Ok(lb) = api::get_leaderboard(None, 5).await {
                    leaderboard.set(lb);
                }
                if let Ok(s) = api::get_sync_status().await {
                    sync_status.set(s.last_sync);
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_sync = {
        let sync_status = sync_status.clone();
        Callback::from(move |_: MouseEvent| {
            let sync_status = sync_status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = api::trigger_sync().await;
                if let Ok(s) = api::get_sync_status().await {
                    sync_status.set(s.last_sync);
                }
            });
        })
    };

    if *loading {
        return html! { <div class="loading">{"加载中..."}</div> };
    }

    html! {
        <div class="home-page">
            <section class="hero">
                <h1>{"开源贡献者激励平台"}</h1>
                <p>{"追踪贡献，激励协作，共建开源社区"}</p>
                <div class="sync-info">
                    <span>{ format!("上次同步: {}", sync_status.as_deref().unwrap_or("尚未同步")) }</span>
                    <button onclick={on_sync}>{"立即同步"}</button>
                </div>
            </section>

            <section class="section">
                <h2>{"贡献者"}</h2>
                <div class="user-cards">
                    { for (*users).iter().take(6).map(|u| {
                        html! { <UserCard user={u.clone()} /> }
                    })}
                </div>
            </section>

            <section class="section">
                <h2>{"排行榜 Top 5"}</h2>
                <LeaderboardTable entries={(*leaderboard).clone()} />
            </section>
        </div>
    }
}
