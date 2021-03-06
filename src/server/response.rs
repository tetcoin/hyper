//! Server Responses
//!
//! These are responses sent by a `hyper::Server` to clients, after
//! receiving a request.
use header;
use http;
use status;
use version;


/// The outgoing half for a Tcp connection, created by a `Server` and given to a `Handler`.
///
/// The default `StatusCode` for a `Response` is `200 OK`.
#[derive(Debug)]
pub struct Response<'a> {
    head: &'a mut http::ResponseHead
}

impl<'a> Response<'a> {
    /// The headers of this response.
    #[inline]
    pub fn headers(&self) -> &header::Headers { &self.head.headers }

    /// The status of this response.
    #[inline]
    pub fn status(&self) -> status::StatusCode { 
        status::StatusCode::from_u16(self.head.subject.0)
    }

    /// The HTTP version of this response.
    #[inline]
    pub fn version(&self) -> &version::HttpVersion { &self.head.version }

    /// Get a mutable reference to the Headers.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut header::Headers { &mut self.head.headers }

    /// Get a mutable reference to the status.
    #[inline]
    pub fn set_status(&mut self, status: status::StatusCode) {
        self.head.subject = status.into();
    }
}

/// Creates a new Response that can be used to write to a network stream.
pub fn new<'a>(head: &'a mut http::ResponseHead) -> Response<'a> {
    Response {
        head: head
    }
}
