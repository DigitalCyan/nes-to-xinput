mod nes;
mod nes_to_x;
mod serial;

use std::{time::{Instant, Duration}, thread, env};

use serial::read_from_serial;
use vigem_client::TargetId;

use crate::nes_to_x::convert_to_xinput;

fn main() -> anyhow::Result<()> {
    let client = vigem_client::Client::connect().unwrap();

    let serial_path = env::args().nth(1).expect("No serial path provided");

    let mut controller = vigem_client::Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
    let mut serial = serialport::new(serial_path, 9600).open()?;

    controller.plugin().unwrap();
    controller.wait_ready().unwrap();

    loop {
        let nes_mask = read_from_serial(&mut serial)?;
        let x_mask = convert_to_xinput(nes_mask);

        let state = vigem_client::XGamepad{
            buttons: vigem_client::XButtons(x_mask),
            ..Default::default()
        };

        controller.update(&state)?;
    }
}
