pub mod error;
pub mod message;

use bytes::{Buf, BufMut, BytesMut};
use error::CodecError;
pub use message::MagicMessage;
use tokio_util::codec::{Decoder, Encoder};

pub struct MagicCodec;

impl Encoder<MagicMessage> for MagicCodec {
    type Error = CodecError;

    fn encode(&mut self, item: MagicMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u64(item.magic);
        dst.put_u32(item.id);
        dst.put_u64(item.len);
        Ok(())
    }
}

impl Decoder for MagicCodec {
    type Item = MagicMessage;

    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        let magic = src.get_u64();
        let id = src.get_u32();
        let len = src.get_u64();
        Ok(Some(MagicMessage::new(magic, id, len)))
    }
}
