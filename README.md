# Resterror
This is a simple library to handle errors in a RESTful way.  
It uses a lightweight syntax to define errors and their codes.  

## Support 
This crate uses the 'PO' translation method.  
This means that the error messages are defined in a PO file for each language.

## Usage
You can for example define an error for actix
### Define errors
```rust
    #[derive(resterror::AsApiError, Debug, Clone)]
    #[po_directory = "locales/"]
    pub enum Error {
        /// Invalid request
        #[error(status = "BadRequest")]
        InvalidRequest,
        /// Database timeout 
        #[error(status = "InternalServerError")]
        DatabaseTimeout,
        /// Too large request
        #[error(status = "BadRequest")]
        RequestTooLarge,
        /// SQLite error
        #[error(status = "InternalServerError", msg_id = "database")]
        SqliteError,
        /// Invalid JSON
        #[error(status = "BadRequest")]
        InvalidJson,
        /// Blocking error
        #[error(status = "InternalServerError")]
        BlockingError,
        /// Auth required
        #[error(code = 511)]
        AuthentificationRequired,
    }

    impl From<BlockingError> for Error {
        fn from(_: BlockingError) -> Self { BlockingError }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
    }

    impl actix_web::ResponseError for Error {
        fn status_code(&self) -> StatusCode {
            let api_error = self.as_api_error();
            StatusCode::from_u16(api_error.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        }

        fn error_response(&self) -> actix_web::HttpResponse {
            let api_error = self.as_api_error();
            actix_web::HttpResponse::build(self.status_code()).json(api_error)
        }
    }

```
