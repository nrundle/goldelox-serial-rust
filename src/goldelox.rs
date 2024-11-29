use embedded_io::{Read, Write};
use crate::constants::cmd;

pub struct Goldelox<T> {
    port: T,
}

#[allow(non_snake_case)]
impl<T> Goldelox<T> 
    where T: Read + Write,
{
    pub fn new(port: T) -> Self {
        Self { port }
    }

    pub fn sys_GetVersion(&mut self) -> Result<u16, &'static str> {
        let _bytes_written = self.write_word(cmd::F_SYS_GET_VERSION).expect("Failed to write");

        match self.get_ack_resp() {
            Ok(version) => Ok(version),
            Err(()) => Err("Failed to get version"),
        }
    }

    fn get_ack_resp(&mut self) -> Result<u16, ()> {
        self.get_ack()?;
        self.get_word()
    }

    fn get_ack(&mut self) -> Result<(), ()> {
        let mut buf: [u8;1] = [0;1];
        let bytes_read = self.port.read(&mut buf).expect("Failed to read ack");
        if bytes_read != 1 {
            return Err(());
        }
        if buf[0] != 6 {
            return Err(());
        }
        Ok(())
    }

    fn get_word(&mut self) -> Result<u16, ()> {
        let mut buf: [u8;2] = [0;2];
        let bytes_read = self.port.read(&mut buf).expect("Failed to get version");
        if bytes_read != 2 {
            return Err(());
        }
        Ok((buf[0] as u16) << 8 | (buf[1] as u16))
    }

    fn write_word(&mut self, word: i16) -> Result<usize, &'static str> {
        let bytes = word.to_be_bytes();
        let bytes_written = self.port.write(&bytes).expect("Failed to write");
        if bytes_written != 2 {
            return Err("Failed to write word");
        }
        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serialport::*;
    use std::time::Duration;
    use embedded_io_adapters::std::FromStd;

    #[test]
    fn get_ver() {
        let port= serialport::new("/dev/ttyUSB0", 4_800)
            .timeout(Duration::from_millis(1000))
            .open().expect("Failed to open port");
        let mut em_port: FromStd<Box<dyn SerialPort>> = FromStd::new(port);
        let mut goldelox = Goldelox::new(&mut em_port);
        let result = goldelox.sys_GetVersion();
        match result {
            Ok(version) => println!("Version: {}", version),
            Err(e) => println!("Error: {}", e)
        };
        assert_eq!(result.unwrap(), 259);
    }
}
