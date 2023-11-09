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

use globals::{SPI_PATH0, SPI_PATH1};
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::io;
use std::io::{BufRead, Read, Write};
use std::io::{Error, ErrorKind};

/**
 * Correspond to the SPI Chip Enable Pins on the raspberry pi.
 */
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
The most common spi modes regulating the clock phase and polarity.
Mode 0 seems to be the most used one and is set as default.
See https://en.wikipedia.org/wiki/Serial_Peripheral_Interface_Bus#Clock_polarity_and_phase for an explanation
*/
#[derive(PartialEq, Default)]
pub enum SpiMode {
    #[default]
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

impl Default for ComMode {
    /** ComMode::FullDuplex */
    fn default() -> Self {
        ComMode::FullDuplex
    }
}

fn spi_open_error() -> Error {
    Error::new(
        ErrorKind::NotFound,
        "Error: Unable to open the spi device. Did you set \"dtparam=spi=on\" in /boot/config.txt?",
    )
}

/**
 * Interface for the spi bus of the Raspberry Pi.
 *
 * A SerialPi spi struct can be opened to access the serial interface of the Raspberry Pi as Master. It will initialize the necessary components for the communication. A device, a bus clock, a spi mode and a communication mode have to be configured.
 *
 * The device correspond to the Chip Enable Pins of the raspberry pi. Look at the pin layout of of your raspberry pi to determine which pin is associated with which CE device.
 *
 * The bus clock determines the communication speed. Use a speed that your slave device can handle.
 *
 * The spi mode sets clock polarity and clock phase of the transmission. The configuration has to match with the slave device. Mode 0 is said to be the most common. For an explanation you can go to the [wikipedia spi article](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface_Bus#Clock_polarity_and_phase).
 *
 * The communication mode can be Full or HalfDuplex. In full duplex mode, for every byte that is send, a byte will be received. In half duplex mode the bytes that are received during a transmission are ignored.
 *
 * In full duplex mode, the data that was received with `write` will be buffered internally. Calls to the `Read` and `BufRead` trait implementation will first read out the buffered content and will issue an actual spi read if the buffer is exhausted.
 * The buffer will be reallocated if the last read bytes do not fit. It will preserve the capacity after that until a resize is issued with `try_shrink_to`.
 *
 * In half duplex mode the buffer will not be filled when calling write, but it will be consumed when calling read.
 */
pub struct SerialPi {
    device: Spidev,
    pub com_mode: ComMode,
    read_buffer: Vec<u8>,
}

impl SerialPi {
    /**
     * Calls `with_capacity` with a buffer size of 1000 bytes.
     */
    pub fn new(
        device: Device,
        speed: Speed,
        spi_mode: SpiMode,
        communication_mode: ComMode,
    ) -> io::Result<SerialPi> {
        SerialPi::with_capacity(device, speed, spi_mode, communication_mode, 1000)
    }

    /**
     * Creates a serial wrapper for the raspberry pi. Also sets the buffers capacity.
     *
     * Note: The SpiMode and the ComMode have a default value.
     *
     * # Errors
     * Can return an error if the spi device can't be opened. It might be already in use or the raspberry is not configured correctly. Check the [documentation](https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md#overview) of the raspberry pi in this case.
     */
    pub fn with_capacity(
        device: Device,
        speed: Speed,
        spi_mode: SpiMode,
        communication_mode: ComMode,
        buffer_capacity: usize,
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
                SpiMode::Mode0 => SpiModeFlags::SPI_MODE_0,
                SpiMode::Mode1 => SpiModeFlags::SPI_MODE_1,
                SpiMode::Mode2 => SpiModeFlags::SPI_MODE_2,
                SpiMode::Mode3 => SpiModeFlags::SPI_MODE_3,
            })
            .lsb_first(false)
            .build();
        spi.configure(&options)?;
        Ok(SerialPi {
            device: spi,
            com_mode: communication_mode,
            read_buffer: Vec::with_capacity(buffer_capacity),
        })
    }

    /**
     * Returns the current capacity of the internal buffer. To change it call `try_shrink_to`. Maybe you have to `consume` or `read` some bytes of the buffer first.
     */
    pub fn buffer_capacity(&self) -> usize {
        self.read_buffer.capacity()
    }

    /**
     * Shrinks the internal buffer to fit its length. If the new capacity is les then the desired, the capacity is extended to match the desired.
     * Already read bytes are not dropped. To do that call `consume`.
     * The actual capacity might be greater than the desired.
     */
    pub fn try_shrink_to(&mut self, desired_capacity: usize) -> usize {
        self.read_buffer.shrink_to_fit();
        if self.read_buffer.capacity() < desired_capacity {
            let reserve = desired_capacity - self.read_buffer.len();
            self.read_buffer.reserve_exact(reserve);
        }
        self.read_buffer.capacity()
    }
}

impl Read for SerialPi {
    /**
     * Fills buf with the bytes from the internal buffer. If buf.len() is greater then the buffered byte count, the serial device is read until buf is filled.
     */
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buffer_read_count = self.read_buffer.as_slice().read(buf)?;
        self.read_buffer.drain(0..buffer_read_count);
        if buffer_read_count < buf.len() {
            let (_, rest_buffer) = buf.split_at_mut(buffer_read_count);
            buffer_read_count += self.device.read(rest_buffer)?;
        }
        Ok(buffer_read_count)
    }
}

impl BufRead for SerialPi {
    /**
     * Does nothing if [`ComMode`] is not [`ComMode::FullDuplex`] and returns the internal buffer as slice.
     *
     * Tries to read the full buffer capacity if [`ComMode`] is [`ComMode::FullDuplex`]. returns all bytes that could be read.
     *
     * [`ComMode`]: ./enum.ComMode.html
     * [`ComMode::FullDuplex`]: ./enum.ComMode.html#variant.FullDuplex
     */
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.com_mode == ComMode::FullDuplex {
            let buffer_length = self.read_buffer.len();
            let bytes_read = {
                let rest_buffer = {
                    let capacity = self.read_buffer.capacity();
                    unsafe {
                        self.read_buffer.set_len(capacity);
                    }
                    let (_, rest_buffer) =
                        self.read_buffer.as_mut_slice().split_at_mut(buffer_length);
                    rest_buffer
                };
                self.device.read(rest_buffer)?
            };
            self.read_buffer.truncate(buffer_length + bytes_read);
        }
        Ok(self.read_buffer.as_slice())
    }
    /**
     * Does nothing if [`ComMode`] is not [`ComMode::FullDuplex`].
     *
     * Drains the first amt bytes from the internal buffer if [`ComMode`] is [`ComMode::FullDuplex`].
     *
     * [`ComMode`]: ./enum.ComMode.html
     * [`ComMode::FullDuplex`]: ./enum.ComMode.html#variant.FullDuplex
     */
    fn consume(&mut self, amt: usize) {
        if self.com_mode == ComMode::FullDuplex {
            self.read_buffer.drain(0..(amt));
        }
    }
}

impl Write for SerialPi {
    /**
     * Calls write in the spi device if [`ComMode`] is [`ComMode::HalfDuplex`].
     *
     * Does a full duplex transfer if [`ComMode`] is not [`ComMode::HalfDuplex`] and buffers the received bytes internally.
     * The buffer is allowed to grow above its capacity. Call `consume` to drop already received bytes or `read` them. To deallocate buffer space call `try_shrink_to` (after consuming already received data).
     *
     * [`ComMode`]: ./enum.ComMode.html
     * [`ComMode::HalfDuplex`]: ./enum.ComMode.html#variant.HalfDuplex
     */
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.com_mode == ComMode::HalfDuplex {
            self.device.write(buf)
        } else {
            let mut read_data: Vec<u8> = vec![0_u8; buf.len()];
            {
                let mut transfer = SpidevTransfer::read_write(buf, read_data.as_mut_slice());
                self.device.transfer(&mut transfer)?;
            }
            self.read_buffer.append(&mut read_data);
            Ok(buf.len())
        }
    }
    /**
     * Does nothing if [`ComMode`] is not [`ComMode::HalfDuplex`].
     *
     * Flushes the spi device if [`ComMode`] is [`ComMode::HalfDuplex`].
     *
     * [`ComMode`]: ./enum.ComMode.html
     * [`ComMode::HalfDuplex`]: ./enum.ComMode.html#variant.HalfDuplex
     */
    fn flush(&mut self) -> io::Result<()> {
        if self.com_mode == ComMode::HalfDuplex {
            self.device.flush()
        } else {
            Ok(())
        }
    }
}
