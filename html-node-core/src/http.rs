#[cfg(feature = "axum")]
mod axum {
    use axum::response::{Html, IntoResponse, Response};

    #[cfg(feature = "pretty")]
    use crate::pretty::Pretty;
    use crate::Node;

    impl IntoResponse for Node {
        fn into_response(self) -> Response {
            Html(self.to_string()).into_response()
        }
    }

    #[cfg(feature = "pretty")]
    impl IntoResponse for Pretty {
        fn into_response(self) -> Response {
            Html(self.to_string()).into_response()
        }
    }
}
