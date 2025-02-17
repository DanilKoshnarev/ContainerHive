pub struct ContainerConfig {
    pub id: String,
    pub container_id: String,
    pub config_data: String,
}

impl ContainerConfig {
    pub fn new(id: String, container_id: String, config_data: String) -> Self {
        Self { id, container_id, config_data }
    }
}
