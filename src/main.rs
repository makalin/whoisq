use rocket::form::Form;
use rocket::fs::relative;
use rocket::response::Redirect;
use rocket::{get, launch, post, routes};
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// Form data structure for the domain input
#[derive(Deserialize, FromForm)]
struct DomainForm {
    domain: String,
}

// Function to get WHOIS server based on TLD
fn get_whois_server(tld: &str) -> Option<&str> {
    let mut whois_servers = HashMap::new();
    whois_servers.insert("com", "whois.verisign.com");
    whois_servers.insert("net", "whois.verisign.com");
    whois_servers.insert("org", "whois.pir.org");
    whois_servers.insert("edu", "whois.educause.edu");
    whois_servers.insert("uk", "whois.nic.uk");

    whois_servers.get(tld).copied()
}

// Async function to perform WHOIS query
async fn query_whois(domain: &str) -> Result<String, String> {
    // Extract TLD from domain (e.g., "example.com" -> "com")
    let tld = domain.split('.').last().ok_or("Invalid domain format")?;
    let whois_server = get_whois_server(tld).ok_or(format!("No WHOIS server for TLD: {}", tld))?;

    // Connect to WHOIS server on port 43
    let addr = format!("{}:43", whois_server);
    let mut stream = TcpStream::connect(&addr)
        .await
        .map_err(|e| format!("Failed to connect to WHOIS server: {}", e))?;

    // Send domain query
    let query = format!("{}\r\n", domain);
    stream
        .write_all(query.as_bytes())
        .await
        .map_err(|e| format!("Failed to send query: {}", e))?;

    // Read response
    let mut buffer = Vec::new();
    stream
        .read_to_end(&mut buffer)
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    String::from_utf8(buffer)
        .map_err(|e| format!("Invalid UTF-8 response: {}", e))
}

// Index route (GET /)
#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

// Search route (POST /search)
#[post("/search", data = "<form>")]
async fn search(form: Form<DomainForm>) -> Template {
    let domain = &form.domain;
    let response = match query_whois(domain).await {
        Ok(result) => result,
        Err(error) => format!("Error: {}", error),
    };

    Template::render("result", context! {
        response: response
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, search])
        .attach(Template::fairing())
}