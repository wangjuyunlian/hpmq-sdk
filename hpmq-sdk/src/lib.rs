pub use anyhow;
pub use ntex_bytes;
pub use serde;
use serde::{Deserialize, Serialize};
pub use serde_json;

#[cfg(feature = "sdk")]
pub use hpmq_sdk_macro::*;
#[cfg(feature = "sdk")]
pub mod wapc_guest {
    pub use wapc_guest::*;
}
pub use ty::*;

#[derive(Deserialize, Serialize, Debug)]
pub enum HandResult {
    Discard,
    Transmit((Vec<u8>, Vec<u8>)),
}

mod ty {
    use anyhow::Result;
    use ntex_bytes::{ByteString, Bytes};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /// 转换前的订阅topic
    #[derive(Deserialize, Serialize, Debug)]
    pub struct TopicDatas {
        pub topic: String,
        pub context: HashMap<String, String>,
    }

    impl TopicDatas {
        pub fn new(topic: &ByteString, context: &HashMap<ByteString, ByteString>) -> Self {
            Self {
                topic: topic.to_string(),
                context: hash_tran(context.clone()),
            }
        }
        pub fn to_vec(&self) -> Result<Vec<u8>> {
            Ok(serde_json::to_vec(&self)?)
        }
    }

    /// 转换前的发布数据
    #[derive(Deserialize, Serialize, Debug)]
    pub struct PublishDatas {
        pub topic: String,
        pub payload: Bytes,
        pub context: HashMap<String, String>,
        pub his_payload: Option<Bytes>,
    }
    impl PublishDatas {
        pub fn new(
            topic: ByteString,
            payload: Bytes,
            context: HashMap<ByteString, ByteString>,
            his_payload: Option<Bytes>,
        ) -> Self {
            Self {
                topic: topic.to_string(),
                payload,
                context: hash_tran(context),
                his_payload,
            }
        }
        pub fn to_vec(&self) -> Result<Vec<u8>> {
            Ok(serde_json::to_vec(self)?)
        }
    }

    /// 转换后的发布数据
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResPublish {
        pub topic: Vec<u8>,
        pub payload: Vec<u8>,
    }
    impl ResPublish {
        pub fn new(topic: Vec<u8>, payload: Vec<u8>) -> Self {
            Self { topic, payload }
        }
        pub fn from_vec(data: Vec<u8>) -> Result<Self> {
            Ok(serde_json::from_slice(data.as_slice())?)
        }
    }

    fn hash_tran(src: HashMap<ByteString, ByteString>) -> HashMap<String, String> {
        let mut dis: HashMap<String, String> = HashMap::with_capacity(src.len());

        src.into_iter().for_each(|(key, val)| {
            dis.insert(key.to_string(), val.to_string());
        });
        dis
    }
}
impl From<(Vec<u8>, Vec<u8>)> for HandResult {
    fn from(val: (Vec<u8>, Vec<u8>)) -> Self {
        Self::Transmit(val)
    }
}
