use crate::domain::entities::Container;
use crate::domain::repositories::ContainerRepository;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct ContainerModel {
    containers: Mutex<HashMap<String, Container>>,
}

impl ContainerModel {
    pub fn new() -> Self {
        Self {
            containers: Mutex::new(HashMap::new()),
        }
    }
}

impl ContainerRepository for ContainerModel {
    fn save(&self, container: Container) {
        let mut containers = self.containers.lock().unwrap();
        containers.insert(container.id.clone(), container);
    }

    fn find_by_id(&self, id: &str) -> Option<Container> {
        let containers = self.containers.lock().unwrap();
        containers.get(id).cloned()
    }

    fn find_all(&self) -> Vec<Container> {
        let containers = self.containers.lock().unwrap();
        containers.values().cloned().collect()
    }

    fn delete(&self, id: &str) {
        let mut containers = self.containers.lock().unwrap();
        containers.remove(id);
    }
}
