use anyhow::Result;
use iocraft::prelude::*;

#[derive(Clone, Debug, Default)]
struct Repo {
    name: String,
    stars: i32,
    contributors: i32,
    activity: String,
}

impl Repo {
    fn new(name: &str, stars: i32, contributors: i32, activity: &str) -> Self {
        Self {
            name: name.to_string(),
            stars,
            contributors,
            activity: activity.to_string(),
        }
    }
    fn fetch() -> Result<Vec<Self>> {
        let repos = vec![
            Repo::new("axum", 100, 403, "301/y"),
            Repo::new("actix-web", 200, 75, "100/y"),
            Repo::new("rocket", 500, 85, "80/y"),
        ];
        Ok(repos)
    }
}

#[derive(Default, Props)]
struct ReposTableProps {
    repos: Vec<Repo>,
}

#[component]
fn ReposTableView(props: &ReposTableProps) -> impl Into<AnyElement<'static>> {
    element! {
        View(
            width: 100pct,
            flex_direction: FlexDirection::Column,
            border_style: BorderStyle::Round,
        ) {
            View(border_style: BorderStyle::Single, border_edges: Edges::Bottom, border_color: Color::Grey) {
                View(width: 20pct, justify_content: JustifyContent::End, padding_right: 2) {
                    Text(content: "Name", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }
                View(width: 20pct) {
                    Text(content: "Stars", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }
                View(width: 20pct) {
                    Text(content: "Contributors", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }
                View(width: 20pct) {
                    Text(content: "Activity", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }
            }

            #(props.repos.iter().enumerate().map(|(i, repo)| element! {
                View(background_color: if i % 2 == 0 { None } else { Some(Color::DarkGrey) }) {
                    View(width: 20pct, justify_content: JustifyContent::End, padding_right: 2) {
                        Text(content: repo.name.to_string())
                    }
                    View(width: 20pct) {
                        Text(content: repo.stars.to_string())
                    }
                    View(width: 20pct) {
                        Text(content: repo.contributors.to_string())
                    }
                    View(width: 20pct) {
                        Text(content: repo.activity.to_string())
                    }
                }
            }))
        }
    }
}

#[component]
fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);
    let mut repos = hooks.use_state(|| Repo::fetch().unwrap_or_default());

    let mut sort_by_name = move || {
        let mut sorted = repos.read().clone();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        repos.set(sorted);
    };

    let mut sort_by_stars = move || {
        let mut sorted = repos.read().clone();
        sorted.sort_by(|a, b| b.stars.cmp(&a.stars));
        repos.set(sorted);
    };

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Char('q') => should_exit.set(true),
                    KeyCode::Char('1') => sort_by_name(),
                    KeyCode::Char('2') => sort_by_stars(),
                    _ => {}
                }
            }
            _ => {}
        }
    });

    if should_exit.get() {
        system.exit();
    }

    element! {
        View(width, height, flex_direction: FlexDirection::Column) {
            View(flex_grow: 1.0) {
                ReposTableView(repos: repos.read().clone())
            }
            View(
                border_style: BorderStyle::Single,
                border_color: Color::DarkGrey,
                border_edges: Edges::Top,
                padding_left: 1,
            ) {
                Text(content: "Sort by: [1] Name · [2] Stars . [Q] Quit")
            }
        }
    }
}

fn main() {
    smol::block_on(element!(App).render_loop()).unwrap();
}
