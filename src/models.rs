#[derive(Debug, PartialEq)]
pub enum Status {
    PENDING,
    DONE,
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
}
