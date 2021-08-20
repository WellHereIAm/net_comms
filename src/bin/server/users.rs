use std::collections::HashMap;

use library::prelude::User;

pub struct _Users {
    database: HashMap<String, User>,
    index: Vec<usize>,
}
