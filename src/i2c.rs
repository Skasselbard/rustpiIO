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
pub use i2c_linux::{Message, ReadFlags, WriteFlags, Functionality, ReadWrite};
use i2c_linux;
use i2c_linux::{I2c};
use std::time::Duration;

pub struct I2CPi {
    inner: I2c<std::fs::File>,
}

impl I2CPi {
    pub fn new() -> std::io::Result<I2CPi> {
        let i2c = match I2c::from_path("/dev/i2c-0") {
            Ok(device) => device,
            Err(_) => I2c::from_path("/dev/i2c-1")?,
        };
        Ok(I2CPi { inner: i2c })
    }

    pub fn slave_address(&mut self, addr: u16, tenbit: bool) -> std::io::Result<()>{
        self.inner.smbus_set_slave_address(addr, tenbit)
    }

    pub fn functionality(&self) -> std::io::Result<Functionality> {
        self.inner.i2c_functionality()
    }

    pub fn transfer(&mut self, messages: &mut [Message]) -> std::io::Result<()> {
        self.inner.i2c_transfer(messages)
    }

    pub fn transfer_flags(&self) -> std::io::Result<(ReadFlags, WriteFlags)>{
        self.inner.i2c_transfer_flags()
    }

    pub fn set_retries(&self, value: usize) -> std::io::Result<()>{
        self.inner.i2c_set_retries(value)
    }

    pub fn set_timeout(&self, duration: Duration) -> std::io::Result<()>{
        self.inner.i2c_set_timeout(duration)
    }
    
    pub fn read_block_data(&mut self, command: u8, value: &mut [u8]) -> std::io::Result<usize> {
        self.inner.i2c_read_block_data(command, value)
    }
    pub fn write_block_data(&mut self, command: u8, value: &[u8]) -> std::io::Result<()> {
        self.inner.i2c_write_block_data(command, value)
    }
}

impl std::io::Read for I2CPi {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl std::io::Write for I2CPi {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

pub struct SMBusPi {
    inner: I2c<std::fs::File>,
}

impl SMBusPi {
    pub fn new() -> std::io::Result<I2CPi> {
        let i2c = match I2c::from_path("/dev/i2c-0") {
            Ok(device) => device,
            Err(_) => I2c::from_path("/dev/i2c-1")?,
        };
        Ok(I2CPi { inner: i2c })
    }

    pub fn slave_address(&mut self, addr: u16, tenbit: bool) -> std::io::Result<()>{
        self.inner.smbus_set_slave_address(addr, tenbit)
    }

    pub fn functionality(&self) -> std::io::Result<Functionality> {
        self.inner.i2c_functionality()
    }

    pub fn transfer(&mut self, messages: &mut [Message]) -> std::io::Result<()> {
        self.inner.i2c_transfer(messages)
    }

    pub fn transfer_flags(&self) -> std::io::Result<(ReadFlags, WriteFlags)>{
        self.inner.i2c_transfer_flags()
    }

    pub fn set_retries(&self, value: usize) -> std::io::Result<()>{
        self.inner.i2c_set_retries(value)
    }

    pub fn set_timeout(&self, duration: Duration) -> std::io::Result<()>{
        self.inner.i2c_set_timeout(duration)
    }



    pub fn write_bit(&mut self, value: ReadWrite) -> std::io::Result<()> {
        self.inner.smbus_write_quick( value)
    }

    pub fn read_primitive(&mut self) -> std::io::Result<u8> {
        self.inner.smbus_read_byte()
    }

    pub fn write_primitive(&mut self, value: u8) -> std::io::Result<()> {
        self.inner.smbus_write_byte(value)
    }

    pub fn read_byte(&mut self, command: u8) -> std::io::Result<u8> {
        self.inner.smbus_read_byte_data(command)
    }

    pub fn write_byte(&mut self, command: u8, value: u8) -> std::io::Result<()> {
        self.inner.smbus_write_byte_data(command, value)
    }

    pub fn read_word(&mut self, command: u8) -> std::io::Result<u16> {
        self.inner.smbus_read_word_data(command)
    }

    pub fn write_word(&mut self, command: u8, value: u16) -> std::io::Result<()> {
        self.inner.smbus_write_word_data(command, value)
    }

    pub fn process_call(&mut self, command: u8, value: u16) -> std::io::Result<u16> {
        self.inner.smbus_process_call(command, value)
    }

    pub fn read_block_data(&mut self, command: u8, values: &mut [u8]) -> std::io::Result<usize> {
        self.inner.smbus_read_block_data(command, values)
    }

    pub fn write_block_data(&mut self, command: u8, value: &[u8]) -> std::io::Result<()>{
        self.inner.smbus_write_block_data(command, value)
    }

    pub fn block_process_call(&mut self, command: u8, write: &[u8], read: &mut [u8]) -> std::io::Result<usize> {
        self.inner.smbus_block_process_call(command, write, read)
    }
}

impl std::io::Read for SMBusPi {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl std::io::Write for SMBusPi {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
