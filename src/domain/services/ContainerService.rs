use crate::domain::entities::Container;
use crate::domain::repositories::ContainerRepository;

pub struct ContainerService<T: ContainerRepository> {
    repository: T,
}

impl<T: ContainerRepository> ContainerService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn add_container(&self, container: Container) {
        self.repository.save(container);
    }

    pub fn get_container(&self, id: &str) -> Option<Container> {
        self.repository.find_by_id(id)
    }

    pub fn get_all_containers(&self) -> Vec<Container> {
        self.repository.find_all()
    }

    pub fn delete_container(&self, id: &str) {
        self.repository.delete(id);
    }
}
