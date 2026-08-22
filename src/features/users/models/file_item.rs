use uuid::Uuid;

#[derive(Clone, PartialEq, Debug)]
pub struct FileItem {
    pub id: Uuid,
    pub name: String,
    pub size: f64,
}
