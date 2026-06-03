use yew::prelude::*;
use crate::components::settings::SettingsForm;

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    html! {
        <div class="settings-page">
            <h1>{"账号设置"}</h1>
            <SettingsForm />
        </div>
    }
}
