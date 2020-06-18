use std::collections::HashSet;

#[derive(Debug)]
pub struct Room {
    pub members: HashSet<usize>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            members: HashSet::new(),
        }
    }
}
