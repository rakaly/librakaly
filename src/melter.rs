use std::io::Cursor;

use crate::errors::LibError;
use ck3save::file::Ck3Binary;
use eu4save::Eu4Melter;
use hoi4save::file::Hoi4Binary;
use imperator_save::ImperatorMelter;
use vic3save::file::Vic3Binary;

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

impl<'a> Melter for &'_ Ck3Binary<'a> {
    fn melt(self) -> Result<MeltedBuffer, LibError> {
        let melted = self
            .melter()
            .on_failed_resolve(ck3save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&ck3save::EnvTokens)?;

        Ok(MeltedBuffer::Binary {
            unknown_tokens: !melted.unknown_tokens().is_empty(),
            body: melted.into_data(),
        })
    }
}

impl<'a> Melter for ImperatorMelter<'a> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        let doc = self
            .on_failed_resolve(eu4save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &eu4save::EnvTokens)?;

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

impl<'a> Melter for &'_ Hoi4Binary<'a> {
    fn melt(self) -> Result<MeltedBuffer, LibError> {
        let melted = self
            .melter()
            .on_failed_resolve(hoi4save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&hoi4save::EnvTokens)?;

        Ok(MeltedBuffer::Binary {
            unknown_tokens: !melted.unknown_tokens().is_empty(),
            body: melted.into_data(),
        })
    }
}

impl<'a> Melter for &'_ Vic3Binary<'a> {
    fn melt(self) -> Result<MeltedBuffer, LibError> {
        let melted = self
            .melter()
            .on_failed_resolve(vic3save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&vic3save::EnvTokens)?;

        Ok(MeltedBuffer::Binary {
            unknown_tokens: !melted.unknown_tokens().is_empty(),
            body: melted.into_data(),
        })
    }
}
