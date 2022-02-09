use std::path::PathBuf;

use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::Path;
use rocket::response::{self, Responder, Response};
use rocket::request::Request;

#[macro_use] 
extern crate rocket;

struct CachedFile(NamedFile);

impl<'r> Responder<'r, 'r> for CachedFile {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Cache-control", "max-age=86400") //  24h (24*60*60)
            .raw_header("Content-type", "application/x-ns-proxy-autoconfig")
            .ok()
    }
}


#[get("/<file..>")]
async fn files(file: PathBuf) -> Result<CachedFile, NotFound<String>> {
    let path = Path::new(".").join(file);
    NamedFile::open(&path).await.map(|nf| CachedFile(nf)).map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![files])
}