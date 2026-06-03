use yew::prelude::*;
use crate::types::UserResponse;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user: UserResponse,
}

#[function_component(UserCard)]
pub fn user_card(props: &Props) -> Html {
    let user = &props.user;
    let display = user.display_name.clone()
        .unwrap_or_else(|| user.github_username.clone());

    html! {
        <div class="user-card">
            { if let Some(ref url) = user.avatar_url {
                html! { <img class="user-card-avatar" src={url.clone()} alt="" /> }
            } else {
                html! { <div class="user-card-avatar placeholder"></div> }
            }}
            <div class="user-card-info">
                <h3>{ &display }</h3>
                <p class="github-name">{ format!("@{}", user.github_username) }</p>
                <p class="score">{ format!("总分: {}", user.total_score) }</p>
            </div>
        </div>
    }
}
