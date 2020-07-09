// #![cfg(not(target_pointer_width = "64"))]
// compile_error!("This crate can only be used on 64-bit systems.");

// pub mod error;
pub mod class;
// pub mod exec;
// pub mod memory;
// pub mod vm;
pub mod instruction;
pub mod types;
pub mod classloader;

pub use pom;