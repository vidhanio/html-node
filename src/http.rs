#[cfg(feature = "axum")]
mod axum {
    use axum::response::{Html, IntoResponse, Response};

    use crate::Node;

    impl IntoResponse for Node {
        fn into_response(self) -> Response {
            Html(self.to_string()).into_response()
        }
    }
}
