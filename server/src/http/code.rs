use std::fmt::Display;

#[allow(dead_code)]
pub enum HttpCode {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl HttpCode {
    pub fn to_u16(&self) -> u16 {
        match self {
            HttpCode::Ok => 200,
            HttpCode::Created => 201,
            HttpCode::Accepted => 202,
            HttpCode::NoContent => 204,
            HttpCode::MovedPermanently => 301,
            HttpCode::Found => 302,
            HttpCode::NotModified => 304,
            HttpCode::BadRequest => 400,
            HttpCode::Unauthorized => 401,
            HttpCode::Forbidden => 403,
            HttpCode::NotFound => 404,
            HttpCode::MethodNotAllowed => 405,
            HttpCode::InternalServerError => 500,
            HttpCode::NotImplemented => 501,
            HttpCode::BadGateway => 502,
            HttpCode::ServiceUnavailable => 503,
        }
    }
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            HttpCode::Ok => "OK",
            HttpCode::Created => "Created",
            HttpCode::Accepted => "Accepted",
            HttpCode::NoContent => "No Content",
            HttpCode::MovedPermanently => "Moved Permanently",
            HttpCode::Found => "Found",
            HttpCode::NotModified => "Not Modified",
            HttpCode::BadRequest => "Bad Request",
            HttpCode::Unauthorized => "Unauthorized",
            HttpCode::Forbidden => "Forbidden",
            HttpCode::NotFound => "Not Found",
            HttpCode::MethodNotAllowed => "Method Not Allowed",
            HttpCode::InternalServerError => "Internal Server Error",
            HttpCode::NotImplemented => "Not Implemented",
            HttpCode::BadGateway => "Bad Gateway",
            HttpCode::ServiceUnavailable => "Service Unavailable",
        };
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u16() {
        assert_eq!(HttpCode::Ok.to_u16(), 200);
        assert_eq!(HttpCode::Created.to_u16(), 201);
        assert_eq!(HttpCode::Accepted.to_u16(), 202);
        assert_eq!(HttpCode::NoContent.to_u16(), 204);
        assert_eq!(HttpCode::MovedPermanently.to_u16(), 301);
        assert_eq!(HttpCode::Found.to_u16(), 302);
        assert_eq!(HttpCode::NotModified.to_u16(), 304);
        assert_eq!(HttpCode::BadRequest.to_u16(), 400);
        assert_eq!(HttpCode::Unauthorized.to_u16(), 401);
        assert_eq!(HttpCode::Forbidden.to_u16(), 403);
        assert_eq!(HttpCode::NotFound.to_u16(), 404);
        assert_eq!(HttpCode::MethodNotAllowed.to_u16(), 405);
        assert_eq!(HttpCode::InternalServerError.to_u16(), 500);
        assert_eq!(HttpCode::NotImplemented.to_u16(), 501);
        assert_eq!(HttpCode::BadGateway.to_u16(), 502);
        assert_eq!(HttpCode::ServiceUnavailable.to_u16(), 503);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", HttpCode::Ok), "OK");
        assert_eq!(format!("{}", HttpCode::Created), "Created");
        assert_eq!(format!("{}", HttpCode::Accepted), "Accepted");
        assert_eq!(format!("{}", HttpCode::NoContent), "No Content");
        assert_eq!(format!("{}", HttpCode::MovedPermanently), "Moved Permanently");
        assert_eq!(format!("{}", HttpCode::Found), "Found");
        assert_eq!(format!("{}", HttpCode::NotModified), "Not Modified");
        assert_eq!(format!("{}", HttpCode::BadRequest), "Bad Request");
        assert_eq!(format!("{}", HttpCode::Unauthorized), "Unauthorized");
        assert_eq!(format!("{}", HttpCode::Forbidden), "Forbidden");
        assert_eq!(format!("{}", HttpCode::NotFound), "Not Found");
        assert_eq!(format!("{}", HttpCode::MethodNotAllowed), "Method Not Allowed");
        assert_eq!(format!("{}", HttpCode::InternalServerError), "Internal Server Error");
        assert_eq!(format!("{}", HttpCode::NotImplemented), "Not Implemented");
        assert_eq!(format!("{}", HttpCode::BadGateway), "Bad Gateway");
        assert_eq!(format!("{}", HttpCode::ServiceUnavailable), "Service Unavailable");
    }
}