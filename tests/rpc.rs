#![allow(unused)]

use std::{
    io::{self, Cursor, Write},
    process::id, cell::BorrowError, borrow::BorrowMut,
};

use ncclient::{
    error::Error,
    rpc::{Deserialize, Deserializer, Hello, Rpc, RpcReply, Serialize, Serializer},
};
use quick_xml::{de::from_str, Reader, Writer};

#[test]
fn hello_ser() {
    let hello = Hello::new();
    let mut ser = Serializer::with_root(String::new(), None).unwrap();
    ser.indent(' ', 2);
    let mut r = hello.serialize(ser).unwrap();
    // let rs = Writer::new(r.as_bytes().to_vec());
    println!("{}", r)
}

#[test]
fn hellow_de() {
    let content = include_str!("documents/standard_hello.xml");
    let mut de = Deserializer::from_str(content);
    let result = Hello::deserialize(&mut de).unwrap();
    println!("会话ID={:?}", result.session_id());
    for (idx, cap) in result.capabilities().iter().enumerate() {
        println!("能力{},值= {}", idx + 1, cap)
    }
}

#[test]
fn simple_cursor() -> Result<(), Error> {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    cursor.write("sss".as_bytes());
    cursor.write("ddd".as_bytes());
    let a = cursor.into_inner();
    let b = String::from_utf8(a)?;
    println!("{}", b);
    Ok(())
}
