#[derive(Debug)]
pub enum Status {
    PENNDING,
    DONE,
}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
}
