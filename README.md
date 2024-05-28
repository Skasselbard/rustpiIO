# rustpiIO
RustpiIO is a library to read and write to the SPI bus and the GPIO pins of a Raspberry Pi.

It uses the system interface under /sys/class/gpio/ provided by the linux OS for the gpios. And wraps [spidev](https://crates.io/crates/spidev) for the serial interface.

There is also an interface to read out the [revision codes](https://www.raspberrypi.org/documentation/hardware/raspberrypi/revision-codes/README.md) in /proc/cpuinfo for programmatic use.

# Documentation
You can find the documentation [here](https://skasselbard.github.io/rustpiIO/).

# Installation
To compile a raspberry pi program you need to prepare a cross compiler for rust
(for the older pi processors try `*gnueabi` instead of `*gnueabihf`):  
`rustup target add arm-unknown-linux-gnueabihf`  
`sudo apt-get install gcc-arm-linux-gnueabihf`  
To tell the linker which program to use add the following lines to a corresponding
./cargo/config file (like in this project)  
`[target.arm-unknown-linux-gnueabihf]`  
`linker="arm-linux-gnueabihf-gcc"`  
Build for the Raspberry with `cargo build --target=arm-unknown-linux-gnueabihf`  


# Example  

```
extern crate rustpi_io;
use rustpi_io::gpio::{GPIOData, GPIOMode, GPIO};

fn main() {
    let gpio2 = GPIO::new(2, GPIOMode::Write).unwrap();
    let gpio3 = GPIO::new(3, GPIOMode::Read).unwrap();
    let mut value: u8 = 1;
    for _ in 1..100 {
        value = 1 - value;
        let data = match value {
            0 => GPIOData::Low,
            1 => GPIOData::High,
            _ => GPIOData::High,
        };
        gpio2.set(data).unwrap();
        let data = gpio3.value().unwrap();
        println!("value: {}", data);
    }
}
```
