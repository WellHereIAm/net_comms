use std::collections::HashMap;

use library::prelude::User;

pub struct Users {
    database: HashMap<String, User>,
    index: Vec<usize>,
}
