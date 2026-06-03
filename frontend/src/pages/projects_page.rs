use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::api;
use crate::types::*;
use crate::components::project_list::ProjectList;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    let projects = use_state(|| Vec::<Project>::new());
    let owner = use_state(|| String::new());
    let repo = use_state(|| String::new());
    let message = use_state(|| String::new());
    let loading = use_state(|| true);

    {
        let projects = projects.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(p) = api::list_projects().await {
                    projects.set(p);
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_submit = {
        let owner = owner.clone();
        let repo = repo.clone();
        let projects = projects.clone();
        let message = message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let owner = owner.clone();
            let repo = repo.clone();
            let projects = projects.clone();
            let message = message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let req = AddProjectRequest {
                    owner: (*owner).clone(),
                    repo: (*repo).clone(),
                };
                match api::add_project(&req).await {
                    Ok(_) => {
                        message.set("项目添加成功！".to_string());
                        if let Ok(p) = api::list_projects().await {
                            projects.set(p);
                        }
                    }
                    Err(e) => message.set(format!("错误: {}", e)),
                }
            });
        })
    };

    let on_owner_input = {
        let owner = owner.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            owner.set(input.value());
        })
    };

    let on_repo_input = {
        let repo = repo.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            repo.set(input.value());
        })
    };

    html! {
        <div class="projects-page">
            <h1>{"项目管理"}</h1>

            <form class="add-project-form" onsubmit={on_submit}>
                <h3>{"添加跟踪项目"}</h3>
                <div class="form-row">
                    <input type="text" value={(*owner).clone()} oninput={on_owner_input}
                           placeholder="owner (e.g. rust-lang)" required=true />
                    <span>{"/"}</span>
                    <input type="text" value={(*repo).clone()} oninput={on_repo_input}
                           placeholder="repo (e.g. rust)" required=true />
                    <button type="submit">{"添加"}</button>
                </div>
                { if !message.is_empty() {
                    html! { <p class="form-message">{ &*message }</p> }
                } else {
                    html! {}
                }}
            </form>

            { if *loading {
                html! { <div class="loading">{"加载中..."}</div> }
            } else {
                html! { <ProjectList projects={(*projects).clone()} /> }
            }}
        </div>
    }
}
