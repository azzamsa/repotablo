use std::path::PathBuf;

use crate::Error;

enum Input {
    Url(String),
    File(PathBuf),
    Editor,
}

pub async fn get_repos(arg: Option<String>) -> Result<Vec<(String, String)>, Error> {
    match detect_input(arg) {
        Input::Url(url) => {
            let content = reqwest::get(&url).await?.text().await?;
            Ok(parse_repos(&content))
        }
        Input::File(path) => {
            let content = std::fs::read_to_string(path)?;
            Ok(parse_repos(&content))
        }
        Input::Editor => open_editor(),
    }
}

fn detect_input(arg: Option<String>) -> Input {
    match arg {
        Some(s) if s.starts_with("http://") || s.starts_with("https://") => Input::Url(s),
        Some(s) => Input::File(PathBuf::from(s)),
        None => Input::Editor,
    }
}

fn parse_repos(content: &str) -> Vec<(String, String)> {
    let re = regex::Regex::new(r"https://github\.com/([^/]+)/([^/)\s#]+)").unwrap();
    re.captures_iter(content)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect()
}

pub fn open_editor() -> Result<Vec<(String, String)>, Error> {
    let tmp = tempfile::NamedTempFile::new()?;
    // open $EDITOR
    let editor = std::env::var("EDITOR")?;
    std::process::Command::new(editor)
        .arg(tmp.path())
        .status()?;

    // read and parse
    let content = std::fs::read_to_string(tmp.path())?;
    let re = regex::Regex::new(r"https://github\.com/([^/]+)/([^/)\s]+)")?;
    let repos = re
        .captures_iter(&content)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect();

    Ok(repos)
}
