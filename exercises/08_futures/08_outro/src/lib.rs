// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.
mod model;
mod routes;
mod store;

pub use routes::router;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{header, Method, Request, StatusCode};
    use axum::Router;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    async fn send(app: &Router, method: Method, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
        let builder = Request::builder().method(method).uri(uri);
        let request = match body {
            Some(body) => builder
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string())),
            None => builder.body(Body::empty()),
        }
        .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, serde_json::from_slice(&bytes).unwrap_or_default())
    }

    fn draft() -> Value {
        json!({ "title": "A title", "description": "A description" })
    }

    #[tokio::test]
    async fn create_get_and_patch() {
        let app = router();

        let (status, created) = send(&app, Method::POST, "/tickets", Some(draft())).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(created["status"], "ToDo");
        let uri = format!("/tickets/{}", created["id"]);

        let (status, fetched) = send(&app, Method::GET, &uri, None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(fetched, created);

        let patch = json!({ "title": "New title", "status": "InProgress" });
        let (status, patched) = send(&app, Method::PATCH, &uri, Some(patch)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(patched["title"], "New title");
        assert_eq!(patched["description"], "A description");
        assert_eq!(patched["status"], "InProgress");

        let (_, fetched) = send(&app, Method::GET, &uri, None).await;
        assert_eq!(fetched, patched);
    }

    #[tokio::test]
    async fn unknown_ticket_is_not_found() {
        let app = router();
        let (status, _) = send(&app, Method::GET, "/tickets/42", None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let patch = json!({ "status": "Done" });
        let (status, _) = send(&app, Method::PATCH, "/tickets/42", Some(patch)).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn invalid_input_is_rejected() {
        let app = router();
        let empty_title = json!({ "title": "", "description": "A description" });
        let (status, _) = send(&app, Method::POST, "/tickets", Some(empty_title)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        let (_, created) = send(&app, Method::POST, "/tickets", Some(draft())).await;
        let uri = format!("/tickets/{}", created["id"]);
        let patch = json!({ "title": "Valid", "description": "" });
        let (status, _) = send(&app, Method::PATCH, &uri, Some(patch)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        let (_, fetched) = send(&app, Method::GET, &uri, None).await;
        assert_eq!(fetched, created);
    }
}
