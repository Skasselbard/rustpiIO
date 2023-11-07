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

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Result;
use std::io::ErrorKind;
use std::io::Error;
use std::path::Path;
use std::fmt;

use globals::GPIO_PATH;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GPIOMode {
    Read,
    Write,
}

#[derive(Debug, Clone, Copy)]
pub enum GPIOData {
    Low = 0,
    High = 1,
}

/// A software representation for a GPIO pin of the raspberry pi.
pub struct GPIO {
    pin: u8,
    mode: GPIOMode,
}

impl GPIO {
    /// Returns the configured pin of the GPIO struct.
    pub fn gpio_number(&self) -> u8 {
        self.pin
    }

    /// Returns the configured mode of the GPIO struct.
    pub fn current_mode(&self) -> GPIOMode {
        self.mode
    }

    /// Changes the mode of the pin and writes the corresponding value to the fitting direction file
    pub fn set_mode(&mut self, mode: GPIOMode) -> Result<&mut Self> {
        let mut direction = OpenOptions::new()
            .write(true)
            .open(format!("{}gpio{}/direction", GPIO_PATH, self.pin))?;
        match mode {
            GPIOMode::Read => direction.write_all("in".as_bytes())?,
            GPIOMode::Write => direction.write_all("out".as_bytes())?,
        };
        self.mode = mode;
        Ok(self)
    }

    /// Initializes the gpio. Exports the pin with the /sys/class/gpio/export file
    /// and calls the set_mode() function with the given mode.
    /// Returns an Error if the gpio was already exported earlier (inside or outside of the application)
    pub fn new(gpio: u8, mode: GPIOMode) -> Result<Self> {
        if Path::new(&format!("{}gpio{}/", GPIO_PATH, gpio)).exists() {
            return Err(Error::new(
                ErrorKind::AddrInUse,
                "Error: gpio was already initialized",
            ));
        }
        {
            let mut export = OpenOptions::new()
                .write(true)
                .open(format!("{}export", GPIO_PATH))?;
            export.write_all(format!("{}", gpio).as_bytes())?;
        }
        let mut result = GPIO {
            pin: gpio,
            mode,
        };
        result.set_mode(mode)?;
        Ok(result)
    }

    /// Reads the current value of the pin in both Read and Write mode.
    /// Returns an Error if a value other than "1" or "0" is read
    pub fn value(&self) -> Result<GPIOData> {
        let mut value = OpenOptions::new()
            .read(true)
            .open(format!("{}gpio{}/value", GPIO_PATH, self.pin))?;
        let mut buffer = vec![];
        value.read_to_end(&mut buffer)?;
        match buffer[0] as char {
            '0' => Ok(GPIOData::Low),
            '1' => Ok(GPIOData::High),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "read value other than 1 or 0",
            )),
        }
    }

    /// Sets the value of the gpio to HIGH or LOW
    /// Returns an Error if the GPIO::Mode is not Write
    pub fn set(&self, data: GPIOData) -> Result<()> {
        if self.mode != GPIOMode::Write {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                "Error: gpio is not in write mode",
            ));
        }
        let buffer = match data {
            GPIOData::Low => "0",
            GPIOData::High => "1",
        };
        let mut direction = OpenOptions::new()
            .write(true)
            .open(format!("{}gpio{}/value", GPIO_PATH, self.pin))?;
        direction.write_all(buffer.as_bytes())?;
        Ok(())
    }
}

/// Closes the gpio and write its pin number into /sys/class/gpio/unexport
impl Drop for GPIO {
    fn drop(&mut self) {
        if let Ok(mut unexport) = OpenOptions::new()
            .write(true)
            .open(format!("{}unexport", GPIO_PATH))
        {
            if let Err(why) = unexport.write_all(format!("{}", self.pin).as_bytes()) {
                panic!("couldn't close gpio {}: {}", self.pin, why)
            }
        } else {
            panic!("file error: {}")
        }
    }
}

/// Writes "LOW" or "HIGH"
impl fmt::Display for GPIOData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GPIOData::Low => write!(f, "LOW"),
            GPIOData::High => write!(f, "HIGH"),
        }
    }
}

///Writes "Read" or "Write"
impl fmt::Display for GPIOMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GPIOMode::Read => write!(f, "Read"),
            GPIOMode::Write => write!(f, "Write"),
        }
    }
}
