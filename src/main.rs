mod math;

use axum::{
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use askama::Template;
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Instant;
use math::{EuclideanRow};


// Templates mit Askama
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    items: Vec<String>,
}

#[derive(Template)]
#[template(path = "euclidean.html")]
struct EuclideanTemplate {
    a: usize,
    b: usize,
    gcd: usize,
    euclidean_rows: Vec<EuclideanRow>,
}

#[derive(Template)]
#[template(path = "euclidean_wc.html")]
struct EuclideanWortsCaseTemplate {
    upper_range: usize,
    best_pair: (usize,usize),
    max_steps: usize,
    duration_ms: u128,
}

#[derive(Template)]
#[template(path = "factorization.html")]
struct FactorizationTemplate {
    number: usize,
    factors: Vec<usize>,
}

#[derive(Template)]
#[template(path = "residue_table.html")]
struct ResidueTableTemplate {
    moduli: usize,
    is_prime: bool,
    primes: Vec<usize>,
    addition: Vec<Vec<usize>>,
    multiplication: Vec<Vec<usize>>,
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

async fn euclidian_algorithm(Path((a, b)): Path<(usize, usize)>) -> Html<String> {
    let mut rows: Vec<EuclideanRow> =  Vec::new();
    let gcd = math::gcd(a, b, &mut rows);
    let template = EuclideanTemplate {
        a: a,
        b: b,
        gcd: gcd,
        euclidean_rows: rows,
    };
    Html(template.render().unwrap())
}

async fn euclidian_algorithm_worst_case(Path((upper)): Path<usize>) -> Html<String> {
    let mut max_steps = 0;
    let mut best_pair = (0, 0);

    let start = Instant::now();

    for i in 1..upper {
        for j in 1..i {
            let mut rows: Vec<EuclideanRow> =  Vec::new();
            let _gcd = math::gcd(i, j, &mut rows);
            if rows.len() > max_steps {
                max_steps = rows.len();
                best_pair = (i, j);
            }
        }
    }

    let duration = start.elapsed().as_millis();

    let template = EuclideanWortsCaseTemplate {
        upper_range: upper,
        best_pair: best_pair,
        max_steps: max_steps,
        duration_ms: duration,
    };
    Html(template.render().unwrap())
}

async fn integer_factorization(Path(number): Path<usize>) -> Html<String> {
    let template = FactorizationTemplate {
        number,
        factors: math::prime_factors(number),
    };
    Html(template.render().unwrap())
}

async fn residue_class(Path(moduli): Path<usize>) -> Html<String> {
    let prime = math::is_prime(moduli);
    let primes = math::get_primes(moduli);

    let addition = math::remainder_table(moduli, |row, col| row + col);
    let multiplication = math::remainder_table(moduli, |row, col| row * col);

    let template = ResidueTableTemplate {
        moduli: moduli,
        is_prime: prime,
        primes: primes,
        addition,
        multiplication,
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
        .route("/euclidian_algorithm/:number/:number", get(euclidian_algorithm))
        .route("/euclidian_algorithm_worst_case/:number", get(euclidian_algorithm_worst_case))
        .route("/integer_factorization/:number", get(integer_factorization))
        .route("/residue_class/:size", get(residue_class))

        .route("/detail/:id", get(detail))
        .route("/search", get(search))
        .route("/contact", get(contact_form).post(submit_contact));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server läuft auf http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
