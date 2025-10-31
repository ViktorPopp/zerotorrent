use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Hashes(pub Vec<[u8; 20]>);
struct HashesVisitor;

impl<'de> Visitor<'de> for HashesVisitor {
    type Value = Hashes;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a byte string whose length is a multiple of 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!("Length is: {}", v.len())));
        }
        Ok(Hashes(
            v.chunks_exact(20)
                .map(|slice_20| {
                    slice_20
                        .try_into()
                        .expect("Failed to convert &[u8] to Hashes")
                })
                .collect(),
        ))
    }
}

impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashesVisitor)
    }
}

impl Serialize for Hashes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let single_slice = self.0.concat();
        serializer.serialize_bytes(&single_slice)
    }
}
