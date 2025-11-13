use crate::prelude::*;

/// The message content
#[derive(Debug, Clone, From, Eq, PartialEq)]
#[from(String, "Content::Text { text: value.into() }")]
#[from(&str, "Content::Text { text: value.into() }")]
pub enum Content {
    Text { text: String },
    Image { image: Image },
}

impl Content {
    /// Adds response chunk to content
    pub(crate) fn add_chunk(&mut self, add_text: &str) {
        match self {
            Content::Text { text } => text.push_str(add_text),
            _ => {}
        }
    }
}

impl ::serde::Serialize for Content {
    fn serialize<S>(&self, se: S) -> StdResult<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        match self {
            Content::Text { text } => {
                let mut s = se.serialize_struct("Content", 2)?;
                s.serialize_field("type", "text")?;
                s.serialize_field("text", text)?;
                s.end()
            }
            Content::Image { image } => {
                let mut s = se.serialize_struct("Content", 2)?;
                s.serialize_field("type", "image_url")?;
                s.serialize_field("image_url", image)?;
                s.end()
            }
        }
    }
}

struct ContentVisitor;

impl<'de> ::serde::de::Visitor<'de> for ContentVisitor {
    type Value = Content;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("struct Content with type field")
    }

    fn visit_map<V>(self, mut map: V) -> StdResult<Self::Value, V::Error>
    where
        V: ::serde::de::MapAccess<'de>,
    {
        let mut ctype: Option<String> = None;
        let mut text: Option<String> = None;
        let mut image_url: Option<Image> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => {
                    if ctype.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"));
                    }
                    ctype = Some(map.next_value()?);
                }
                "text" => {
                    if text.is_some() {
                        return Err(serde::de::Error::duplicate_field("text"));
                    }
                    text = Some(map.next_value()?);
                }
                "image_url" => {
                    if image_url.is_some() {
                        return Err(serde::de::Error::duplicate_field("image_url"));
                    }
                    image_url = Some(map.next_value()?);
                }
                _ => {
                    let _ : serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        let ctype = ctype.ok_or_else(|| serde::de::Error::missing_field("type"))?;

        match ctype.as_str() {
            "text" => {
                let text = text.ok_or_else(|| serde::de::Error::missing_field("text"))?;
                Ok(Content::Text { text })
            }
            "image_url" => {
                let image_url = image_url.ok_or_else(|| serde::de::Error::missing_field("image_url"))?;
                Ok(Content::Image { image: image_url })
            }
            _ => Err(serde::de::Error::unknown_variant(&ctype, &["text", "image_url"])),
        }
    }
}

impl<'de> ::serde::Deserialize<'de> for Content {
    fn deserialize<D>(de: D) -> ::std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["type", "text", "image_url"];
        de.deserialize_struct("Content", FIELDS, ContentVisitor)
    }
}
