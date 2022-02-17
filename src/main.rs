
mod http_server;
use std::sync::Arc;

use http_server::HttpServer;
fn main(){
    let mut server = HttpServer::new("localhost", 8080);
    let file_system = server.get_file_system();

    let index_html = file_system.add_file("index.html").expect("Dosya okunamadı");
    let html_404 = file_system.add_file("404.html").expect("Dosya okunamadı");
    for file in server.get_file_system().get_files(){
        println!("{:?}", file.get_path());
    }

    server.get("/", move |response, request|{
        response.send_file(&index_html);
        response.send(format!("{:?}", request.get_headers()));
    });


    server.not_found_page( move |response, request|{
        response.set_status(http_server::ResponseStatus::NotFound);    
        response.send(format!("Bulunamadı: {}", request.get_path()));
        response.send_file(&html_404);
    });
    
    server.get("/json", |response, request| {
        response.add_header("Content-Type: application/json; charset=UTF-8");
        response.send("{\"text\": \"Test\"}");
    });
    server.get("/test", |response, request| {
        response.set_status(http_server::ResponseStatus::Custom(String::from("1902 Yeni Bir Hata")));
        response.send(r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Test</h1>
    <p>Hi from Rust</p>
  </body>
</html>"#);
    });

    server.get_with_variables("/user/{user}?{pass}?", |response, request|{
        response.add_header("Content-Type: application/json; charset=UTF-8");
        response.send(format!("{{\"{}\":\"{}\"}}", response.get_variable("{user}"), response.get_variable("{pass}")))
    });
    
    server.start_listen();
}