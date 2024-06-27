use std::path::Path;

use phf::phf_map;

pub mod c_sharp;
pub mod c;
pub mod go;
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod lua;
pub mod mojo;
pub mod python;
pub mod rust;
pub mod typescript;
pub mod zig;

use c_sharp::write_files as c_sharp_write_files;
use c::write_files as c_write_files;
use go::write_files as go_write_files;
use java::write_files as java_write_files;
use javascript::write_files as javascript_write_files;
use kotlin::write_files as kotlin_write_files;
use lua::write_files as lua_write_files;
use mojo::write_files as mojo_write_files;
use python::write_files as python_write_files;
use rust::write_files as rust_write_files;
use typescript::write_files as typescript_write_files;
use zig::write_files as zig_write_files;

use crate::model::File;

type Writer = fn(&Path, &Vec<File>) -> anyhow::Result<()>;


pub static WRITERS: phf::Map<&'static str, Writer> = phf_map! {
    "c_sharp" => c_sharp_write_files,
    "c" => c_write_files,
    "go" => go_write_files,
    "java" => java_write_files,
    "javascript" => javascript_write_files,
    "kotlin" => kotlin_write_files,
    "lua" => lua_write_files,
    "mojo" => mojo_write_files,
    "python" => python_write_files,
    "rust" => rust_write_files, 
    "typescript" => typescript_write_files,
    "zig" => zig_write_files,
};