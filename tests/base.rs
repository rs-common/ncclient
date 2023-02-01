#![feature(str_internals)]
use core::str::utf8_char_width;
use std::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
};

#[test]
fn dpy() {
    let mut a = [1u8; 4];
    a[0] = 2;
    println!("{:?}", a)
}

#[test]
fn test_unicode() {
    let a = '代';
    println!("{}", a.len_utf8());
    let mut b = [0u8; 4];
    let c = a.encode_utf8(&mut b);
    println!("{:?}", b);
    unsafe {
        // let x: Vec<u8> = vec![228, 187, 163];
        let x: [u8; 4] = [228, 187, 163, 0];
        let r = String::from_utf8_unchecked(x.to_vec());
        println!("{}", r)
    }
    let v = b.iter_mut();
}

#[test]
fn test_u81() {
    let a = "a";
    let b = a.as_bytes();
    for by in b {
        println!("{}", by);
        println!("{:0>8b}", by); // 0>8 , fill, align, width
        println!("{}", utf8_char_width(*by))
    }
}

#[test]
fn hex_to_int() {
    let v = "ff";
    let x = u8::from_str_radix(v, 16);
    match x {
        Ok(v) => {
            println!("{}", v)
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

#[test]
fn int_to_hex() {
    let i: u8 = 0x7E;
    let a = String::from(i as char);
    println!("{}", a)
}

#[test]
fn test_into() {
    let mut vec: Vec<u8> = Vec::with_capacity(4);
    println!("{}", vec.len());
    vec.push(97);
    vec.push(97);
    vec.push(97);
    vec.push(97);
    vec.push(97);
    println!("{:?}", vec);
    let r = String::from_utf8(vec.clone());
    println!("{:?}", r);
    vec.clear();
    println!("{:?}", vec);
    let r = String::from_utf8(vec);
    println!("{:?}", r)
}

// #[test]
// fn test_borrow() {
//     let a = String::from("abc");
//     let b: &str = a.borrow();
//     let c = &a as &str;
//     println!("{}", b);
//     println!("{}", c);
// }

struct Person {
    name: Name,
    sex: Sex,
    age: Age,
}

struct Name(String);
struct Sex(u8);
struct Age(u8);

impl Borrow<Name> for Person {
    fn borrow(&self) -> &Name {
        &self.name
    }
}

impl Borrow<Sex> for Person {
    fn borrow(&self) -> &Sex {
        &self.sex
    }
}

impl Borrow<Age> for Person {
    fn borrow(&self) -> &Age {
        &self.age
    }
}

// impl Borrow<Age> for &Person {
//     fn borrow(&self) -> &Age {
//         &self.age
//     }
// }

// impl Borrow<Person> for &&Person {
//     fn borrow(&self) -> &Person {
//         *self
//     }
// }

#[test]
fn test_borrow() {
    let person = Person {
        name: Name("daipengyuan".to_string()),
        sex: Sex(1),
        age: Age(35),
    };
    let name: &Name = person.borrow();
    let sex: &Sex = person.borrow();
    let age: &Age = (&person).borrow();
    println!("{},{:p}", name.0, name);
    println!("{},{:p}", sex.0, sex);
    println!("{},{:p}", age.0, age);

    // let ref a = person;
    // let ref b = a;
    // let ref c = b;
    // let ref d = c;
    // let ref e = d;

    // println!("{}", e.age.0)
}

#[test]
fn test_asref() {
    let a = String::from("abc");
    let b: &str = a.as_ref();
    println!("{}", b)
}

#[test]
fn test_col() {
    let a = line!();
    println!("{}", a);
    println!("{}", a)
}
