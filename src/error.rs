use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::net::AddrParseError;
use std::string::FromUtf8Error;

use quick_xml::DeError;

#[derive(Debug)]
pub enum Error {
    XmlError(quick_xml::Error),
    XmlDeError(quick_xml::de::DeError),
    IoError(io::Error),
    SSHError(ssh2::Error),
    FromUtf8Error(FromUtf8Error),
    AddrParseError(AddrParseError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Error::XmlError(value)
    }
}

impl From<ssh2::Error> for Error {
    fn from(value: ssh2::Error) -> Self {
        Error::SSHError(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::FromUtf8Error(value)
    }
}

impl From<DeError> for Error {
    fn from(value: DeError) -> Self {
        Error::XmlDeError(value)
    }
}

impl From<AddrParseError> for Error {
    fn from(value: AddrParseError) -> Self {
        Error::AddrParseError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::XmlError(err) => f.write_fmt(format_args!(
                r#"{{"error_type":"xml", "msg":"{}"}}"#,
                err.to_string()
            )),
            Error::IoError(err) => f.write_fmt(format_args!(
                r#"{{"error_type":"io", "msg":"{}"}}"#,
                err.to_string()
            )),
            Error::SSHError(err) => f.write_fmt(format_args!(
                r#"{{"error_type":"ssh", "msg":"{}"}}"#,
                err.to_string()
            )),
            Error::FromUtf8Error(err) => f.write_fmt(format_args!(
                r#"{{"error_type":"from_utf8", "msg":"{}"}}"#,
                err.to_string()
            )),
            Error::XmlDeError(err) => f.write_fmt(format_args!(
                r#"{{"error_type":"xml_de", "msg":"{}"}}"#,
                err.to_string()
            )),
            Error::AddrParseError(err) => f.write_fmt(format_args!(
                r#"{{"error_type":"addr_parse", "msg":"{}"}}"#,
                err.to_string()
            )),
        }
    }
}

impl std::error::Error for Error {}
