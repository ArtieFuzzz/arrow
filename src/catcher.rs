use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("{} is invalid", req.uri())
}

#[catch(500)]
pub fn internal_error<'a>(_req: &Request) -> &'a str {
    "internal_error"
}

#[catch(401)]
pub fn unauthorized<'a>(_req: &Request) -> &'a str {
    "unauthorized"
}

#[catch(403)]
pub fn forbidden<'a>(_req: &Request) -> &'a str {
    "forbidden"
}

#[catch(400)]
pub fn broken_request(_req: &Request) -> String {
    format!(
        "{}",
        "there is something wrong or missing from your request..."
    )
}