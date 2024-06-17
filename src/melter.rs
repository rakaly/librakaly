use std::io::Cursor;

use crate::errors::LibError;
use ck3save::Ck3Melter;
use eu4save::Eu4Melter;
use hoi4save::Hoi4Melter;
use imperator_save::ImperatorMelter;
use vic3save::Vic3Melter;

pub enum MeltedBufferResult {
    Ok(MeltedBuffer),
    Err(LibError),
}

/// An opaque struct that holds the results of the melting operatation
pub enum MeltedBuffer {
    Verbatim,
    Text { header: Vec<u8>, body: Vec<u8> },
    Binary { body: Vec<u8>, unknown_tokens: bool },
}

impl MeltedBuffer {
    pub fn len(&self) -> usize {
        match self {
            MeltedBuffer::Verbatim => 0,
            MeltedBuffer::Text { header, body } => header.len() + body.len(),
            MeltedBuffer::Binary { body, .. } => body.len(),
        }
    }
}

pub trait Melter {
    fn melt(self) -> Result<MeltedBuffer, LibError>;
}

impl<'a> Melter for Eu4Melter<'a> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), eu4save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(eu4save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &eu4save::EnvTokens)?;

        if self.input_encoding().is_text() {
            Ok(MeltedBuffer::Text {
                header: Vec::new(),
                body: out.into_inner(),
            })
        } else {
            Ok(MeltedBuffer::Binary {
                body: out.into_inner(),
                unknown_tokens: !doc.unknown_tokens().is_empty(),
            })
        }
    }
}

impl<'a> Melter for Ck3Melter<'a> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), ck3save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(ck3save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &ck3save::EnvTokens)?;

        if matches!(self.input_encoding(), ck3save::Encoding::TextZip) {
            Ok(MeltedBuffer::Text {
                header: Vec::new(),
                body: out.into_inner(),
            })
        } else {
            Ok(MeltedBuffer::Binary {
                body: out.into_inner(),
                unknown_tokens: !doc.unknown_tokens().is_empty(),
            })
        }
    }
}

impl<'a> Melter for ImperatorMelter<'a> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), imperator_save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(imperator_save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &imperator_save::EnvTokens)?;

        if matches!(self.input_encoding(), imperator_save::Encoding::TextZip) {
            Ok(MeltedBuffer::Text {
                header: Vec::new(),
                body: out.into_inner(),
            })
        } else {
            Ok(MeltedBuffer::Binary {
                body: out.into_inner(),
                unknown_tokens: !doc.unknown_tokens().is_empty(),
            })
        }
    }
}

impl<'a> Melter for Hoi4Melter<'a> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), hoi4save::Encoding::Plaintext) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(hoi4save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &hoi4save::EnvTokens)?;

        Ok(MeltedBuffer::Binary {
            body: out.into_inner(),
            unknown_tokens: !doc.unknown_tokens().is_empty(),
        })
    }
}

impl<'a> Melter for Vic3Melter<'a> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), vic3save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(vic3save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &vic3save::EnvTokens)?;

        if matches!(self.input_encoding(), vic3save::Encoding::TextZip) {
            Ok(MeltedBuffer::Text {
                header: Vec::new(),
                body: out.into_inner(),
            })
        } else {
            Ok(MeltedBuffer::Binary {
                body: out.into_inner(),
                unknown_tokens: !doc.unknown_tokens().is_empty(),
            })
        }
    }
}
