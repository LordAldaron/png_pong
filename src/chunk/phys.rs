// PNG Pong
//
// Copyright © 2019-2021 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{Chunk, DecoderError, EncoderError};
use crate::{consts, decoder::Parser, encoder::Enc};
use std::io::{Read, Write};

/// Physical dimensions chunk (pHYs)
#[derive(Copy, Clone, Debug)]
pub struct Physical {
    /// Pixels per unit: X dimension
    pub ppu_x: u32,
    /// Pixels per unit: Y dimension
    pub ppu_y: u32,
    /// Unit is `meter` if true, `unknown` if false.
    pub is_meter: bool,
}

impl Physical {
    pub(crate) fn write<W: Write>(
        &self,
        enc: &mut Enc<W>,
    ) -> Result<(), EncoderError> {
        enc.prepare(9, consts::PHYSICAL)?;
        enc.u32(self.ppu_x)?;
        enc.u32(self.ppu_y)?;
        enc.u8(if self.is_meter { 1 } else { 0 })?;
        enc.write_crc()
    }

    pub(crate) fn parse<R: Read>(
        parse: &mut Parser<R>,
    ) -> Result<Chunk, DecoderError> {
        // 9 bytes
        let ppu_x = parse.u32()?;
        let ppu_y = parse.u32()?;
        let is_meter = match parse.u8()? {
            0 => false,
            1 => true,
            _ => return Err(DecoderError::PhysUnits),
        };

        Ok(Chunk::Physical(Physical {
            ppu_x,
            ppu_y,
            is_meter,
        }))
    }
}
