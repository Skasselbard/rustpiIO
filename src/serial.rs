use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

pub fn create_spi() -> io::Result<Spidev> {
    let mut spi = try!(Spidev::open("/dev/spidev0.0"));
    let mut options = SpidevOptions::new();
          options.bits_per_word(8).max_speed_hz(20_000).mode(SPI_MODE_0);
    try!(spi.configure(&options));
    Ok(spi)
}

/// perform half duplex operations using Read and Write traits
pub fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
    let mut rx_buf = [0_u8; 10];
    try!(spi.write(&[0x01, 0x02, 0x03]));
    try!(spi.read(&mut rx_buf));
    println!("{:?}", rx_buf);
    Ok(())
}

/*
/// Perform full duplex operations using Ioctl
pub fn full_duplex(spi: &mut Spidev) -> io::Result<()> {
    // "write" transfers are also reads at the same time with
    // the read having the same length as the write
    let data = [0x01, 0x02, 0x03];
    let mut transfer = SpidevTransfer::write(&data);//SpidevTransfer::write(&[0x01, 0x02, 0x03]);
    spi.transfer(&mut transfer.rx_buf);
    println!("{:?}", transfer);
    Ok(())
}
*/
