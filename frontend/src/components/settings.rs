use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::api;
use crate::types::CreateUserRequest;

#[function_component(SettingsForm)]
pub fn settings_form() -> Html {
    let username = use_state(|| String::new());
    let pat = use_state(|| String::new());
    let display_name = use_state(|| String::new());
    let message = use_state(|| String::new());
    let loading = use_state(|| false);

    let on_submit = {
        let username = username.clone();
        let pat = pat.clone();
        let display_name = display_name.clone();
        let message = message.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username.clone();
            let pat = pat.clone();
            let display_name = display_name.clone();
            let message = message.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let req = CreateUserRequest {
                    github_username: (*username).clone(),
                    github_pat: (*pat).clone(),
                    display_name: if display_name.is_empty() { None } else { Some((*display_name).clone()) },
                };
                match api::create_user(&req).await {
                    Ok(_) => message.set("绑定成功！".to_string()),
                    Err(e) => message.set(format!("错误: {}", e)),
                }
                loading.set(false);
            });
        })
    };

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_pat_input = {
        let pat = pat.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            pat.set(input.value());
        })
    };

    let on_name_input = {
        let display_name = display_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            display_name.set(input.value());
        })
    };

    html! {
        <form class="settings-form" onsubmit={on_submit}>
            <h2>{"绑定 GitHub 账号"}</h2>
            <div class="form-group">
                <label>{"GitHub 用户名"}</label>
                <input type="text" value={(*username).clone()} oninput={on_username_input}
                       placeholder="your-github-username" required=true />
            </div>
            <div class="form-group">
                <label>{"Personal Access Token"}</label>
                <input type="password" value={(*pat).clone()} oninput={on_pat_input}
                       placeholder="ghp_xxxxxxxxxxxx" required=true />
            </div>
            <div class="form-group">
                <label>{"显示名称（可选）"}</label>
                <input type="text" value={(*display_name).clone()} oninput={on_name_input}
                       placeholder="显示名称" />
            </div>
            <button type="submit" disabled={*loading}>
                { if *loading { "绑定中..." } else { "绑定账号" } }
            </button>
            { if !message.is_empty() {
                html! { <p class="form-message">{ &*message }</p> }
            } else {
                html! {}
            }}
        </form>
    }
}
