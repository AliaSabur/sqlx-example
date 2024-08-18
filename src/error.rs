use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

pub type ServerResult<T> = Result<T, ServerError>;

// Make our own error that wraps `anyhow::Error`.
#[derive(serde::Serialize, Debug)]
pub struct ServerError {
    #[serde(skip)]
    code: u16,
    msg: Option<String>,
}

impl ServerError {
    pub fn new(msg: String, code: StatusCode) -> Self {
        Self {
            msg: Some(msg),
            code: code.as_u16(),
        }
    }
}

// Tell axum how to convert `ResponseError` into a response.
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        // Convert our error into a response with the appropriate status code.
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        // 4xx, 5xx, json
        (
            status_code,
            [(CONTENT_TYPE, "application/json")],
            Json(self),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to
// turn them into `Result<_>`. That way you don't need to do that manually.
impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        let err = err.into();
        let err_msg = err.to_string();

        let make_error = |code: StatusCode| ServerError {
            msg: Some(err_msg),
            code: code.as_u16(),
        };

        // default 500
        make_error(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

macro_rules! static_err {
    ($name:ident, $status:expr) => {
        #[allow(non_snake_case, missing_docs)]
        pub fn $name<E>(err: E) -> ServerError
        where
            E: Into<anyhow::Error> + ToString,
        {
            let code: StatusCode = $status;
            ServerError {
                msg: Some(err.to_string()),
                code: code.as_u16(),
            }
        }
    };
}

impl ServerError {
    // 4xx
    static_err!(BadRequest, StatusCode::BAD_REQUEST);

    static_err!(NotFound, StatusCode::NOT_FOUND);

    static_err!(Unauthorized, StatusCode::UNAUTHORIZED);

    static_err!(PaymentRequired, StatusCode::PAYMENT_REQUIRED);

    static_err!(Forbidden, StatusCode::FORBIDDEN);

    static_err!(MethodNotAllowed, StatusCode::METHOD_NOT_ALLOWED);

    static_err!(NotAcceptable, StatusCode::NOT_ACCEPTABLE);

    static_err!(
        ProxyAuthenticationRequired,
        StatusCode::PROXY_AUTHENTICATION_REQUIRED
    );

    static_err!(RequestTimeout, StatusCode::REQUEST_TIMEOUT);

    static_err!(Conflict, StatusCode::CONFLICT);

    static_err!(Gone, StatusCode::GONE);

    static_err!(LengthRequired, StatusCode::LENGTH_REQUIRED);

    static_err!(PreconditionFailed, StatusCode::PRECONDITION_FAILED);

    static_err!(PreconditionRequired, StatusCode::PRECONDITION_REQUIRED);

    static_err!(PayloadTooLarge, StatusCode::PAYLOAD_TOO_LARGE);

    static_err!(UriTooLong, StatusCode::URI_TOO_LONG);

    static_err!(UnsupportedMediaType, StatusCode::UNSUPPORTED_MEDIA_TYPE);

    static_err!(RangeNotSatisfiable, StatusCode::RANGE_NOT_SATISFIABLE);

    static_err!(ExpectationFailed, StatusCode::EXPECTATION_FAILED);

    static_err!(UnprocessableEntity, StatusCode::UNPROCESSABLE_ENTITY);

    static_err!(TooManyRequests, StatusCode::TOO_MANY_REQUESTS);

    static_err!(
        RequestHeaderFieldsTooLarge,
        StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
    );

    static_err!(
        UnavailableForLegalReasons,
        StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
    );

    // 5xx
    static_err!(InternalServerError, StatusCode::INTERNAL_SERVER_ERROR);

    static_err!(NotImplemented, StatusCode::NOT_IMPLEMENTED);

    static_err!(BadGateway, StatusCode::BAD_GATEWAY);

    static_err!(ServiceUnavailable, StatusCode::SERVICE_UNAVAILABLE);

    static_err!(GatewayTimeout, StatusCode::GATEWAY_TIMEOUT);

    static_err!(VersionNotSupported, StatusCode::HTTP_VERSION_NOT_SUPPORTED);

    static_err!(VariantAlsoNegotiates, StatusCode::VARIANT_ALSO_NEGOTIATES);

    static_err!(InsufficientStorage, StatusCode::INSUFFICIENT_STORAGE);

    static_err!(LoopDetected, StatusCode::LOOP_DETECTED);
}
