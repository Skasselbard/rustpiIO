use std::io;
use std::io::{Error, ErrorKind};
use spidev::{SPI_MODE_0, SPI_MODE_1, SPI_MODE_2, SPI_MODE_3, Spidev, SpidevOptions, SpidevTransfer};
use globals::{SPI_PATH0, SPI_PATH1};
use std::io::{BufRead, Read, Write};

#[derive(PartialEq)]
pub enum Device {
    CE0 = 0,
    CE1 = 1,
}

/** 
 From https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md#driver  
 Possible Speeds:  
    125.0 MHz  
    62.5 MHz  
    31.2 MHz  
    15.6 MHz  
    7.8 MHz  
    3.9 MHz  
    1953 kHz  
    976 kHz  
    488 kHz  
    244 kHz  
    122 kHz  
    61 kHz  
    30.5 kHz  
    15.2 kHz  
    7629 Hz  
 */
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Speed {
    Mhz125_0,
    Mhz62_5,
    Mhz31_2,
    Mhz15_6,
    Mhz7_8,
    Mhz3_9,
    Khz1953,
    Khz976,
    Khz488,
    Khz244,
    Khz122,
    Khz61,
    Khz30_5,
    Khz15_2,
    Hz7629,
}


impl Speed {
    /// Converts the `Speed` variants into an integer representing the Hz value
    fn to_int(&self) -> u32 {
        match *self {
            Speed::Mhz125_0 => 125_000_001,
            Speed::Mhz62_5 => 62_500_001,
            Speed::Mhz31_2 => 31_200_001,
            Speed::Mhz15_6 => 15_600_001,
            Speed::Mhz7_8 => 7_800_001,
            Speed::Mhz3_9 => 3_900_001,
            Speed::Khz1953 => 1_935_001,
            Speed::Khz976 => 976_001,
            Speed::Khz488 => 488_001,
            Speed::Khz244 => 244_001,
            Speed::Khz122 => 122_001,
            Speed::Khz61 => 61_001,
            Speed::Khz30_5 => 30_501,
            Speed::Khz15_2 => 15_201,
            Speed::Hz7629 => 7_630,
        }
    }
}

/**
The most common spi modes. regulating the clock edge and polariy.  
Mode 0 seems to be the most used one.  
See https://en.wikipedia.org/wiki/Serial_Peripheral_Interface_Bus#Clock_polarity_and_phase f. for an explanation 
*/
#[derive(PartialEq)]
pub enum SpiMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

#[derive(PartialEq)]
pub enum ComMode {
    FullDuplex,
    HalfDuplex,
}

fn spi_open_error() -> Error {
    Error::new(
        ErrorKind::NotFound,
        "Error: Unable to open the spi device. Did you set \"dtparam=spi=on\" in /boot/config.txt?",
    )
}

pub struct SerialPi {
    device: Spidev,
    pub com_mode: ComMode,
    read_buffer: Vec<u8>,
}

impl SerialPi {
    pub fn new(
        device: Device,
        speed: Speed,
        spi_mode: SpiMode,
        communication_mode: ComMode,
    ) -> io::Result<SerialPi> {
        //TODO: Check that correponding GPIOS are free
        let mut spi = match device {
            Device::CE0 => match Spidev::open(SPI_PATH0) {
                Err(_) => return Err(spi_open_error()),
                Ok(device) => device,
            },
            Device::CE1 => match Spidev::open(SPI_PATH1) {
                Err(_) => return Err(spi_open_error()),
                Ok(device) => device,
            },
        };
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(speed.to_int())
            .mode(match spi_mode {
                SpiMode::Mode0 => SPI_MODE_0,
                SpiMode::Mode1 => SPI_MODE_1,
                SpiMode::Mode2 => SPI_MODE_2,
                SpiMode::Mode3 => SPI_MODE_3,
            })
            .lsb_first(false)
            .build();
        try!(spi.configure(&options));
        Ok(SerialPi {
            device: spi,
            com_mode: communication_mode,
            read_buffer: Vec::with_capacity(100),
        })
    }
}

impl Read for SerialPi {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buffer_read_count = try!(self.read_buffer.as_slice().read(buf));
        self.read_buffer = self.read_buffer.split_off(buffer_read_count);
        if buffer_read_count < buf.len() {
            let (_, rest_buffer) = buf.split_at_mut(buffer_read_count);
            buffer_read_count = buffer_read_count + try!(self.device.read(rest_buffer));
        }
        Ok(buffer_read_count)
    }
}

impl BufRead for SerialPi {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        {
            let buffer_length = self.read_buffer.len();
            let (_, rest_buffer) = self.read_buffer.as_mut_slice().split_at_mut(buffer_length);
            try!(self.device.read(rest_buffer));
        }
        Ok(self.read_buffer.as_slice())
    }
    fn consume(&mut self, amt: usize) {
        self.read_buffer.drain(0..(amt - 1));
    }
}

impl Write for SerialPi {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.com_mode == ComMode::HalfDuplex {
            self.device.write(buf)
        } else {
            let mut read_data: Vec<u8> = Vec::with_capacity(buf.len());
            read_data.resize(buf.len(), 0 as u8);
            {
                let mut transfer = SpidevTransfer::read_write(buf, read_data.as_mut_slice());
                try!(self.device.transfer(&mut transfer));
            }
            self.read_buffer.append(&mut read_data);
            Ok(buf.len())
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        if self.com_mode == ComMode::HalfDuplex {
            self.device.flush()
        } else {
            Ok(())
        }
    }
}
