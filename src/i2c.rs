use std;
use i2c_linux::{I2c, Message, ReadFlags, WriteFlags};
use internal_i2c;
use internal_i2c::{Address, BlockTransfer, BulkTransfer, Master, ReadWrite};

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

// impl BulkTransfer for I2CPi{
//     fn i2c_transfer_support(
//         &mut self
//     ) -> Result<(internal_i2c::ReadFlags, internal_i2c::WriteFlags), Self::Error>{
//         self.inner.i2c_transfer_support()
//     }
//     fn i2c_transfer(
//         &mut self,
//         messages: &mut [internal_i2c::Message]
//     ) -> Result<(), Self::Error>{
//         self.inner.i2c_transfer(messages)
//     }
// }
