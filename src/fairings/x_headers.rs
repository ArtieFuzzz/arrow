use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

#[derive(Default, Clone)]
pub struct XHeaders(String);

#[rocket::async_trait]
impl Fairing for XHeaders {
    fn info(&self) -> Info {
        Info {
            name: "Header Rewrite",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("X-WhoIs", "ArtieFuzzz"));
        res.set_header(Header::new("X-WhoIs-Email", "artiefuzzz@duck.com"));
    }
}