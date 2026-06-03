use yew::prelude::*;
use crate::api;
use crate::types::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user_id: i64,
}

#[function_component(ProfilePage)]
pub fn profile_page(props: &Props) -> Html {
    let user_id = props.user_id;
    let score = use_state(|| None::<Score>);
    let prs = use_state(|| Vec::<PullRequest>::new());
    let issues = use_state(|| Vec::<Issue>::new());
    let loading = use_state(|| true);

    {
        let score = score.clone();
        let prs = prs.clone();
        let issues = issues.clone();
        let loading = loading.clone();
        use_effect_with(user_id, move |uid| {
            let uid = *uid;
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(s) = api::get_user_score(uid).await {
                    score.set(s);
                }
                if let Ok(p) = api::get_user_prs(uid).await {
                    prs.set(p);
                }
                if let Ok(i) = api::get_user_issues(uid).await {
                    issues.set(i);
                }
                loading.set(false);
            });
            || ()
        });
    }

    if *loading {
        return html! { <div class="loading">{"加载中..."}</div> };
    }

    html! {
        <div class="profile-page">
            <h1>{"贡献者详情"}</h1>

            { if let Some(ref s) = *score {
                html! {
                    <div class="score-breakdown">
                        <div class="score-item">
                            <span class="score-label">{"代码行数分"}</span>
                            <span class="score-value">{ s.lines_changed_score }</span>
                        </div>
                        <div class="score-item">
                            <span class="score-label">{"评论分"}</span>
                            <span class="score-value">{ s.comments_score }</span>
                        </div>
                        <div class="score-item">
                            <span class="score-label">{"奖励分"}</span>
                            <span class="score-value">{ s.bonus_score }</span>
                        </div>
                        <div class="score-item total">
                            <span class="score-label">{"总分"}</span>
                            <span class="score-value">{ s.total_score }</span>
                        </div>
                    </div>
                }
            } else {
                html! { <p>{"暂无分数数据"}</p> }
            }}

            <section class="section">
                <h2>{ format!("Pull Requests ({})", prs.len()) }</h2>
                <table class="contrib-table">
                    <thead>
                        <tr>
                            <th>{"标题"}</th>
                            <th>{"状态"}</th>
                            <th>{"+ 行"}</th>
                            <th>{"- 行"}</th>
                            <th>{"日期"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (*prs).iter().map(|pr| {
                            html! {
                                <tr>
                                    <td>{ &pr.title }</td>
                                    <td class={format!("state-{}", pr.state)}>{ &pr.state }</td>
                                    <td class="additions">{ format!("+{}", pr.additions) }</td>
                                    <td class="deletions">{ format!("-{}", pr.deletions) }</td>
                                    <td>{ &pr.created_at[..10] }</td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </section>

            <section class="section">
                <h2>{ format!("Issues ({})", issues.len()) }</h2>
                <table class="contrib-table">
                    <thead>
                        <tr>
                            <th>{"标题"}</th>
                            <th>{"状态"}</th>
                            <th>{"评论数"}</th>
                            <th>{"日期"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (*issues).iter().map(|issue| {
                            html! {
                                <tr>
                                    <td>{ &issue.title }</td>
                                    <td class={format!("state-{}", issue.state)}>{ &issue.state }</td>
                                    <td>{ issue.comments_count }</td>
                                    <td>{ &issue.created_at[..10] }</td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </section>
        </div>
    }
}
