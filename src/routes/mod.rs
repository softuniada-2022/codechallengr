use std::io;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;

#[get("/")]
pub async fn index() -> io::Result<NamedFile> {
    NamedFile::open("dist/index.html").await
}

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> io::Result<NamedFile> {
    let f = NamedFile::open(Path::new("dist/").join(file)).await;
    match f {
        Err(_) => NamedFile::open(Path::new("dist/index.html")).await,
        x => x,
    }
}

pub mod exercise;
pub mod login;
pub mod score;
pub mod solution;
pub mod user;
