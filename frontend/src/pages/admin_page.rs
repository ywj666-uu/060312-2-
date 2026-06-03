use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::api;
use crate::types::*;
use crate::components::bonus_form::BonusForm;

#[function_component(AdminPage)]
pub fn admin_page() -> Html {
    let users = use_state(|| Vec::<UserResponse>::new());
    let token = use_state(|| String::new());
    let is_logged_in = use_state(|| false);
    let login_error = use_state(|| String::new());
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let loading = use_state(|| true);

    {
        let users = users.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(u) = api::list_users().await {
                    users.set(u);
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_login = {
        let username = username.clone();
        let password = password.clone();
        let token = token.clone();
        let is_logged_in = is_logged_in.clone();
        let login_error = login_error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username.clone();
            let password = password.clone();
            let token = token.clone();
            let is_logged_in = is_logged_in.clone();
            let login_error = login_error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let req = LoginRequest {
                    username: (*username).clone(),
                    password: (*password).clone(),
                };
                match api::login(&req).await {
                    Ok(resp) => {
                        if resp.is_maintainer {
                            token.set(resp.token);
                            is_logged_in.set(true);
                            login_error.set(String::new());
                        } else {
                            login_error.set("该用户不是维护者，无权操作".to_string());
                        }
                    }
                    Err(e) => login_error.set(format!("登录失败: {}", e)),
                }
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

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    if *loading {
        return html! { <div class="loading">{"加载中..."}</div> };
    }

    if !*is_logged_in {
        return html! {
            <div class="admin-page">
                <h1>{"管理面板"}</h1>
                <form class="settings-form" onsubmit={on_login}>
                    <h2>{"维护者登录"}</h2>
                    <p class="form-hint">{"只有维护者身份的用户才能发放奖励分"}</p>
                    <div class="form-group">
                        <label>{"用户名"}</label>
                        <input type="text" value={(*username).clone()} oninput={on_username_input}
                               placeholder="github-username" required=true />
                    </div>
                    <div class="form-group">
                        <label>{"密码"}</label>
                        <input type="password" value={(*password).clone()} oninput={on_password_input}
                               placeholder="password" required=true />
                    </div>
                    <button type="submit">{"登录"}</button>
                    { if !login_error.is_empty() {
                        html! { <p class="form-message" style="color: var(--danger);">{ &*login_error }</p> }
                    } else {
                        html! {}
                    }}
                </form>
            </div>
        };
    }

    html! {
        <div class="admin-page">
            <h1>{"管理面板"}</h1>
            <BonusForm users={(*users).clone()} token={(*token).clone()} />
        </div>
    }
}
