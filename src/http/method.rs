use std::fmt;

macro_rules! declare_methods {
    ($({$enum_name:ident, $text:expr}),+) => {
        #[derive(PartialEq, Eq)]
        pub enum Method {
            $($enum_name,)+
        }

        impl Method {
            pub fn from_text(code: &[u8]) -> Option<Self> {
                return match code {
                    $($text => Some(Method::$enum_name)),+,
                    _ => None,
                };
            }

            pub fn get_text(&self) -> &'static [u8] {
                return match self {
                    $(Method::$enum_name => $text),+
                };
            }
        }
    };
}

declare_methods!(
    {GET, b"GET"},
    {HEAD, b"HEAD"},
    {POST, b"POST"},
    {PUT, b"PUT"},
    {DELETE, b"DELETE"},
    {CONNECT, b"CONNECT"},
    {OPTIONS, b"OPTIONS"},
    {TRACE, b"TRACE"},
    {PATCH, b"PATCH"},
    {COPY, b"COPY"},
    {MOVE, b"MOVE"}
);

impl fmt::Display for Method {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return unsafe {
            formatter.write_str(std::str::from_utf8_unchecked(self.get_text()))
        };
    }
}

impl fmt::Debug for Method {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return unsafe {
            formatter.write_str(std::str::from_utf8_unchecked(self.get_text()))
        };
    }
}
