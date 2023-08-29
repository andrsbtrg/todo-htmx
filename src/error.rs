use askama_axum::IntoResponse;
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

/// An error Enum for the server
#[derive(Debug)]
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
}

impl IntoResponse for Error {
    fn into_response(self) -> askama_axum::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
