use yew::prelude::*;
use crate::types::Project;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub projects: Vec<Project>,
}

#[function_component(ProjectList)]
pub fn project_list(props: &Props) -> Html {
    html! {
        <div class="project-list">
            { for props.projects.iter().map(|p| {
                html! {
                    <div class="project-card">
                        <div class="project-header">
                            <h3>{ format!("{}/{}", p.owner, p.repo) }</h3>
                            { if let Some(ref lang) = p.language {
                                html! { <span class="language-tag">{ lang }</span> }
                            } else {
                                html! {}
                            }}
                        </div>
                        { if let Some(ref desc) = p.description {
                            html! { <p class="project-desc">{ desc }</p> }
                        } else {
                            html! {}
                        }}
                        <div class="project-stats">
                            <span class="hotness-badge">{ format!("🔥 热度 {}", p.hotness) }</span>
                            <span>{ format!("⭐ {}", p.stars) }</span>
                            <span>{ format!("🍴 {}", p.forks) }</span>
                            <span>{ format!("📋 {}", p.open_issues) }</span>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
