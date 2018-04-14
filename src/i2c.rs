
use std;
use i2c_linux::I2c;
use internal_i2c::{Address, Master, ReadWrite, BlockTransfer, BulkTransfer};

pub struct I2CPi {}

impl I2CPi{
    pub fn new() -> std::io::Result<I2CPi> {
        Ok(I2CPi {})
    }
}


