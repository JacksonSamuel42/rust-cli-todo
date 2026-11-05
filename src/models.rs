#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    PENDING,
    DONE,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
}
