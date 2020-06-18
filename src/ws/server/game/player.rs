use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Player {
    pub id: usize,
    pub points: usize,
    pub chances: usize,
    pub is_active: bool,
    // pub connection: Rc<Connection>,
}

// impl Default for Player {
//     fn default() -> Self {
//         Player {
//             id: 0,
//             points: 0,
//         }
//     }
// }