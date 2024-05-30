use serde::Serialize;

#[derive(Debug)]
pub struct TACommandError(pub anyhow::Error);
impl std::fmt::Display for TACommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0)
    }
}
impl std::error::Error for TACommandError {}
impl Serialize for TACommandError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:#}", self.0))
    }
}
impl From<anyhow::Error> for TACommandError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

pub type TAResult<T> = std::result::Result<T, TACommandError>;
pub trait IntoTAResult<T> {
    fn into_ta_result(self) -> TAResult<T>;
}

impl<T, E> IntoTAResult<T> for std::result::Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn into_ta_result(self) -> TAResult<T> {
        self.map_err(|e| TACommandError(e.into()))
    }
}
impl<T> IntoTAResult<T> for anyhow::Error {
    fn into_ta_result(self) -> TAResult<T> {
        Err(TACommandError(self))
    }
}

pub trait IntoEmptyTAResult<T> {
    fn into_ta_empty_result(self) -> TAResult<T>;
}
impl IntoEmptyTAResult<()> for anyhow::Error {
    fn into_ta_empty_result(self) -> TAResult<()> {
        Err(TACommandError(self))
    }
}

#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err(anyhow::anyhow!($msg)).into_ta_result()
    };
    ($err:expr $(,)?) => {
        return Err(anyhow::anyhow!($err)).into_ta_result()
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(anyhow::anyhow!($fmt, $($arg)*)).into_ta_result()
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            return Err(anyhow::anyhow!(concat!(
                "Condition failed: `",
                stringify!($cond),
                "`"
            ))).into_ta_result();
        }
    };
    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return Err(anyhow::anyhow!($msg)).into_ta_result()
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return Err(anyhow::anyhow!($err)).into_ta_result()
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(anyhow::anyhow!($fmt, $($arg)*)).into_ta_result()
        }
    };
}
