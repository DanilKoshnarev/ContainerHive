use crate::domain::entities::Container;
use crate::domain::services::ContainerService;

pub struct ManageContainer {
    container_service: ContainerService,
}

impl ManageContainer {
    pub fn new(container_service: ContainerService) -> Self {
        Self { container_service }
    }

    pub fn create_container(&self, id: String, name: String, status: String, image: String) {
        let container = Container::new(id, name, status, image);
        self.container_service.add_container(container);
    }

    pub fn view_container(&self, id: &str) -> Option<Container> {
        self.container_service.get_container(id)
    }

    pub fn view_all_containers(&self) -> Vec<Container> {
        self.container_service.get_all_containers()
    }

    pub fn remove_container(&self, id: &str) {
        self.container_service.delete_container(id);
    }
}
