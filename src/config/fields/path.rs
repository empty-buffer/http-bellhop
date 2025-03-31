use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path(String);

impl Default for Path {
    fn default() -> Self {
        Self("/".to_owned())
    }
}

impl Path {
    pub fn into_inner(self) -> String {
        self.0
    }
}
