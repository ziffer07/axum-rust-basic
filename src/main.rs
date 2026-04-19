use askama::{Template};
use axum::{
    Router, extract::Query, http::StatusCode, response::{Html, IntoResponse, Response}, routing::get
};
use serde::{Serialize, Deserialize};

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T> 
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", error),
            )
            .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "../templates/main.html")]
struct MainTemplate{}

#[derive(Template)]
#[template(path = "../templates/echo.html")]
struct EchoTemplate {
    text: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
struct Params {
    text: String,
}

async fn main_page() -> impl IntoResponse {
    let template = MainTemplate{};
    HtmlTemplate(template)
}

async fn echo(Query(input_text): Query<Params>) -> impl IntoResponse {
    let template = EchoTemplate {
        text: input_text.text,
    };
    HtmlTemplate(template)
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/echo", get(echo))

}

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listenening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;