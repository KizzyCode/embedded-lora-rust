//! The crate's error type

/// Creates an error
#[macro_export]
macro_rules! err {
    ($kind:expr, $desc:expr) => {{
        #[cfg(feature = "backtrace")]
        {
            // Create an error with rich backtrace information
            $crate::error::Error { kind: $kind, file: file!(), line: line!(), description: $desc }
        }

        #[cfg(not(feature = "backtrace"))]
        {
            // Create a size-optimized error
            $crate::error::Error { kind: $kind }
        }
    }};
    (eio: $desc:expr) => {{
        err!($crate::error::ErrorKind::IoError, $desc)
    }};
    (etimedout: $desc:expr) => {{
        err!($crate::error::ErrorKind::Timeout, $desc)
    }};
    (ebadmsg: $desc:expr) => {{
        err!($crate::error::ErrorKind::InvalidMessage, $desc)
    }};
    (einval: $desc:expr) => {{
        err!($crate::error::ErrorKind::InvalidValue, $desc)
    }};
}

/// The error kind
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorKind {
    /// An SPI or reset-related I/O error
    IoError,
    /// A timeout occurred
    Timeout,
    /// A CRC validation failed
    InvalidMessage,
    /// A function argument is invalid
    InvalidValue,
}

/// The crate's error type
#[derive(Debug)]
pub struct Error {
    /// The error kind
    pub kind: ErrorKind,
    /// The file where the error was created
    #[cfg(feature = "backtrace")]
    pub file: &'static str,
    /// The line at which the error was created
    #[cfg(feature = "backtrace")]
    pub line: u32,
    /// A human readable error description
    #[cfg(feature = "backtrace")]
    pub description: &'static str,
}
