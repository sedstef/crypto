use axum::{
    extract::{Path, Query},
    response::Html,
    routing::{get, post},
    Form, Router,
};
use askama::Template;
use serde::Deserialize;
use std::net::SocketAddr;

// Templates mit Askama
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    items: Vec<String>,
}

#[derive(Template)]
#[template(path = "detail.html")]
struct DetailTemplate {
    id: u32,
    name: String,
}

#[derive(Template)]
#[template(path = "form.html")]
struct FormTemplate {
    message: Option<String>,
}

// Form Daten
#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

// Handler
async fn index() -> Html<String> {
    let template = IndexTemplate {
        title: "Meine Rust Webapp".to_string(),
        items: vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
            "Item 3".to_string(),
        ],
    };
    Html(template.render().unwrap())
}

async fn detail(Path(id): Path<u32>) -> Html<String> {
    let template = DetailTemplate {
        id,
        name: format!("Artikel {}", id),
    };
    Html(template.render().unwrap())
}

async fn search(Query(params): Query<SearchQuery>) -> Html<String> {
    let query = params.q.unwrap_or_default();
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head><title>Suche</title></head>
        <body>
            <h1>Suchergebnisse</h1>
            <p>Du hast gesucht nach: <strong>{}</strong></p>
            <a href="/">Zurück</a>
        </body>
        </html>
        "#,
        query
    );
    Html(html)
}

async fn contact_form() -> Html<String> {
    let template = FormTemplate { message: None };
    Html(template.render().unwrap())
}

async fn submit_contact(Form(form): Form<ContactForm>) -> Html<String> {
    let template = FormTemplate {
        message: Some(format!(
            "Danke {}, deine Nachricht wurde empfangen!",
            form.name
        )),
    };
    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() {
    // Router mit verschiedenen Routes
    let app = Router::new()
        .route("/", get(index))
        .route("/detail/:id", get(detail))
        .route("/search", get(search))
        .route("/contact", get(contact_form).post(submit_contact));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server läuft auf http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
