//! The crate's error types

/// Creates an error
#[macro_export]
macro_rules! err {
    ($kind:tt, $desc:expr) => {{
        $kind {
            #[cfg(feature = "backtrace")]
            file: file!(),
            #[cfg(feature = "backtrace")]
            line: line!(),
            #[cfg(feature = "backtrace")]
            description: $desc,
        }
    }};
}

/// An I/O error
#[derive(Debug, Clone, Copy)]
pub struct IoError {
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

/// A timeout error
#[derive(Debug, Clone, Copy)]
pub struct TimeoutError {
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

/// A CRC-validation or format error
#[derive(Debug, Clone, Copy)]
pub struct InvalidMessageError {
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

/// An invalid-argument error
#[derive(Debug, Clone, Copy)]
pub struct InvalidArgumentError {
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

/// An TX-start error
#[derive(Debug, Clone, Copy)]
pub enum TxStartError {
    /// An I/O error
    IoError(IoError),
    /// An invalid-argument error
    InvalidArgumentError(InvalidArgumentError),
}
impl From<IoError> for TxStartError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}
impl From<InvalidArgumentError> for TxStartError {
    fn from(error: InvalidArgumentError) -> Self {
        Self::InvalidArgumentError(error)
    }
}

/// An RX-start error
#[derive(Debug, Clone, Copy)]
pub enum RxStartError {
    /// An I/O error
    IoError(IoError),
    /// An invalid-argument error
    InvalidArgumentError(InvalidArgumentError),
}
impl From<IoError> for RxStartError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}
impl From<InvalidArgumentError> for RxStartError {
    fn from(error: InvalidArgumentError) -> Self {
        Self::InvalidArgumentError(error)
    }
}

/// An RX-completion specific error
#[derive(Debug, Clone, Copy)]
pub enum RxCompleteError {
    /// An I/O error
    IoError(IoError),
    /// A timeout error
    TimeoutError(TimeoutError),
    /// A CRC-validation or format error
    InvalidMessageError(InvalidMessageError),
}
impl From<IoError> for RxCompleteError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}
impl From<TimeoutError> for RxCompleteError {
    fn from(error: TimeoutError) -> Self {
        Self::TimeoutError(error)
    }
}
impl From<InvalidMessageError> for RxCompleteError {
    fn from(error: InvalidMessageError) -> Self {
        Self::InvalidMessageError(error)
    }
}
