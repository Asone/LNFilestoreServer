use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::Responder;

type InnerResponder = NamedFile;

#[derive(Responder)]
#[response(status = 200)]
pub struct DownloadResponder {
    pub inner: InnerResponder,
    pub disposition: Header<'static>,
}
