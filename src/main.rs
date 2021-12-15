use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    if let Some(ip) = req.connection_info().realip_remote_addr() {
        let v: Vec<&str> = ip.split(":").collect();
        if let Some(remote) = v.into_iter().nth(0) {
            return format!("{}", remote);
        }
    }
    String::from("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
