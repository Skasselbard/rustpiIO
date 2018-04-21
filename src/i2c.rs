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
use i2c_linux::I2c;
use i2c_linux;
pub use internal_i2c::{Address, BlockTransfer, BulkTransfer, Master, Message, ReadFlags,
                       ReadWrite, WriteFlags};

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
    pub fn i2c_transfer(&mut self, messages: &mut [i2c_linux::Message]) -> Result<(), <I2CPi as Master>::Error> {
        self.inner.i2c_transfer(messages)
    }
}

impl Master for I2CPi {
    type Error = std::io::Error;
}

impl Address for I2CPi {
    fn set_slave_address(&mut self, addr: u16, tenbit: bool) -> Result<(), Self::Error> {
        self.inner.smbus_set_slave_address(addr, tenbit)
    }
}

impl BlockTransfer for I2CPi {
    fn i2c_read_block_data(&mut self, command: u8, value: &mut [u8]) -> Result<usize, Self::Error> {
        self.inner.i2c_read_block_data(command, value)
    }
    fn i2c_write_block_data(&mut self, command: u8, value: &[u8]) -> Result<(), Self::Error> {
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
