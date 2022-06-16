use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Ballot {
    pub cad: String,
    pub floater: f32,
}

