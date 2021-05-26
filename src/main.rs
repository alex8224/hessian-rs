mod hessian;

extern crate nom;

use std::fs::{File, read};
use std::io::prelude::*;
use nom::bytes::complete::take;
use nom::number::complete::{be_u32, be_u16, be_u8, be_f32};
use nom::error;
use nom::Err;
use nom::error::ErrorKind;
use std::cmp::min;


#[derive(Debug, PartialEq)]
struct JavaClass {
    header: Header,
    cp_info: Vec<CpInfo>,
}

#[derive(Debug, PartialEq)]
struct Header {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
}

#[derive(Debug, PartialEq)]
struct CpInfo {
    tag: u8,
}


#[derive(Debug, PartialEq)]
struct MethodRef {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug)]
enum ConstantType {
    Class = 7,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
    EOF = 0,
}


impl From<u8> for ConstantType {
    fn from(v: u8) -> Self {
        match v {
            7 => Self::Class,
            9 => Self::Fieldref,
            10 => Self::Methodref,
            11 => Self::InterfaceMethodref,
            8 => Self::String,
            3 => Self::Integer,
            4 => Self::Float,
            5 => Self::Long,
            6 => Self::Double,
            12 => Self::NameAndType,
            1 => Self::Utf8,
            15 => Self::MethodHandle,
            16 => Self::MethodType,
            18 => Self::InvokeDynamic,
            _ => Self::EOF,
        }
    }
}

#[derive(Debug)]
enum AccessFlag {
    PUBLIC,
    FINAL,
    SUPER,
    INTERFACE,
    ABSTRACT,
    SYNTHETIC,
    ANNOTATION,
    ENUM,
}

impl From<u16> for AccessFlag {
    fn from(val: u16) -> Self {
        match val & 0xffff {
            0x0001 => Self::PUBLIC,
            0x0010 => Self::FINAL,
            0x0020 => Self::SUPER,
            0x0200 => Self::INTERFACE,
            0x0400 => Self::ABSTRACT,
            0x1000 => Self::SYNTHETIC,
            0x2000 => Self::ANNOTATION,
            _ => Self::ENUM,
        }
    }
}

type ParseErr<'a> = Err<(&'a [u8], ErrorKind)>;


fn parse_cp_info(input: &[u8]) -> Result<&[u8], ParseErr> {
    use ConstantType::*;
    let (i, tag) = be_u8(input)?.into();
    let c_type = ConstantType::from(tag);
    let i = match c_type {
        Class | String => {
            let (i, name_index) = be_u16(i)?;
            println!("{:?} name_index {}", c_type, name_index);
            i
        }
        Fieldref | Methodref | InterfaceMethodref | NameAndType => {
            let (i, cls_index) = be_u16(i)?;
            let (i, name_type_index) = be_u16(i)?;
            println!("{:?} cls_index {} name_type_index {}", c_type, cls_index, name_type_index);
            i
        }
        Float => {
            let (i, val) = be_f32(i)?;
            println!("{:?} val {} ", c_type, val);
            i
        }
        Integer => {
            let (i, val) = be_u32(i)?;
            println!("{:?} val {} ", c_type, val);
            i
        }
        Utf8 => {
            let (i, length) = be_u16(i)?;
            print!("{:?} len {} ", c_type, length);
            let (i, u8str) = take(length as usize)(i)?;
            println!("{}", std::str::from_utf8(u8str).unwrap());
            i
        }
        _ => {
            println!("{:?} parse constant pool end ", c_type);
            input
        }
    };
    Ok(i)
}

fn parse_java(input: &[u8]) -> Result<(&[u8], JavaClass), ParseErr> {
    let (i, magic) = be_u32(input)?;
    let (i, minor) = be_u16(i)?;
    let (i, major) = be_u16(i)?;
    let (i, count) = be_u16(i)?;
    let mut offset = i;
    let mut cnt = 0;
    for k in 0..count - 1 {
        cnt += 1;
        print!("cnt {} tag  ", k + 1);
        let i = parse_cp_info(offset)?;
        offset = i;
    }

    let (i, access_flag) = be_u16(offset)?;

    let flag = AccessFlag::from(access_flag);

    let (i, this_class) = be_u16(i)?;

    let (i, super_class) = be_u16(i)?;

    let (i, iface_cnt) = be_u16(i)?;

    let (i, field_cnt) = be_u16(i)?;

    Ok((input, JavaClass {
        header: Header {
            magic: magic,
            minor_version: minor,
            major_version: major,
            constant_pool_count: count,
        },
        cp_info: vec![CpInfo { tag: 1 }],
    }))
}


use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "print_constant", about = "打印class文件的常量池")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

use hessian_rs::hessian::Serializer;
use std::time::SystemTime;

fn main() {

    let bin = read("d:/hessian.dat").unwrap();
    let start = SystemTime::now();
    for i in 0..500000 {
        let hessian_de = Serializer::new(bin.as_slice());
        let data = hessian_de.read_binary().unwrap();
        // println!("read data {} bytes", data.len());
    }
    println!("{} ms", start.elapsed().unwrap().as_millis());

    // let opt = Opt::from_args();
    // let path = "d:\\TestPrivate.class";
    // let mut buf= read(opt.input).unwrap();
    // let (input, header) = parse_java(&buf[..]).unwrap();
    // println!("{:?}", header);
}
