use std::fmt;

macro_rules! declare_status {
    ($({$enum_name: ident, $code: expr, $code_text: expr, $text: expr, $phrase: expr}),+) => {
        // HTTP status code as registered with the Internet Assigned Numbers Authority (https://www.iana.org).
        // See https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml.
        #[derive(PartialEq, Eq)]
        pub enum Status {
            $($enum_name,)+
        }

        impl Status {
            pub fn from_code(code: usize) -> Option<Self> {
                return match code {
                    $($code => Some(Status::$enum_name)),+,
                    _ => None,
                }
            }

            pub fn get_code(&self) -> usize {
                return match self {
                    $(Status::$enum_name => $code),+
                }
            }

            pub fn from_code_text(code: &[u8]) -> Option<Self> {
                return match code {
                    $($code_text => Some(Status::$enum_name)),+,
                    _ => None,
                }
            }

            pub fn get_code_text(&self) -> &'static [u8] {
                return match self {
                    $(Status::$enum_name => $code_text),+
                }
            }

            pub fn from_text(code: &[u8]) -> Option<Self> {
                return match code {
                    $($text => Some(Status::$enum_name)),+,
                    _ => None,
                }
            }

            pub fn get_text(&self) -> &'static [u8] {
                return match self {
                    $(Status::$enum_name => $text),+
                }
            }

            pub fn from_phrase(code: &[u8]) -> Option<Self> {
                return match code {
                    $($phrase => Some(Status::$enum_name)),+,
                    _ => None,
                }
            }

            pub fn get_phrase(&self) -> &'static [u8] {
                return match self {
                    $(Status::$enum_name => $phrase),+
                }
            }
        }
    };
}

declare_status!(
    // 1xx: Informational - Request received, continuing process
    // RFC 7231n 6.2.1
    {Continue, 100, b"100", b"Continue", b"100 Continue"},
    // RFC 7231 6.2.2
    {SwitchingProtocols, 101, b"101", b"Switching Protocols", b"101 Switching Protocols"},
    // RFC 2518
    {Processing, 102, b"102", b"Processing", b"102 Processing"},
    // RFC 8297
    {EarlyHints, 103, b"103", b"Early Hints", b"103 Early Hints"},

    // 2xx: Success - The action was successfully received, understood, and accepted
    // RFC 7231 6.3.1
    {OK, 200, b"200", b"OK", b"200 OK"},
    // RFC 7231 6.3.2
    {Created, 201, b"201", b"Created", b"201 Created"},
    // RFC 7231 6.3.3
    {Accepted, 202, b"202", b"Accepted", b"202 Accepted"},
    // RFC 7231 6.3.4
    {NonAuthoritativeInformation, 203, b"203", b"Non-Authoritative Information", b"203 Non-Authoritative Information"},
    // RFC 7231 6.3.5
    {NoContent, 204, b"204", b"No Content", b"204 No Content"},
    // RFC 7231 6.3.6
    {ResetContent, 205, b"205", b"Reset Content", b"205 Reset Content"},
    // RFC 7233 4.1
    {PartialContent, 206, b"206", b"Partial Content", b"206 Partial Content"},
    // RFC 4918
    {MultiStatus, 207, b"207", b"Multi-Status", b"207 Multi-Status"},
    // RFC 5842
    {AlreadyReported, 208, b"208", b"Already Reported", b"208 Already Reported"},
    // RFC 3229
    {IMUsed, 226, b"226", b"IM Used", b"226 IM Used"},


    // 3xx: Redirection - Further action must be taken in order to complete the request
    // RFC 7231 6.4.1
    {MultipleChoices, 300, b"300", b"Multiple Choices", b"300 Multiple Choices"},
    // RFC 7231 6.4.2
    {MovedPermanently, 301, b"301", b"Moved Permanently", b"301 Moved Permanently"},
    // RFC 7231 6.4.3
    {Found, 302, b"302", b"Found", b"302 Found"},
    // RFC 7231 6.4.4
    {SeeOther, 303, b"303", b"See Other", b"303 See Other"},
    // RFC 7232 4.1
    {NotModified, 304, b"304", b"Not Modified", b"304 Not Modified"},
    // RFC 7231 6.4.5
    {UseProxy, 305, b"305", b"Use Proxy", b"305 Use Proxy"},
    // RFC 7231 6.4.7
    {TemporaryRedirect, 307, b"307", b"Temporary Redirect", b"307 Temporary Redirect"},
    // RFC 7538
    {PermanentRedirect, 308, b"308", b"Permanent Redirect", b"308 Permanent Redirect"},

    // 4xx: Client Error - The request contains bad syntax or cannot be fulfilled
    // RFC 7231 6.5.1
    {BadRequest, 400, b"400", b"Bad Request", b"400 Bad Request"},
    // RFC 7235 3.1
    {Unauthorized, 401, b"401", b"Unauthorized", b"401 Unauthorized"},
    // RFC 7231 6.5.2
    {PaymentRequired, 402, b"402", b"Payment Required", b"402 Payment Required"},
    // RFC 7231 6.5.3
    {Forbidden, 403, b"403", b"Forbidden", b"403 Forbidden"},
    // RFC 7231 6.5.4
    {NotFound, 404, b"404", b"Not Found", b"404 Not Found"},
    // RFC 7231 6.5.5
    {MethodNotAllowed, 405, b"405", b"Method Not Allowed", b"405 Method Not Allowed"},
    // RFC 7231 6.5.6
    {NotAcceptable, 406, b"406", b"Not Acceptable", b"406 Not Acceptable"},
    // RFC 7235 3.2
    {ProxyAuthenticationRequired, 407, b"407", b"Proxy Authentication Required", b"407 Proxy Authentication Required"},
    // RFC 7231 6.5.7
    {RequestTimeout, 408, b"408", b"Request Timeout", b"408 Request Timeout"},
    // RFC 7231 6.5.8
    {Conflict, 409, b"409", b"Conflict", b"409 Conflict"},
    // RFC 7231 6.5.9
    {Gone, 410, b"410", b"Gone", b"410 Gone"},
    // RFC7231 6.5.10
    {LengthRequired, 411, b"411", b"Length Required", b"411 Length Required"},
    // RFC 7232 4.2
    // RFC 8144 3.2
    {PreconditionFailed, 412, b"412", b"Precondition Failed", b"412 Precondition Failed"},
    // RFC 7231 6.5.11
    {PayloadTooLarge, 413, b"413", b"Payload Too Large", b"413 Payload Too Large"},
    // RFC 7231 6.5.12
    {URITooLong, 414, b"414", b"URI Too Long", b"414 URI Too Long"},
    // RFC 7231 6.5.13
    // RFC 7694 3
    {UnsupportedMediaType, 415, b"415", b"Unsupported Media Type", b"415 Unsupported Media Type"},
    // RFC 7233 4.4
    {RangeNotSatisfiable, 416, b"416", b"Range Not Satisfiable", b"416 Range Not Satisfiable"},
    // RFC 7231 6.5.14
    {ExpectationFailed, 417, b"417", b"Expectation Failed", b"417 Expectation Failed"},
    // RFC 7540 9.1.2
    {MisdirectedRequest, 421, b"421", b"Misdirected Request", b"421 Misdirected Request"},
    // RFC 4918
    {UnprocessableEntity, 422, b"422", b"Unprocessable Entity", b"422 Unprocessable Entity"},
    // RFC 4918
    {Locked, 423, b"423", b"Locked", b"423 Locked"},
    // RFC 4918
    {FailedDependency, 424, b"424", b"Failed Dependency", b"424 Failed Dependency"},
    // RFC 8470
    {TooEarly, 425, b"425", b"Too Early", b"425 Too Early"},
    // RFC 7231 6.5.15
    {UpgradeRequired, 426, b"426", b"Upgrade Required", b"426 Upgrade Required"},
    // RFC 6585
    {PreconditionRequired, 428, b"428", b"Precondition Required", b"428 Precondition Required"},
    // RFC 6585
    {TooManyRequests, 429, b"429", b"Too Many Requests", b"429 Too Many Requests"},
    // RFC 6585
    {RequestHeaderFieldsTooLarge, 431, b"431", b"Request Header Fields Too Large", b"431 Request Header Fields Too Large"},
    // RFC 7725
    {UnavailableForLegalReasons, 451, b"451", b"Unavailable For Legal Reasons", b"451 Unavailable For Legal Reasons"},

    // 5xx: Server Error - The server failed to fulfill an apparently valid request
    // RFC 7231 6.6.1
    {InternalServerError, 500, b"500", b"Internal Server Error", b"500 Internal Server Error"},
    // RFC 7231 6.6.2
    {NotImplemented, 501, b"501", b"Not Implemented", b"501 Not Implemented"},
    // RFC 7231 6.6.3
    {BadGateway, 502, b"502", b"Bad Gateway", b"502 Bad Gateway"},
    // RFC 7231 6.6.4
    {ServiceUnavailable, 503, b"503", b"Service Unavailable", b"503 Service Unavailable"},
    // RFC 7231 6.6.5
    {GatewayTimeout, 504, b"504", b"Gateway Timeout", b"504 Gateway Timeout"},
    // RFC 7231 6.6.6
    {HTTPVersionNotSupported, 505, b"505", b"HTTP Version Not Supported", b"505 HTTP Version Not Supported"},
    // RFC 2295
    {VariantAlsoNegotiates, 506, b"506", b"Variant Also Negotiates", b"506 Variant Also Negotiates"},
    // RFC 4918
    {InsufficientStorage, 507, b"507", b"Insufficient Storage", b"507 Insufficient Storage"},
    // RFC 5842
    {LoopDetected, 508, b"508", b"Loop Detected", b"508 Loop Detected"},
    // RFC 2774
    {NotExtended, 510, b"510", b"Not Extended", b"510 Not Extended"},
    // RFC 6585
    {NetworkAuthenticationRequired, 511, b"511", b"Network Authentication Required", b"511 Network Authentication Required"}
);

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return unsafe {
            formatter.write_str(std::str::from_utf8_unchecked(self.get_phrase()))
        };
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return unsafe {
            formatter.write_str(std::str::from_utf8_unchecked(self.get_phrase()))
        };
    }
}
