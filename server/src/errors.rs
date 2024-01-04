use std::fmt;

// 引入错误状态码，协助制定自定义错误
use ntex:: {
    http::StatusCode,
    web::{HttpResponse, WebResponseError},
};

#[derive(Debug, Clone)]
pub enum CustomError { 
    // 这个error需要实现WebResponseError这个trait的两个方法：status_code, error_response,
    // 还需要实现 fmt::Display
    NotFound(String),
    InternalServerError(String),
}

impl WebResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self, _: &ntex::web::HttpRequest) -> HttpResponse {
        // 错误返回response
        HttpResponse::new(self.status_code()).set_body(
            match self { // 其实就是解构出来
                Self::NotFound(e) => e,
                Self::InternalServerError(e) => e,
            }
            .into(),
        )
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NotFound(e) => write!(f, "{e}"),
            CustomError::InternalServerError(e) => write!(f, "{e}"),
        }
    }
}