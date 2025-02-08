use crate::{
    errors::LibError,
    tokens::{
        ck3_tokens_resolver, eu4_tokens_resolver, hoi4_tokens_resolver, imperator_tokens_resolver,
        vic3_tokens_resolver,
    },
};
use ck3save::Ck3Melter;
use eu4save::Eu4Melter;
use hoi4save::Hoi4Melter;
use imperator_save::ImperatorMelter;
use std::io::Cursor;
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

impl Melter for Eu4Melter<'_> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), eu4save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(eu4save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, eu4_tokens_resolver())?;

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

impl Melter for Ck3Melter<'_> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), ck3save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(ck3save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &ck3_tokens_resolver())?;

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

impl Melter for ImperatorMelter<'_> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), imperator_save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(imperator_save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &imperator_tokens_resolver())?;

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

impl Melter for Hoi4Melter<'_> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), hoi4save::Encoding::Plaintext) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(hoi4save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &hoi4_tokens_resolver())?;

        Ok(MeltedBuffer::Binary {
            body: out.into_inner(),
            unknown_tokens: !doc.unknown_tokens().is_empty(),
        })
    }
}

impl Melter for Vic3Melter<'_> {
    fn melt(mut self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.input_encoding(), vic3save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let doc = self
            .on_failed_resolve(vic3save::FailedResolveStrategy::Stringify)
            .verbatim(true)
            .melt(&mut out, &vic3_tokens_resolver())?;

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
