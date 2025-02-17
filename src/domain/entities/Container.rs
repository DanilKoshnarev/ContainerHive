pub struct Container {
    pub id: String,
    pub name: String,
    pub status: String,
    pub image: String,
}

impl Container {
    pub fn new(id: String, name: String, status: String, image: String) -> Self {
        Self { id, name, status, image }
    }
}
