#![allow(unused)]
use crate::error::Error;
use crate::rpc::{ser_string, Hello};
use ssh2::Session;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

pub struct SSH {
    ch: ssh2::Channel,
    cap: Vec<String>,
}

impl SSH {
    pub fn new(
        target: &str,
        username: &str,
        password: &str,
        cap_list: Option<Vec<String>>,
    ) -> Result<Box<SSH>, Error> {
        let tcp =
            TcpStream::connect_timeout(&SocketAddr::from_str(target)?, Duration::from_secs(1))?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(username, password)?;
        if !session.authenticated() {
            let ecode = ssh2::ErrorCode::Session(-18);
            return Err(Error::from(ssh2::Error::from_errno(ecode)));
        }
        let mut ch = session.channel_session()?;
        ch.subsystem("netconf")?;
        let mut session = SSH {
            ch,
            cap: Vec::new(),
        };
        let _ = session.read();
        session.hello(cap_list)?;
        Ok(Box::new(session))
    }

    fn read(&mut self) -> Result<String, Error> {
        let mut result = String::new();
        loop {
            let mut buffer = [0u8; 4096];
            let bytes_read = self.ch.read(&mut buffer[..])?;
            let s = String::from_utf8_lossy(&buffer[..bytes_read]);
            result.push_str(&s);
            if result.ends_with("]]>]]>") || bytes_read == 0 || self.ch.eof() {
                break;
            }
        }
        println!("{}", result);
        Ok(result)
    }

    fn hello(&mut self, caps: Option<Vec<String>>) -> Result<(), Error> {
        let hello = match caps {
            Some(caps) => Hello::with_caps(caps),
            None => Hello::new(),
        };
        let s = ser_string(hello, false)?;
        self.ch.write(s.as_bytes())?;
        Ok(())
    }

    // fn send_hello(&mut self, cap: Option<Vec<String>>) -> Result<(), Error> {
    //     let mut writer = quick_xml::Writer::new(Vec::new());
    //     writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    //     let mut hello_start = BytesStart::new("hello");
    //     hello_start.push_attribute(Attribute::from(("xmlns", BASE_XML_NS)));
    //     let hello_end = BytesEnd::new("hello");
    //     writer.write_event(Event::Start(hello_start))?;
    //     writer.write_event(Event::Start(BytesStart::new("capabilities")))?;
    //     writer.write_event(Event::Start(BytesStart::new("capability")))?;
    //     writer.write_event(Event::Text(BytesText::new(BASE_NS_1_0)))?;
    //     writer.write_event(Event::End(BytesEnd::new("capability")))?;
    //     writer.write_event(Event::Start(BytesStart::new("capability")))?;
    //     writer.write_event(Event::Text(BytesText::new(BASE_NS_1_1)))?;
    //     writer.write_event(Event::End(BytesEnd::new("capability")))?;
    //     if let Some(cap_list) = cap {
    //         if cap_list.len() > 0 {
    //             for cap in cap_list {
    //                 writer.write_event(Event::Start(BytesStart::new("capability")))?;
    //                 writer.write_event(Event::Text(BytesText::new(cap.as_str())))?;
    //                 writer.write_event(Event::End(BytesEnd::new("capability")))?;
    //             }
    //         }
    //     }
    //     writer.write_event(Event::End(BytesEnd::new("capabilities")))?;
    //     writer.write_event(Event::End(hello_end))?;
    //     let content = writer.inner();
    //     content.append(&mut Vec::from(SSH_END_IDENT));
    //     println!("{}", String::from_utf8(content.to_vec())?);
    //     self.ch.write(content)?;
    //     Ok(())
    // }
}

