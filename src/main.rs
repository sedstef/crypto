use askama::Template;
use axum::{
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
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
#[template(path = "factorization.html")]
struct FactorizationTemplate {
    number: u64,
    factors: Vec<u64>,
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

#[derive(Template)]
#[template(path = "residue_table.html")]
struct ResidueTableTemplate {
    moduli: usize,
    is_prime: bool,
    primes: Vec<usize>,
    addition: Vec<Vec<usize>>,
    multiplication: Vec<Vec<usize>>,
}

// Template für Tabelle
#[derive(Template)]
#[template(path = "table.html")]
struct TableTemplate {
    rows: usize,
    cols: usize,
    data: Vec<Vec<String>>,
}

// Query Parameter für Tabellengröße
#[derive(Deserialize)]
struct TableQuery {
    rows: Option<usize>,
    cols: Option<usize>,
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

async fn integer_factorization(Path(number): Path<u64>) -> Html<String> {
    let template = FactorizationTemplate {
        number,
        factors: prime_factors(number),
    };
    Html(template.render().unwrap())
}


fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while divisor * divisor <= n {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
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

// Handler für Tabelle
async fn create_table(Query(params): Query<TableQuery>) -> Html<String> {
    let rows = params.rows.unwrap_or(5).min(50);
    let cols = params.cols.unwrap_or(5).min(20);

    // Generiere Beispieldaten
    let mut data = Vec::new();
    for row in 0..rows {
        let mut row_data = Vec::new();
        for col in 0..cols {
            row_data.push(format!("R{}C{}", row + 1, col + 1));
        }
        data.push(row_data);
    }

    let template = TableTemplate { rows, cols, data };
    Html(template.render().unwrap())
}

// Handler for residue class
async fn residue_class(Path(m): Path<usize>) -> Html<String> {
    let moduli = m;
    let prime = is_prime(moduli);
    let mut primes = Vec::new();
    for number in 0..=moduli {
        if is_prime(number) {
            primes.push(number as usize);
        }
    }

    let addition = fill_table(moduli, |row, col| row + col);
    let multiplication = fill_table(moduli, |row, col| row * col);

    let template = ResidueTableTemplate {
        moduli: moduli,
        is_prime: prime,
        primes: primes,
        addition,
        multiplication,
    };
    Html(template.render().unwrap())
}

fn fill_table(moduli: usize, function: fn(usize, usize) -> usize) -> Vec<Vec<usize>> {
    let mut data = Vec::new();

    for row in 0..=moduli {
        let mut row_data = Vec::new();
        for col in 0..=moduli {
            let value = function(row, col).rem_euclid(moduli);
            row_data.push(value);
        }
        data.push(row_data);
    }
    data
}

async fn multiplication_table(Path(size): Path<usize>) -> Html<String> {
    let size = size.min(20);
    let mut data = Vec::new();

    for row in 1..=size {
        let mut row_data = Vec::new();
        for col in 1..=size {
            row_data.push(format!("{}", row * col));
        }
        data.push(row_data);
    }

    let template = TableTemplate {
        rows: size,
        cols: size,
        data,
    };
    Html(template.render().unwrap())
}

// Define a function named 'is_prime' that takes a number as parameter and returns true if it's prime, false otherwise
fn is_prime(num: usize) -> bool {
    if num <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }

    // Check if num is divisible by any number from 2 to the square root of num
    for i in 2..=(num as f64).sqrt() as usize {
        if num % i == 0 {
            return false; // If num is divisible by any number other than 1 and itself, it's not prime
        }
    }

    true // If num is not divisible by any number other than 1 and itself, it's prime
}

#[tokio::main]
async fn main() {
    // Router mit verschiedenen Routes
    let app = Router::new()
        .route("/", get(index))
        .route("/integer_factorization/:number", get(integer_factorization))
        .route("/table", get(create_table))
        .route("/residue_class/:size", get(residue_class))
        .route("/multiplication/:size", get(multiplication_table))
        .route("/detail/:id", get(detail))
        .route("/search", get(search))
        .route("/contact", get(contact_form).post(submit_contact));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server läuft auf http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
