use actix_web::get;

#[get("/test")]
pub async fn test() -> &'static str {
    "foo"
}
