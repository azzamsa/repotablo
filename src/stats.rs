use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use octocrab::Octocrab;
use tokio::sync::mpsc;

use crate::Error;

pub struct ReposStats {
    pub repos: Vec<RepoStats>,
}

impl ReposStats {
    pub async fn fetch(
        oct: &Octocrab,
        repos: Vec<(String, String)>,
        progress: mpsc::Sender<(usize, usize)>,
    ) -> Result<ReposStats, Error> {
        let total = repos.len();
        let mut results = Vec::new();
        for (i, (owner, repo)) in repos.iter().enumerate() {
            let stat = RepoStats::fetch(oct, owner, repo).await?;
            results.push(stat);
            let _ = progress.send((i + 1, total)).await;
        }
        Ok(ReposStats { repos: results })
    }
}

pub struct RepoStats {
    pub owner: String,
    pub name: String,
    pub stars: u32,
    pub forks: u32,
    /// Age
    pub created_at: DateTime<Utc>,
    /// Last Update
    pub pushed_at: DateTime<Utc>,
    pub license: String,
}

impl RepoStats {
    pub fn ref_array(&self) -> [String; 6] {
        [
            self.name.clone(),                            // Name
            prettify_num(self.stars),                     // Stars
            prettify_num(self.forks),                     // Forks
            self.license.clone(),                         // License
            HumanTime::from(self.created_at).to_string(), // Age
            HumanTime::from(self.pushed_at).to_string(),  // Updated
        ]
    }

    pub async fn fetch(oct: &Octocrab, owner: &str, name: &str) -> Result<RepoStats, Error> {
        let repo = oct.repos(owner, name);
        let info = repo.get().await.map_err(|e| match &e {
            octocrab::Error::GitHub { source, .. } if source.status_code == 403 => Error::RateLimit,
            _ => Error::GitHub(e),
        })?;
        let stars = info.stargazers_count.unwrap_or(0);
        let forks = info.forks_count.unwrap_or(0);
        let license = info
            .license
            .as_ref()
            .map(|l| l.key.clone())
            .unwrap_or_else(|| "None".to_string());
        let age = info.created_at.unwrap();
        let last_push = info.pushed_at.unwrap();

        Ok(RepoStats {
            owner: owner.to_string(),
            name: name.to_string(),
            stars,
            forks,
            license,
            created_at: age,
            pushed_at: last_push,
        })
    }
}

fn prettify_num(stars: u32) -> String {
    if stars >= 1_000_000 {
        format!("{:.1}M", stars as f32 / 1_000_000.0)
    } else if stars >= 1_000 {
        format!("{:.1}k", stars as f32 / 1_000.0)
    } else {
        stars.to_string()
    }
}
