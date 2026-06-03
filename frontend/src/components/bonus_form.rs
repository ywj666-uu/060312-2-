use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::api;
use crate::types::{BonusRequest, UserResponse};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub users: Vec<UserResponse>,
    pub token: String,
}

#[function_component(BonusForm)]
pub fn bonus_form(props: &Props) -> Html {
    let selected_user = use_state(|| 0i64);
    let points = use_state(|| String::new());
    let reason = use_state(|| String::new());
    let message = use_state(|| String::new());
    let token = props.token.clone();

    let on_submit = {
        let selected_user = selected_user.clone();
        let points = points.clone();
        let reason = reason.clone();
        let message = message.clone();
        let token = token.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let selected_user = selected_user.clone();
            let points = points.clone();
            let reason = reason.clone();
            let message = message.clone();
            let token = token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let pts: i64 = points.parse().unwrap_or(0);
                if pts == 0 || *selected_user == 0 {
                    message.set("请填写完整信息".to_string());
                    return;
                }
                let req = BonusRequest {
                    user_id: *selected_user,
                    project_id: None,
                    points: pts,
                    reason: (*reason).clone(),
                };
                match api::award_bonus(&token, &req).await {
                    Ok(_) => message.set("奖励分已发放！".to_string()),
                    Err(e) => message.set(format!("错误: {}", e)),
                }
            });
        })
    };

    let on_user_change = {
        let selected_user = selected_user.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            selected_user.set(input.value().parse().unwrap_or(0));
        })
    };

    let on_points_input = {
        let points = points.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            points.set(input.value());
        })
    };

    let on_reason_input = {
        let reason = reason.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            reason.set(input.value());
        })
    };

    html! {
        <form class="bonus-form" onsubmit={on_submit}>
            <h2>{"发放特殊贡献分"}</h2>
            <div class="form-group">
                <label>{"选择贡献者"}</label>
                <select onchange={on_user_change}>
                    <option value="0">{"-- 请选择 --"}</option>
                    { for props.users.iter().map(|u| {
                        html! {
                            <option value={u.id.to_string()}>
                                { &u.github_username }
                            </option>
                        }
                    })}
                </select>
            </div>
            <div class="form-group">
                <label>{"分数"}</label>
                <input type="number" value={(*points).clone()} oninput={on_points_input}
                       placeholder="10" required=true />
            </div>
            <div class="form-group">
                <label>{"原因"}</label>
                <input type="text" value={(*reason).clone()} oninput={on_reason_input}
                       placeholder="出色的代码审查" required=true />
            </div>
            <button type="submit">{"发放奖励"}</button>
            { if !message.is_empty() {
                html! { <p class="form-message">{ &*message }</p> }
            } else {
                html! {}
            }}
        </form>
    }
}
