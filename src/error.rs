use askama_axum::IntoResponse;
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

/// An error Enum for the server
#[derive(Clone, Debug, strum_macros::AsRefStr)]
pub enum Error {
    /// Login failed
    LoginFail,
    /// Ticket creation failed
    TicketCreationFailed,
    /// No tickets
    NoTicketsFound,
    /// The id provided was not found
    TicketNotFound,
    /// No Auth Token cookie found
    AuthFailNoAuthToken,
    EmptyTicket,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
    UserCreateFail,
    /// The provided user id doesnt exist
    UserIdNotFound,
    /// Change ticket state failed
    UpdateTicketError,
    DatabaseError,
    TicketUpdateFailed,
}

impl IntoResponse for Error {
    fn into_response(self) -> askama_axum::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum response

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // Auth errors
            Self::AuthFailNoAuthToken
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model errors
            Self::TicketNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            // fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
