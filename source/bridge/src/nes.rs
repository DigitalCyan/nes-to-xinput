
pub mod buttons {
    pub type Button = u8;

    #[allow(dead_code)]
    pub const A: u8 = 1 << 7;

    #[allow(dead_code)]
    pub const B: u8 = 1 << 6;

    #[allow(dead_code)]
    pub const SELECT: u8 = 1 << 5;

    #[allow(dead_code)]
    pub const START: u8 = 1 << 4;

    #[allow(dead_code)]
    pub const UP: u8 = 1 << 3;

    #[allow(dead_code)]
    pub const DOWN: u8 = 1 << 2;

    #[allow(dead_code)]
    pub const LEFT: u8 = 1 << 1;

    #[allow(dead_code)]
    pub const RIGHT: u8 = 1 << 0;
}

pub type NesInputMask = u8;

use buttons::Button;
pub fn is_down(input_mask: NesInputMask, button: Button) -> bool {
    input_mask & button != 0
}
