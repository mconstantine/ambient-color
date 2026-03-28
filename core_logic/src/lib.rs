use serde::Serialize;

#[derive(Serialize)]
pub struct ColorResult {
    pub result: u32,
}

pub fn generate_color() -> ColorResult {
    ColorResult { result: 42 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = generate_color();
        assert_eq!(result.result, 42);
    }
}
