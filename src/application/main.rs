use actix_web::{web, App, HttpServer};
use container_hive::domain::repositories::ContainerRepository;
use container_hive::domain::services::ContainerService;
use container_hive::domain::use_cases::ManageContainer;
use container_hive::infrastructure::models::ContainerModel;
use container_hive::infrastructure::controllers::ContainerController;

mod domain;
mod infrastructure;
mod application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let container_repository = ContainerModel::new();
    let container_service = ContainerService::new(container_repository);
    let manage_container = ManageContainer::new(container_service);
    let container_controller = ContainerController::new(manage_container);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(container_controller.clone()))
            .route("/containers", web::post().to(ContainerController::create_container))
            .route("/containers", web::get().to(ContainerController::get_all_containers))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
