use itertools::Itertools;

pub struct Repo {
    pub name: String,
    pub stars: String,
}

impl Repo {
    pub const fn ref_array(&self) -> [&String; 2] {
        [&self.name, &self.stars]
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stars(&self) -> &str {
        &self.stars
    }
}

pub fn generate_fake_names() -> Vec<Repo> {
    use fakeit::{contact, name};

    (0..3)
        .map(|_| {
            let name = name::full();
            let stars = contact::email();

            Repo { name, stars }
        })
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .collect()
}
