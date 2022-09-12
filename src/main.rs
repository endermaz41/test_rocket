use rocket::{
    get, launch,
    response::status,
    routes,
    serde::json::{json, Value},
};

#[get("/transactions?<id>")]
pub async fn transactions(id: &str) -> status::Accepted<Value> {
    dbg!(id);
    status::Accepted(Some(json!({})))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![transactions])
}
