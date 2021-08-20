use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use ron::ser::{self, PrettyConfig};
use ron::de;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnchecked{
    pub username: String,
    pub password: String,
}