use palette::Srgb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Theme {
    #[serde(with = "color_hex")]
    pub primary_color: Srgb<u8>,
}

mod color_hex {
    use palette::Srgb;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(color: &Srgb<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex = format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue);
        serializer.serialize_str(&hex)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Srgb<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Srgb::from_str(&s).map_err(serde::de::Error::custom)
    }
}
