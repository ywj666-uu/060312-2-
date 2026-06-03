use yew::prelude::*;
use crate::types::LeaderboardEntry;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entries: Vec<LeaderboardEntry>,
}

#[function_component(LeaderboardTable)]
pub fn leaderboard_table(props: &Props) -> Html {
    html! {
        <table class="leaderboard-table">
            <thead>
                <tr>
                    <th>{"排名"}</th>
                    <th>{"贡献者"}</th>
                    <th>{"代码分"}</th>
                    <th>{"评论分"}</th>
                    <th>{"奖励分"}</th>
                    <th>{"总分"}</th>
                </tr>
            </thead>
            <tbody>
                { for props.entries.iter().map(|entry| {
                    let display = entry.display_name.clone()
                        .unwrap_or_else(|| entry.github_username.clone());
                    html! {
                        <tr class={if entry.rank <= 3 { "top-rank" } else { "" }}>
                            <td class="rank">{ format!("#{}", entry.rank) }</td>
                            <td class="user-cell">
                                { if let Some(ref url) = entry.avatar_url {
                                    html! { <img class="avatar" src={url.clone()} alt="" /> }
                                } else {
                                    html! {}
                                }}
                                <span class="username">{ &display }</span>
                            </td>
                            <td>{ entry.lines_changed_score }</td>
                            <td>{ entry.comments_score }</td>
                            <td>{ entry.bonus_score }</td>
                            <td class="total-score">{ entry.total_score }</td>
                        </tr>
                    }
                })}
            </tbody>
        </table>
    }
}
