use serialport::SerialPort;

use crate::nes::NesInputMask;

pub fn read_from_serial(serial: &mut Box<dyn SerialPort>) -> anyhow::Result<NesInputMask> {
    let mut buf = [0u8; 1];
    serial.read(&mut buf)?;

    let Some(value) = buf.get(0) else {
        return Err(anyhow::format_err!("No bytes :("));
    };

    Ok(value.clone())
}