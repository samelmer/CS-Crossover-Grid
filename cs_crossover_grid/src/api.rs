use rocket::http::{ContentType, Header};

#[derive(Responder)]
#[response(content_type = "json")]
pub struct ApiResponse {
    pub body: String,
    pub content_type: ContentType,
    pub status: Header<'static>,
}
