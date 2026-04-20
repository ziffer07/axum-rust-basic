use askama::{Template};
use axum::{
    Form, Router, extract::Query, http::StatusCode, response::{Html, IntoResponse, Redirect, Response}, routing::{get, post}
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

#[derive(Template)]
#[template(path = "../templates/post_form.html")]
struct PostTemplate {}

#[derive(Template)]
#[template(path = "../templates/echo_post.html")]
struct EchoPostTemplate {
    text_in: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
struct Params {
    text: String,
}

// Get instruction
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


// Redirects
async fn go_to_post() -> Redirect {
    Redirect::temporary("/post-form")
}

async fn home_redirect() -> Redirect {
    Redirect::temporary("/")
}

//Post Instructions
async fn post_form() -> impl IntoResponse {
    let template = PostTemplate{};
    HtmlTemplate(template)
}

async fn echo_post(Form(input): Form<Params>) -> impl IntoResponse {
    let template =  EchoPostTemplate {
        text_in: input.text,
    };
    HtmlTemplate(template)
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/echo", get(echo))
        .route("/home_redirect", get(home_redirect))
        .route("/post-form", get(post_form))
        .route("/echo-post", post(echo_post))
        .route("/post-form-redirect", get(go_to_post))
        

}

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("Listenening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;