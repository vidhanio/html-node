#[cfg(feature = "axum")]
mod axum {
    use axum::response::{Html, IntoResponse, Response};

    use crate::{pretty::Pretty, Node};

    impl IntoResponse for Node {
        fn into_response(self) -> Response {
            Html(self.to_string()).into_response()
        }
    }

    impl IntoResponse for Pretty {
        fn into_response(self) -> Response {
            Html(self.to_string()).into_response()
        }
    }
}
