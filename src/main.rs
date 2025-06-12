use actix_files as fs;
use actix_web::{App, HttpResponse, HttpRequest, HttpServer, Responder, middleware, web};
use clap::{App as ClapApp, Arg};
use std::{path::PathBuf, io, fs as std_fs};
use log::{info, warn, error};
use env_logger::Env;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct FolderInfo {
    name: String,
    path: String,
}

async fn hello(req: HttpRequest) -> impl Responder {
    info!("Hello from new user: {}", req.uri());
    HttpResponse::Found()
        .append_header(("Location", "/index.html"))
        .finish()
}

// 处理获取文件夹列表的API
async fn get_folders(root_dir: web::Data<PathBuf>) -> impl Responder {
    let mut folders = Vec::new();
    
    if let Ok(entries) = std_fs::read_dir(root_dir.get_ref()) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let folder_name = entry.file_name().to_string_lossy().into_owned();
                    folders.push(FolderInfo {
                        name: folder_name.clone(),
                        path: format!("/{}/onsyuri.html", folder_name),
                    });
                }
            }
        }
    }

    info!("{} subfolders were found!", folders.len());
    HttpResponse::Ok().json(folders)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // args parser
    let matches = ClapApp::new("actix-web-server")
        .version("1.0")
        .author("Forever")
        .about("ONScripter Web Server")
        .arg(
            Arg::with_name("host")
                .short('H')
                .long("host")
                .value_name("HOST")
                .help("Sets the server host")
                .default_value("127.0.0.1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the server port")
                .default_value("9991")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("root")
                .short('r')
                .long("root")
                .value_name("ROOT_DIR")
                .help("Sets the root directory to serve files from")
                .default_value("./assets")
                .takes_value(true),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let root_dir = PathBuf::from(matches.value_of("root").unwrap());

    if !root_dir.exists() || !root_dir.is_dir() {
        eprintln!("Error: Directory not found!");
        std::process::exit(1);
    }

    

    let address = format!("{}:{}", host, port);
    info!("Server running at http://{}", address);
    info!("Root directory: {:?}", root_dir);

    // 创建一个指向根目录的共享引用
    let shared_root_dir = web::Data::new(root_dir.clone());


    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello))
            .route("/api/folders", web::get().to(get_folders))
            .service(
                fs::Files::new("/", &root_dir)
                    .show_files_listing()
                    .use_last_modified(true)
            )
            // API端点 获取文件夹列表
            .app_data(shared_root_dir.clone())
            // 404
            .default_service(
                actix_web::web::route().to(|| async {
                    HttpResponse::NotFound().body("404 Not Found")
                })
            )
    })
    .bind(address)?
    .run()
    .await
}
    