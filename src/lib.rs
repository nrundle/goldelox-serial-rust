use std::io::{Read, Write};

pub struct Goldelox<T> {
    port: T,
}

impl<T> Goldelox<T> 
    where T: Read + Write,
{
    pub fn new(port: T) -> Self {
        Self { port }
    }

    pub fn sys_GetVersion(&mut self) -> Result<u16, &'static str>{
        let w_buf: [u8;2] = [0,8];
        let bytes_written = self.port.write(&w_buf).expect("Failed to write");
        if bytes_written != 2 {
            return Err("Failed to write 2 bytes");
        }

        let mut buf: [u8;1] = [0;1];
        let bytes_read = self.port.read(&mut buf).expect("Failed to read");
        if bytes_read != 1 {
            return Err("Failed to receive 1 byte in Ack");
        }
        if buf[0] != 6 {
            return Err("Failed to receive Ack response");
        }

        let mut buf: [u8;2] = [0;2];
        let bytes_read = self.port.read(&mut buf).expect("Failed to get version");
        if bytes_read != 2 {
            return Err("Failed to get 2 bytes for version");
        }
        Ok((buf[0] as u16) << 8 | (buf[1] as u16))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serialport;
    use std::time::Duration;

    #[test]
    fn get_ver() {
        let mut port = serialport::new("/dev/ttyUSB0", 4_800)
            .timeout(Duration::from_millis(1000))
            .open().expect("Failed to open port");
        let mut goldelox = Goldelox::new(&mut port);
        let result = goldelox.sys_GetVersion();
        match result {
            Ok(version) => println!("Version: {}", version),
            Err(e) => println!("Error: {}", e)
        };
        assert_eq!(result.unwrap(), 259);
    }
}
