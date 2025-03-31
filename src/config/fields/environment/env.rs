use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Env {
    Default,
    Dev,
    Stage,
    Lab,
    Prod,
}

impl<'de> Deserialize<'de> for Env {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.to_lowercase().as_str() {
            "dev" => Env::Dev,
            "stage" => Env::Stage,
            "lab" => Env::Lab,
            "prod" => Env::Prod,
            "default" => Env::Default,
            _ => Env::Default,
        })
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::Default
    }
}

impl Env {
    pub fn into_inner(self) -> String {
        self.into()
    }

    pub fn new() -> Option<Self> {
        Option::Some(Env::Default)
    }
}

impl From<Env> for String {
    fn from(value: Env) -> Self {
        match value {
            Env::Default => "default".to_owned(),
            Env::Dev => "deb".to_owned(),
            Env::Stage => "stage".to_owned(),
            Env::Lab => "lab".to_owned(),
            Env::Prod => "prod".to_owned(),
        }
    }
}
