
pub mod rustpi_io{
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::Result;
    use std::io::ErrorKind;
    use std::io::Error;

    static GPIO_PATH: &'static str = "/sys/class/gpio/";

    #[derive(Debug, Clone, Copy)]
    pub enum GPIOMode{
        Read,
        Write
    }

    pub struct GPIO {
        pin: u8,
        mode: GPIOMode
    }

    impl GPIO{
        pub fn gpio_number(&self) -> u8 { self.pin }

        pub fn current_mode(&self) -> GPIOMode {
            self.mode
        }

        pub fn set_mode(&mut self, mode: GPIOMode) -> Result<&mut Self>{
            let mut direction = OpenOptions::new().write(true).open(format!("{}gpio{}/direction", GPIO_PATH, self.pin))?;
            match mode{
                GPIOMode::Read   => try!(direction.write_all("in".as_bytes())),
                GPIOMode::Write   => try!(direction.write_all("out".as_bytes())),
            };
            self.mode = mode;
            Ok(self)
        }

        pub fn init(gpio: u8, mode: GPIOMode) -> Result<Self> {
            println!("gpio number = {:?}", gpio);
            println!("mode = {:?}", mode);
            {
                let mut export_file = OpenOptions::new().write(true).open(format!("{}export", GPIO_PATH))?;
                export_file.write_all(format!("{}", gpio).as_bytes())?;
            }
            let mut result = GPIO{pin: gpio, mode: mode};
            result.set_mode(mode)?;
            Ok(result)
        }
    }

    impl Drop for GPIO{
        fn drop(&mut self){
            let mut unexport = match OpenOptions::new().write(true).open(format!("{}unexport", GPIO_PATH)) {
                Ok(f) => f,
                Err(e) => panic!("file error: {}", e),
            };
            match unexport.write_all(format!("{}", self.pin).as_bytes()) {
                Err(why) => {
                   panic!("couldn't close gpio {}: {}",self.pin , why)
                },
                Ok(_) => println!("gpio {} closed", self.pin),
            }
        }
    }

    impl Read for GPIO {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize>{
            let mut value = try!(OpenOptions::new().read(true).open(format!("{}gpio{}/value", GPIO_PATH, self.pin)));
            let mut buffer = vec![];
            try!(value.read_to_end(&mut buffer ));
            buf[0] = buffer[0];
            Ok(buffer.len())
        }
    }

    impl Write for GPIO {
        fn write(&mut self, buf: &[u8]) -> Result<usize>{
            if buf[0] > 1{
                return Err(Error::new(ErrorKind::InvalidData, "trying to write value greater then one, but GPIOs can only be High (one) or Low (0)"))
            }
            let mut direction = OpenOptions::new().write(true).open(format!("{}gpio{}/direction", GPIO_PATH, self.pin))?;
            try!(direction.write_all(buf));
            Ok(5)
        }
        fn flush(&mut self) -> Result<()>{
            Ok(())
        }
    }
}
