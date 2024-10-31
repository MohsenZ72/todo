#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Status {
    ToDo,
    Pending,
    InProgress,
    Done,
}
#[derive(Debug)]

pub enum DoDate {
    Past,
    Today,
    Tomorrow,
    Later,
}
