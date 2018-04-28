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

use std;
use i2cdev::linux::*;
use i2cdev::core::*;
use std::time::Duration;
pub use i2cdev::linux::LinuxI2CError;

pub struct I2CPi {
    inner: LinuxI2CDevice,
}

impl I2CPi {
    

    pub fn new(slave_address: u16) -> Result<Self, LinuxI2CError> {
        let i2c = match LinuxI2CDevice::new("/dev/i2c-0", slave_address) {
            Ok(device) => device,
            Err(_) => LinuxI2CDevice::new("/dev/i2c-1", slave_address)?,
        };
        Ok(I2CPi { inner: i2c })
    }

    pub fn change_slave_address(&mut self, addr: u16) -> Result<(), LinuxI2CError>{
        self.inner.set_slave_address(addr)
    }

    pub fn write_bit(&mut self, read_write_bit: bool) -> Result<(), LinuxI2CError> {
        self.inner.smbus_write_quick( read_write_bit)
    }

    pub fn read_primitive(&mut self) -> Result<u8, LinuxI2CError> {
        self.inner.smbus_read_byte()
    }

    pub fn write_primitive(&mut self, value: u8) -> Result<(), LinuxI2CError> {
        self.inner.smbus_write_byte(value)
    }

    pub fn read_byte(&mut self, command: u8) -> Result<u8, LinuxI2CError> {
        self.inner.smbus_read_byte_data(command)
    }

    pub fn write_byte(&mut self, command: u8, value: u8) -> Result<(), LinuxI2CError> {
        self.inner.smbus_write_byte_data(command, value)
    }

    pub fn read_word(&mut self, command: u8) -> Result<u16, LinuxI2CError> {
        self.inner.smbus_read_word_data(command)
    }

    pub fn write_word(&mut self, command: u8, value: u16) -> Result<(), LinuxI2CError> {
        self.inner.smbus_write_word_data(command, value)
    }

    pub fn process_word(&mut self, command: u8, value: u16) -> Result<u16, LinuxI2CError> {
        self.inner.smbus_process_word(command, value)
    }

    pub fn read_block_data(&mut self, command: u8) -> Result<Vec<u8>, LinuxI2CError> {
        self.inner.smbus_read_block_data(command)
    }

    pub fn write_block_data(&mut self, command: u8, value: &[u8]) -> Result<(), LinuxI2CError>{
        self.inner.smbus_write_block_data(command, value)
    }

    pub fn process_block(&mut self, command: u8, values: &mut [u8]) -> Result<Vec<u8>, LinuxI2CError> {
        self.inner.smbus_process_block(command, values)
    }
}

// impl std::io::Read for I2CPi {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.inner.read(buf)
//     }
// }

// impl std::io::Write for I2CPi {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.inner.write(buf)
//     }
//     fn flush(&mut self) -> std::io::Result<()> {
//         self.inner.flush()
//     }
// }