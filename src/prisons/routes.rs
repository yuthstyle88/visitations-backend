use crate::api_error::ApiError;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::prisons::{Prisons, PrisonsMessage};
use crate::visitors::Visitors;

#[get("/list")]
async fn prisons_list_all() -> Result<HttpResponse, ApiError> {
    let users = Prisons::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/create")]
async fn prisons_create(user: web::Json<PrisonsMessage>) -> Result<HttpResponse, ApiError> {
    let user = Prisons::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/get/{id}")]
async fn prisons_get(params: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let prison = Prisons::find(params.into_inner())?;
    let mut visitors = vec![];
    if prison.first_name != "" {
        visitors  = Visitors::find_family_and_friend(prison.prison_id.clone())?;
    }
    Ok(HttpResponse::Ok().json(json!({ "prison": prison,"visitors": visitors })))
}


#[put("/update/{id}")]
async fn prisons_update(id: web::Path<String>, params: web::Json<PrisonsMessage>) -> Result<HttpResponse, ApiError> {
    let user = Prisons::update(id.into_inner(), params.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete/{id}")]
async fn prisons_delete(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Prisons::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/prisons")
            .service(prisons_list_all)
            .service(prisons_get)
            .service(prisons_create)
            .service(prisons_update)
            .service(prisons_delete)
    );
}
