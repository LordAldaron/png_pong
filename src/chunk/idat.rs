// PNG Pong
//
// Copyright © 2019-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::{Read, Write};

use crate::{
    checksum::CrcDecoder, consts, decode::Result as DecoderResult,
    encode::Error as EncoderError, zlib,
};

/// Image Data Chunk Data (IDAT)
#[derive(Debug)]
pub struct ImageData {
    /// Part of a compressed ZLIB stream
    pub data: Vec<u8>,
}

impl ImageData {
    pub(crate) fn read<R: Read>(reader: &mut R) -> DecoderResult<(Self, u32)> {
        let mut chunk = CrcDecoder::new(reader, consts::IMAGE_DATA);
        let data = chunk.vec_eof()?;
        Ok((ImageData { data }, chunk.end()?))
    }

    pub(crate) fn write<W: Write>(
        &self,
        writer: &mut W,
        level: u8,
    ) -> Result<(), EncoderError> {
        let mut zlib = Vec::new();
        // FIXME: Should already be compressed.
        zlib::compress(&mut zlib, self.data.as_slice(), level);
        super::encode_chunk(writer, consts::IMAGE_DATA, &zlib)?;
        Ok(())
    }

    /// Construct from raw uncompressed image data.
    pub fn with_data(data: Vec<u8>) -> ImageData {
        ImageData { data }
    }

    /// Get the image data
    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }
}
