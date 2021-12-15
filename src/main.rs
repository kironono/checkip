use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use clap::{crate_authors, crate_description, crate_name, crate_version, value_t_or_exit, Arg};

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
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("binding")
                .long("binding")
                .short("b")
                .default_value("127.0.0.1")
                .help("Binds server to the specified IP"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .default_value("8080")
                .help("Runs server on the specified port"),
        )
        .get_matches();

    let binding = value_t_or_exit!(matches.value_of("binding"), String);
    let port = value_t_or_exit!(matches.value_of("port"), u16);

    println!("Server started {}:{}", binding, port);
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind((binding, port))?
        .run()
        .await
}
