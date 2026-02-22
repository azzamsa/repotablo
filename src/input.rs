pub fn open_editor() -> Result<Vec<(String, String)>, crate::Error> {
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
