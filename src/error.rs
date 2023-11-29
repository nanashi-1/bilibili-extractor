macro_rules! create_error_enum {
    ($($error_name: ident, $error: path);*) => {
        #[derive(Debug)]
        pub enum Error {
            $($error_name($error),)*
            FromString(String),
        }

        $(
            impl From<$error> for Error {
                fn from(value: $error) -> Self {
                    Self::$error_name(value)
                }
            }
         )*

        impl From<String> for Error {
            fn from(value: String) -> Self {
                Self::FromString(value)
            }
        }

        impl From<&str> for Error {
            fn from(value: &str) -> Self {
                Self::FromString(value.to_string())
            }
        }

        impl ToString for Error {
            fn to_string(&self) -> String {
                match self {
                    $(Error::$error_name(e) => format!("{}: {e}", stringify!($error_name)),)*
                    Error::FromString(e) => e.to_string(),
                }
            }
        }
    };
}

pub type Result<T> = std::result::Result<T, Error>;

create_error_enum!(
IOError, std::io::Error;
SerdeJSONError, serde_json::Error;
ParseIntError, std::num::ParseIntError
);
