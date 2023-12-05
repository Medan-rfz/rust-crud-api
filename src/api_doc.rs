use serde::{Serialize, Deserialize};
use utoipa::{OpenApi, ToSchema};

#[macro_export]
macro_rules! add_base_api_url {
    ($path:expr) => {
        concat!("/api/v1", $path)
    };
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "API CRUD HTTP server",
        description  = "API CRUD HTTP server",
        contact(
            name = "Medan",
            email = "medan.develop@gmail.com"
        ),
        version = "0.1.1"
    ),
    components(
        schemas(
            Response
        )
    ),
    paths(
        crate::interfaces::api_controlers::create_user,
        crate::interfaces::api_controlers::get_user_by_id,
        crate::interfaces::api_controlers::update_user_by_id,
        crate::interfaces::api_controlers::delete_user_by_id,
    ),
)]
pub struct ApiDoc;
