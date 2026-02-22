use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use octocrab::Octocrab;
use tokio::sync::mpsc;

use crate::Error;

pub struct Stats {
    pub repos: Vec<Repo>,
}

impl Stats {
    pub async fn fetch(
        oct: &Octocrab,
        repos: Vec<(String, String)>,
        progress: mpsc::Sender<(usize, usize)>,
        min_stars: Option<u32>,
    ) -> Result<Stats, Error> {
        let total = repos.len();
        let mut results = Vec::new();
        for (i, (owner, repo)) in repos.iter().enumerate() {
            if let Some(stat) = Repo::fetch(oct, owner, repo).await?
                && min_stars.is_none_or(|min| stat.stars >= min)
            {
                results.push(stat);
            }
            let _ = progress.send((i + 1, total)).await;
        }
        Ok(Stats { repos: results })
    }
}

pub struct Repo {
    pub owner: String,
    pub name: String,
    pub stars: u32,
    pub forks: u32,
    /// Age
    pub created_at: DateTime<Utc>,
    /// Last Update
    pub pushed_at: DateTime<Utc>,
    pub license: String,
    pub description: Option<String>,
    pub topics: Vec<String>,
    pub homepage: Option<String>,
}

impl Repo {
    pub fn ref_array(&self) -> [String; 6] {
        [
            self.name.clone(),                            // Name
            Self::prettify_num(self.stars),               // Stars
            Self::prettify_num(self.forks),               // Forks
            self.license.clone(),                         // License
            HumanTime::from(self.created_at).to_string(), // Age
            HumanTime::from(self.pushed_at).to_string(),  // Updated
        ]
    }

    pub async fn fetch(oct: &Octocrab, owner: &str, name: &str) -> Result<Option<Repo>, Error> {
        let info = match oct.repos(owner, name).get().await {
            Ok(info) => info,
            Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 404 => {
                return Ok(None); // skip silently
            }
            Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 403 => {
                return Err(Error::RateLimit);
            }
            Err(e) => return Err(Error::GitHub(e)),
        };
        let stars = info.stargazers_count.unwrap_or(0);
        let forks = info.forks_count.unwrap_or(0);
        let license = info
            .license
            .as_ref()
            .map(|l| l.key.clone())
            .unwrap_or_else(|| "None".to_string());
        let age = info.created_at.unwrap();
        let last_push = info.pushed_at.unwrap();
        let description = info.description;
        let topics = info.topics.unwrap_or_default();
        let homepage = info.homepage;

        Ok(Some(Repo {
            owner: owner.to_string(),
            name: name.to_string(),
            stars,
            forks,
            license,
            created_at: age,
            pushed_at: last_push,
            description,
            topics,
            homepage,
        }))
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
}
