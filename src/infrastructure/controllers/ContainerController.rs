use actix_web::{web, HttpResponse};
use crate::domain::use_cases::ManageContainer;
use serde::Deserialize;

pub struct ContainerController {
    manage_container: ManageContainer,
}

#[derive(Deserialize)]
pub struct CreateContainerRequest {
    pub id: String,
    pub name: String,
    pub status: String,
    pub image: String,
}

impl ContainerController {
    pub fn new(manage_container: ManageContainer) -> Self {
        Self { manage_container }
    }

    pub async fn create_container(
        manage_container: web::Data<Self>,
        request: web::Json<CreateContainerRequest>,
    ) -> HttpResponse {
        manage_container.manage_container.create_container(
            request.id.clone(),
            request.name.clone(),
            request.status.clone(),
            request.image.clone(),
        );
        HttpResponse::Ok().finish()
    }

    pub async fn get_all_containers(
        manage_container: web::Data<Self>,
    ) -> HttpResponse {
        let containers = manage_container.manage_container.view_all_containers();
        HttpResponse::Ok().json(containers)
    }
}
