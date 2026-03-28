uniffi::setup_scaffolding!();
use core_logic::generate_color;

#[derive(uniffi::Record)]
pub struct AndroidColorResult {
    pub result: u32,
}

#[uniffi::export]
pub fn generate_color_android() -> AndroidColorResult {
    let payload = generate_color();

    AndroidColorResult {
        result: payload.result,
    }
}
