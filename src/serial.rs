use std::io;
use std::io::prelude::*;
use spidev::{SPI_MODE_0, Spidev, SpidevOptions, SpidevTransfer};
use globals::SPI_PATH0;

pub fn create_spi() -> io::Result<Spidev> {
    let mut spi = try!(Spidev::open(SPI_PATH0));
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20_000)
        .mode(SPI_MODE_0)
        .build();
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



/// Perform full duplex operations using Ioctl
pub fn full_duplex(spi: &mut Spidev) -> io::Result<()> {
    // "write" transfers are also reads at the same time with
    // the read having the same length as the write
    let tx_buf = [0x01, 0x05, 0x03];
    let mut rx_buf = [0; 3];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        try!(spi.transfer(&mut transfer));
    }
    println!("{:?}", rx_buf);
    Ok(())
}
