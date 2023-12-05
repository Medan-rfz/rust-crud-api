use actix_web::{get, post, HttpResponse, Responder, http, web};
use crate::infrastructure::repositories::abstract_repository::Repository;
use crate::infrastructure::repositories::users_repository::UserRepository;
use crate::domain::entities::user::User;
use crate::add_base_api_url;

macro_rules! add_controller_url {
    ($path:expr) => {
        concat!(add_base_api_url!(""), $path)
    };
}

#[derive(serde::Deserialize)]
struct UserId {
    id: i32,
}

#[utoipa::path(
    post, 
    path = add_controller_url!("/create_user"),
    request_body = User
)]
#[post("/create_user")]
pub async fn create_user(req_body: String) -> impl Responder {
    match serde_json::from_str::<User>(&req_body) {
        Ok(user) => {
            let mut status: bool = false;
            let result = UserRepository::new().await;

            match result {
                Ok(mut repos) => {
                    repos.add(user).await;
                    status = true;
                }
                Err(err) => {
                    println!("{}", err);
                }
            }

            if status {
                return HttpResponse::Ok().body("");
            }
            else {
                return HttpResponse::new(http::StatusCode::from_u16(500).unwrap());
            }
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {:?}", err);
            return HttpResponse::BadRequest().body("");
        }
    }
}

#[utoipa::path(
    get, 
    path = add_controller_url!("/get_user_by_id/"), 
    params(("id" = i32, Query,))
)]
#[get("/get_user_by_id/")]
pub async fn get_user_by_id(query: web::Query<UserId>) -> impl Responder {
    let user_id = query.id;
    let result = UserRepository::new().await;

    if result.is_ok() {
        let repos = result.unwrap();
        let user = repos.get_by_id(user_id).await;
        if user.is_some() {
            let mut resp: String = String::from("");
            match serde_json::to_string(&user) {
                Ok(json_string) => {
                    resp = json_string;
                }
                Err(_) => {}
            }
            return HttpResponse::Ok().json(resp);
        }
        else {
            return HttpResponse::NotFound().body("");
        }
    }
    else {
        return HttpResponse::BadRequest().body("");
    }
}

#[utoipa::path(
    post, 
    path = add_controller_url!("/update_user_by_id/"), 
    params(("id" = i32, Query,)),
    request_body = User
)]
#[post("/update_user_by_id/")]
pub async fn update_user_by_id(query: web::Query<UserId>, req_body: String) -> impl Responder {
    let user_id = query.id;
    let result = UserRepository::new().await;

    if result.is_ok() {
        let mut repos = result.unwrap();
        let parse_res = serde_json::from_str::<User>(&req_body);

        if parse_res.is_ok() && repos.update(user_id, parse_res.unwrap()).await {
            return HttpResponse::Ok().body("");
        }
    }

    return HttpResponse::BadRequest().body("");
}

#[utoipa::path(
    get, 
    path = add_controller_url!("/delete_user_by_id/"),
    params(("id" = i32, Query,))
)]
#[get("/delete_user_by_id/")]
pub async fn delete_user_by_id(query: web::Query<UserId>) -> impl Responder {
    let user_id = query.id;
    let result = UserRepository::new().await;

    if result.is_ok() {
        let mut repos = result.unwrap();
        if repos.remove(user_id).await {
            return HttpResponse::Ok().body("");
        }
    }

    return HttpResponse::BadRequest().body("");
}
