use tonic::{Request, Status, metadata::MetadataValue};

pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "123456".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) => {
            if t == token {
                Ok(req)
            } else {
                Err(Status::permission_denied("Wrong authentication token"))
            }
        }
        _ => Err(Status::unauthenticated(
            "No valid authentication token found",
        )),
    }
}
