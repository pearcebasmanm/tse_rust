use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser::SerializeStruct};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Block {
    #[serde(rename = "code")] // For deserialization
    pub shape: Shape,
    pub color: Color,
    pub count: u32,
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (code, dimensions) = match self.shape {
            Shape::OneByTwo => ("1021", "1x1"),
            Shape::TwoByTwo => ("2021", "2x2"),
            Shape::TwoByFour => ("2041", "2x4"),
            Shape::TwoByFourC => ("2042", "2x4c"),
            Shape::Head => ("HEAD", "Head"),
        };

        let mut block = serializer.serialize_struct("Block", 4)?;
        block.serialize_field("code", code)?;
        block.serialize_field("shape", dimensions)?;
        block.serialize_field("color", &self.color)?;
        block.serialize_field("count", &self.count)?;
        block.end()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Shape {
    OneByTwo,
    TwoByTwo,
    TwoByFour,
    TwoByFourC,
    Head,
}

impl<'de> Deserialize<'de> for Shape {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let shape = match s {
            "1021" => Shape::OneByTwo,
            "2021" => Shape::TwoByTwo,
            "2041" => Shape::TwoByFour,
            "2042" => Shape::TwoByFourC,
            "HEAD" => Shape::Head,
            _ => return Err(de::Error::custom(format!("Invalid Shape Code `{s}`"))), // Handle unknown status
        };
        Ok(shape)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Color {
    Black,
    Yellow,
    Red,
    White,
}
