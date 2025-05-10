use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Claims {
    pub name: String,
    pub email: String,
    pub exp: usize,
}
