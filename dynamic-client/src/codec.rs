use bytes::{Buf, BufMut};
use prost::Message;
use prost_reflect::{DynamicMessage, MessageDescriptor};
use tonic::{
    Status,
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
};

#[derive(Clone, Debug)]
pub struct DynamicCodec {
    req_desc: MessageDescriptor,
    res_desc: MessageDescriptor,
}

impl DynamicCodec {
    pub fn new(req_desc: MessageDescriptor, res_desc: MessageDescriptor) -> Self {
        Self { req_desc, res_desc }
    }
}

impl Codec for DynamicCodec {
    type Encode = DynamicMessage;
    type Decode = DynamicMessage;
    type Encoder = DynamicEncoder;
    type Decoder = DynamicDecoder;

    fn encoder(&mut self) -> Self::Encoder {
        DynamicEncoder
    }

    fn decoder(&mut self) -> Self::Decoder {
        DynamicDecoder {
            desc: self.res_desc.clone(),
        }
    }
}

pub struct DynamicEncoder;

impl Encoder for DynamicEncoder {
    type Item = DynamicMessage;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        // Reserve exact capacity to prevent reallocation
        buf.reserve(item.encoded_len());
        item.encode(buf)
            .map_err(|e| Status::internal(format!("Failed to encode dynamic message: {}", e)))?;
        Ok(())
    }
}

pub struct DynamicDecoder {
    desc: MessageDescriptor,
}

impl Decoder for DynamicDecoder {
    type Item = DynamicMessage;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        if !buf.has_remaining() {
            return Ok(None);
        }

        let mut msg = DynamicMessage::new(self.desc.clone());
        // Merge consumes the buffer. Tonic's framing ensures this is exactly one message.
        msg.merge(buf)
            .map_err(|e| Status::internal(format!("Failed to decode dynamic message: {}", e)))?;

        Ok(Some(msg))
    }
}
