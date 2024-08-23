#[utoipa::path(
    get,
    path = "/test/",
    responses(
        (status = 200, description = "running test OpenAPI")
    )
)]

pub async fn ping() -> &'static str{
    "Hello world"
}