use crate::nes::{buttons::*, NesInputMask};
use vigem_client::XButtons;

pub fn convert_to_xinput(nes_mask: NesInputMask) -> u16 {
    let mut x_mask = 0u16;

    if nes_mask & A      != 0 { x_mask |= XButtons::A     };
    if nes_mask & B      != 0 { x_mask |= XButtons::B     };
    if nes_mask & SELECT != 0 { x_mask |= XButtons::BACK  };
    if nes_mask & START  != 0 { x_mask |= XButtons::START };
    if nes_mask & UP     != 0 { x_mask |= XButtons::UP    };
    if nes_mask & DOWN   != 0 { x_mask |= XButtons::DOWN  };
    if nes_mask & LEFT   != 0 { x_mask |= XButtons::LEFT  };
    if nes_mask & RIGHT  != 0 { x_mask |= XButtons::RIGHT };

    return x_mask;
}
