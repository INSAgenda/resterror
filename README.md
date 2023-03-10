# Resterror
This is a simple library to handle errors in a RESTful way. It uses a lightweight syntax to define errors and their codes.
  
## Support 
This crate uses the 'PO' translation method and also allows you to define error messages in a JSON file. This means that if you use the PO method, the error messages are defined in a PO file for each language.
  
Because this crate is used in the INSAgenda project, the JSON format of the response is defined with the following fields:

```json
{
  "kind": "invalid_request",
  "messages_fr": "Requête invalide",
  "messages_en": "Invalid request",
  "origin": ""
}
```
In the future, this crate will be more generic and will be in the following format:
```json
{
  "kind": "invalid_request",
  "messages": {
    "fr": "Requête invalide",
    "en": "Invalid request"
  }
}
```
## Usage
You can for example define an error for actix: 
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
        /// Custom error
        #[error(code = 500)]
        CustomError(Translation),
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
You need to define the PO directory where the PO files are located.  
The name of each file must be the language code.

### Use errors
For example, you can use the error in an actix route like this:
```rust
    #[get("/")]
    async fn index() -> Result<HttpResponse, Error> {
        Err(Error::InvalidRequest)
    }

    #[get("/custom-error")]
    async fn custom_error() -> Result<HttpResponse, Error> {
        Err(Error::CustomError(tr! {
            "fr" => "Erreur personnalisée",
            "en" => "Custom error"
        }))
    }
```
This error will be translated to the following JSON:
```json
{
  "kind": "invalid_request",
  "messages_fr": "Requête invalide",
  "messages_en": "Invalid request",
  "origin": ""
}
```
With the following PO file:

fr.po :
```po

msgid "invalid_request"
msgstr "Requête invalide"
```

en.po :
```po

msgid "invalid_request"
msgstr "Invalid request"
```

That's it, you can now use this library to handle errors in a RESTful way in your project. Keep in mind that this is just a basic example, you can customize and expand it to fit your needs.
