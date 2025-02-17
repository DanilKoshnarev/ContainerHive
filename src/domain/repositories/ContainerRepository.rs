use crate::domain::entities::Container;

pub trait ContainerRepository {
    fn save(&self, container: Container);
    fn find_by_id(&self, id: &str) -> Option<Container>;
    fn find_all(&self) -> Vec<Container>;
    fn delete(&self, id: &str);
}
