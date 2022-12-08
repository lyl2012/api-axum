use serde::Serialize;
#[derive(Serialize, Debug)]
pub struct AppResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(1, "success".to_string(), Some(data))
    }

    pub fn err(code: i32, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}
impl<T> axum::response::IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
