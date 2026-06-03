use yew::prelude::*;
use crate::api;
use crate::types::*;
use crate::components::leaderboard::LeaderboardTable;

#[function_component(LeaderboardPage)]
pub fn leaderboard_page() -> Html {
    let entries = use_state(|| Vec::<LeaderboardEntry>::new());
    let projects = use_state(|| Vec::<Project>::new());
    let selected_project = use_state(|| None::<i64>);
    let loading = use_state(|| true);

    {
        let entries = entries.clone();
        let projects = projects.clone();
        let selected_project = selected_project.clone();
        let loading = loading.clone();

        use_effect_with((*selected_project).clone(), move |pid| {
            let pid = pid.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                if let Ok(p) = api::list_projects().await {
                    projects.set(p);
                }
                if let Ok(lb) = api::get_leaderboard(pid, 50).await {
                    entries.set(lb);
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_filter_change = {
        let selected_project = selected_project.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let val: i64 = input.value().parse().unwrap_or(0);
            selected_project.set(if val == 0 { None } else { Some(val) });
        })
    };

    html! {
        <div class="leaderboard-page">
            <h1>{"贡献者排行榜"}</h1>
            <div class="filter-bar">
                <select onchange={on_filter_change}>
                    <option value="0">{"全部项目"}</option>
                    { for (*projects).iter().map(|p| {
                        html! {
                            <option value={p.id.to_string()}>
                                { format!("{}/{}", p.owner, p.repo) }
                            </option>
                        }
                    })}
                </select>
            </div>
            { if *loading {
                html! { <div class="loading">{"加载中..."}</div> }
            } else if entries.is_empty() {
                html! { <p class="empty">{"暂无数据，请先同步 GitHub 数据"}</p> }
            } else {
                html! { <LeaderboardTable entries={(*entries).clone()} /> }
            }}
        </div>
    }
}
