// This file is part of RustpiIO.
//
// Copyright 2018
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

//!
//! Based on https://www.raspberrypi.org/documentation/hardware/raspberrypi/revision-codes/README.md
//!

use globals::RASPI_INFO_PATH;
use std::fmt;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum RevisionStyle {
    Old = 0,
    New = 1,
}

impl RevisionStyle {
    pub fn from(number: u32) -> Option<RevisionStyle> {
        match number {
            0 => Some(RevisionStyle::Old),
            1 => Some(RevisionStyle::New),
            _ => None,
        }
    }
}

impl fmt::Display for RevisionStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevisionStyle::Old => write!(f, "old"),
            RevisionStyle::New => write!(f, "new"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum MemorySize {
    MB256 = 0,
    MB512 = 1,
    MB1024 = 2,
}

impl fmt::Display for MemorySize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MemorySize::MB256 => write!(f, "256MB"),
            MemorySize::MB512 => write!(f, "512MB"),
            MemorySize::MB1024 => write!(f, "1024MB"),
        }
    }
}

impl MemorySize {
    pub fn from(number: u32) -> Option<MemorySize> {
        match number {
            0 => Some(MemorySize::MB256),
            1 => Some(MemorySize::MB512),
            2 => Some(MemorySize::MB1024),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Manufacturer {
    SonyUK = 0,
    Egoman = 1,
    Embest = 2,
    SonyJapan = 3,
}

impl fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Manufacturer::SonyUK => write!(f, "SonyUK"),
            Manufacturer::Egoman => write!(f, "Egoman"),
            Manufacturer::Embest => write!(f, "Embest"),
            Manufacturer::SonyJapan => write!(f, "SonyJapan"),
        }
    }
}

impl Manufacturer {
    pub fn from(number: u32) -> Option<Manufacturer> {
        match number {
            0 => Some(Manufacturer::SonyUK),
            1 => Some(Manufacturer::Egoman),
            2 => Some(Manufacturer::Embest),
            3 => Some(Manufacturer::SonyJapan),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Processor {
    BCM2835 = 0,
    BCM2836 = 1,
    BCM2837 = 2,
}

impl fmt::Display for Processor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Processor::BCM2835 => write!(f, "BCM2835"),
            Processor::BCM2836 => write!(f, "BCM2836"),
            Processor::BCM2837 => write!(f, "BCM2837"),
        }
    }
}

impl Processor {
    pub fn from(number: u32) -> Option<Processor> {
        match number {
            0 => Some(Processor::BCM2835),
            1 => Some(Processor::BCM2836),
            2 => Some(Processor::BCM2837),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    A = 0,
    B = 1,
    APlus = 2,
    BPlus = 3,
    B2 = 4,
    Alpha = 5,
    CM1 = 6,
    B3 = 8,
    Zero = 9,
    CM3 = 10,   //a
    ZeroW = 12, //c
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::A => write!(f, "A"),
            Type::B => write!(f, "B"),
            Type::APlus => write!(f, "APlus"),
            Type::BPlus => write!(f, "BPlus"),
            Type::B2 => write!(f, "B2"),
            Type::Alpha => write!(f, "Alpha"),
            Type::CM1 => write!(f, "CM1"),
            Type::B3 => write!(f, "B3"),
            Type::Zero => write!(f, "Zero"),
            Type::CM3 => write!(f, "CM3"),
            Type::ZeroW => write!(f, "ZeroW"),
        }
    }
}

impl Type {
    pub fn from(number: u32) -> Option<Type> {
        match number {
            0 => Some(Type::A),
            1 => Some(Type::B),
            2 => Some(Type::APlus),
            3 => Some(Type::BPlus),
            4 => Some(Type::B2),
            5 => Some(Type::Alpha),
            6 => Some(Type::CM1),
            8 => Some(Type::B3),
            9 => Some(Type::Zero),
            10 => Some(Type::CM3),
            12 => Some(Type::ZeroW),
            _ => None,
        }
    }
}

/// A function to extract information about the pi's hardware.
/// The /proc/cpuinfo file is converted to progamatically usable data.
/// The last element of the tuple is the revision of the raspberry model.
pub fn get_raspberry_info() -> Result<(MemorySize, Manufacturer, Processor, Type, u32)> {
    //open file and read data
    let mut file = File::open(RASPI_INFO_PATH).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    //parse
    let position = data.rfind("Revision").unwrap();
    let (_, revision_line) = data.split_at(position);
    let mut iterator = revision_line.split_whitespace();
    iterator.next();
    iterator.next();
    let revision_string = iterator.next().unwrap();
    //convert to integer
    let revision_int = u32::from_str_radix(revision_string, 16).unwrap();
    if RevisionStyle::from((revision_int >> 23) & 0b1u32).unwrap() == RevisionStyle::Old {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Found old revision style which is not supported yet",
        ));
    }
    Ok((
        MemorySize::from((revision_int >> 20) & 0b111u32).unwrap(),
        Manufacturer::from((revision_int >> 16) & 0b1111u32).unwrap(),
        Processor::from((revision_int >> 12) & 0b1111u32).unwrap(),
        Type::from((revision_int >> 4) & 0b11111111u32).unwrap(),
        revision_int & 0b1111u32,
    ))
}
