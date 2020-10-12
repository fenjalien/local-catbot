use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Result;
use std::collections::HashMap;

async fn cat() -> HttpResponse {
    let req = reqwest::get("http://aws.random.cat/meow")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap();

    HttpResponse::Ok().body(format!(
        "<html>

<head>
    <meta name=\"viewport\" content=\"width=device-width; height=device-height;\">
    <link rel=\"stylesheet\" href=\"resource://content-accessible/ImageDocument.css\">
    <link rel=\"stylesheet\" href=\"resource://content-accessible/TopLevelImageDocument.css\">
    <link rel=\"stylesheet\" href=\"chrome://global/skin/media/TopLevelImageDocument.css\">
    <title>cat</title>
</head>

<body><img src=\"{}\"
        alt=\"cat\"></body>

</html>",
        req.get("file").expect("Failed to get cat :(")
    ))
}

#[actix_web::main]
async fn main() -> Result<()> {
    let server =
        HttpServer::new(|| App::new().route("/", web::get().to(cat))).bind("127.0.0.1:8080")?;
    if webbrowser::open("http://127.0.0.1:8080").is_err() {
        println!("Failed to open browser, site should be accessible at 'localhost:8080'");
    }
    server.run().await?;
    Ok(())
}
