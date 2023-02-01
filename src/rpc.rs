#![allow(unused)]
// #![feature(linked_list_cursors)]

use crate::{consts::*, error::Error};
pub use quick_xml::de::Deserializer;
pub use quick_xml::se::Serializer;
use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesStart, Event},
    Reader, Writer,
};
pub use serde::{Deserialize, Serialize};
use std::{io::Cursor, path::Path};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "hello")]
pub struct Hello {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<u16>,
    capabilities: Capabilities,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "capabilities")]
pub struct Capabilities {
    #[serde(rename = "capability")]
    capability: Vec<String>,
}

impl Hello {
    pub fn new() -> Self {
        let dft_caps: Vec<String> = vec![BASE_NS_1_0.into(), BASE_NS_1_1.into()];
        Hello {
            xmlns: BASE_XML_NS.into(),
            session_id: None,
            capabilities: Capabilities {
                capability: dft_caps,
            },
        }
    }
    pub fn with_caps(mut caps: Vec<String>) -> Self {
        let mut hello = Hello::new();
        let mut old_cap = &mut hello.capabilities.capability;
        old_cap.append(&mut caps);
        hello
    }

    pub fn session_id(&self) -> Option<u16> {
        self.session_id
    }

    pub fn capabilities(&self) -> Vec<String> {
        self.capabilities.capability.to_vec()
    }
}

#[derive(Debug)]
pub struct Rpc(Cursor<Vec<u8>>);

#[derive(Debug)]
pub struct RpcReply(Vec<u8>);

pub fn ser_string<T: Serialize>(t: T, indent: bool) -> Result<String, Error> {
    let mut ser = Serializer::with_root(String::new(), None)?;
    if indent {
        ser.indent(' ', 2);
    }
    let mut result = t.serialize(ser)?;
    Ok(result)
}
