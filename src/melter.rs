use crate::{
    errors::LibError,
    tokens::{eu4_tokens_resolver, hoi4_tokens_resolver},
};
use eu4save::file::Eu4SliceFile;
use hoi4save::file::Hoi4SliceFile;
use std::io::Cursor;

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

impl Melter for &'_ Eu4SliceFile<'_> {
    fn melt(self) -> Result<MeltedBuffer, LibError> {
        if matches!(self.encoding(), eu4save::Encoding::Text) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let mut out = Cursor::new(Vec::new());
        let options = eu4save::MeltOptions::new()
            .verbatim(true)
            .on_failed_resolve(eu4save::FailedResolveStrategy::Stringify);
        let doc = self.melt(options, eu4_tokens_resolver(), &mut out)?;

        if self.encoding().is_text() {
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

impl Melter for &'_ Hoi4SliceFile<'_> {
    fn melt(self) -> Result<MeltedBuffer, LibError> {
        let mut out = Cursor::new(Vec::new());
        if matches!(self.encoding(), hoi4save::Encoding::Plaintext) {
            return Ok(MeltedBuffer::Verbatim);
        }

        let options = hoi4save::MeltOptions::new()
            .verbatim(true)
            .on_failed_resolve(hoi4save::FailedResolveStrategy::Stringify);
        let doc = self.melt(options, hoi4_tokens_resolver(), &mut out)?;

        Ok(MeltedBuffer::Binary {
            body: out.into_inner(),
            unknown_tokens: !doc.unknown_tokens().is_empty(),
        })
    }
}
