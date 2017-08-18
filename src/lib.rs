// This file is part of RustpiIO.
//
// Copyright 2017
//
// Contributors: Tom Meyer
//
// RustpiIO is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RustpiIO is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RustpiIO.  If not, see <http://www.gnu.org/licenses/>

//! # DocuSchmocu
//! RustpiIO is a library to read and write the GPIO pins of a Raspberry Pi.
//!
//! It uses the system interface under /sys/class/gpio/ provided by the linux OS.
//!
//! # Installation
//! To compile a raspberry pi program you need to prepare a cross compiler for rust:
//!
//! * `rustup target add arm-unknown-linux-gnueabihf`
//! * `sudo apt-get install gcc-arm-linux-gnueabihf`
//!
//! To tell the linker which program to use add the following lines to a corresponding
//! ./cargo/config file (like in this project)
//!
//! `[target.arm-unknown-linux-gnueabihf]`
//!
//! `linker="arm-linux-gnueabihf-gcc"`
//!
//! Build for the Raspberry with `cargo build --target=arm-unknown-linux-gnueabihf`
//!
//! #Example
//!```
//! extern crate rustpi_io;
//! use rustpi_io::*;
//!
//! fn main() {
//!     let gpio2 = match GPIO::init(2, GPIOMode::Write){
//!     Ok(result) => result,
//!     Err(e) => panic!("{:?}", e),
//!     };
//!     let gpio3 = match GPIO::init(3, GPIOMode::Read){
//!         Ok(result) => result,
//!         Err(e) => panic!("{:?}", e),
//!     };
//!     let mut value:u8 = 1;
//!     for n in 1..100 {
//!         value = 1-value;
//!         let data = match value {
//!             0 => GPIOData::Low,
//!             1 => GPIOData::High,
//!             _ => GPIOData::High
//!         };
//!         match gpio2.set(data) {
//!             Ok(_) => {},
//!             Err(e) => panic!("Schmerror{:?}", e),
//!         }
//!         match gpio3.value(){
//!             Ok(data) => println!("value: {}", data),
//!             Err(e) => panic!("{:?}", e),
//!         }
//!     }
//! }
//!```

extern crate spidev;

mod globals;
pub mod gpio;
pub mod pi;
pub mod serial;
