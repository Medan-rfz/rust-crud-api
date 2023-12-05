mod api_doc;
mod interfaces;
mod infrastructure;
mod domain;
mod settings;

use actix_web::{App, HttpServer, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api_doc::ApiDoc;
use crate::interfaces::api_controlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                .service(create_user)
                .service(get_user_by_id)
                .service(update_user_by_id)
                .service(delete_user_by_id)
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind("localhost:7777")?
    .run()
    .await
}
