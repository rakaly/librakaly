use std::io::Cursor;

use crate::{
    errors::LibError,
    melter::Melter,
    tokens::{
        ck3_tokens_resolver, eu4_tokens_resolver, imperator_tokens_resolver, vic3_tokens_resolver,
    },
    MeltedBuffer,
};
use ck3save::file::Ck3SliceFile;
use eu4save::file::{Eu4SliceFile, Eu4Zip};
use hoi4save::file::Hoi4SliceFile;
use imperator_save::file::{ImperatorSliceFile, ImperatorSliceFileKind};
use vic3save::file::{Vic3SliceFile, Vic3SliceFileKind};

pub enum PdsFileResult<'a> {
    Ok(PdsFile<'a>),
    Err(LibError),
}

pub enum PdsFile<'a> {
    Eu4(Eu4SliceFile<'a>),
    Ck3(Ck3SliceFile<'a>),
    Imperator(ImperatorSliceFile<'a>),
    Hoi4(Hoi4SliceFile<'a>),
    Vic3(Vic3SliceFile<'a>),
}

impl PdsFile<'_> {
    pub(crate) fn meta(&self) -> Option<PdsMeta> {
        match self {
            PdsFile::Eu4(file) => {
                let eu4save::file::Eu4SliceFileKind::Zip(zip) = file.kind() else {
                    return None;
                };

                Some(PdsMeta::Eu4(zip.clone()))
            }
            PdsFile::Ck3(file) => Some(PdsMeta::Ck3(file.clone())),
            PdsFile::Imperator(file) => Some(PdsMeta::Imperator(file.clone())),
            PdsFile::Hoi4(_) => None,
            PdsFile::Vic3(file) => Some(PdsMeta::Vic3(file.clone())),
        }
    }

    pub(crate) fn melt_file(&self) -> Result<MeltedBuffer, LibError> {
        match self {
            PdsFile::Eu4(file) => Melter::melt(file),
            PdsFile::Ck3(file) => Melter::melt(file),
            PdsFile::Imperator(file) => Melter::melt(file),
            PdsFile::Hoi4(file) => Melter::melt(file),
            PdsFile::Vic3(file) => Melter::melt(file),
        }
    }

    pub(crate) fn is_binary(&self) -> bool {
        match self {
            PdsFile::Eu4(file) => file.encoding().is_binary(),
            PdsFile::Ck3(file) => matches!(
                file.encoding(),
                ck3save::Encoding::Binary | ck3save::Encoding::BinaryZip
            ),
            PdsFile::Imperator(file) => matches!(
                file.encoding(),
                imperator_save::Encoding::Binary | imperator_save::Encoding::BinaryZip
            ),
            PdsFile::Hoi4(file) => matches!(file.encoding(), hoi4save::Encoding::Binary),
            PdsFile::Vic3(file) => matches!(
                file.encoding(),
                vic3save::Encoding::Binary | vic3save::Encoding::BinaryZip
            ),
        }
    }
}

pub enum PdsMeta<'data> {
    Eu4(Box<Eu4Zip<&'data [u8]>>),
    Ck3(Ck3SliceFile<'data>),
    Imperator(ImperatorSliceFile<'data>),
    Vic3(Vic3SliceFile<'data>),
}

impl PdsMeta<'_> {
    pub(crate) fn melt(&self) -> Result<MeltedBuffer, LibError> {
        match self {
            PdsMeta::Eu4(entry) => {
                let options = eu4save::MeltOptions::new()
                    .verbatim(true)
                    .on_failed_resolve(eu4save::FailedResolveStrategy::Stringify);
                let resolver = eu4_tokens_resolver();
                let mut output = Cursor::new(Vec::new());
                let doc = entry.melt(options, resolver, &mut output)?;
                if entry.encoding().is_text() {
                    Ok(MeltedBuffer::Text {
                        header: Vec::new(),
                        body: output.into_inner(),
                    })
                } else {
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
            }
            PdsMeta::Ck3(file) => match file.kind() {
                ck3save::file::Ck3SliceFileKind::Text(_) => Ok(MeltedBuffer::Verbatim),
                ck3save::file::Ck3SliceFileKind::Binary(binary) => {
                    let options = ck3save::MeltOptions::new().verbatim(true);
                    let mut output = Cursor::new(Vec::new());
                    let doc = binary
                        .clone()
                        .melt(options, ck3_tokens_resolver(), &mut output)?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                ck3save::file::Ck3SliceFileKind::Zip(zip) => {
                    let options = ck3save::MeltOptions::new().verbatim(true);
                    let mut output = Cursor::new(Vec::new());
                    let doc = zip
                        .meta()?
                        .melt(options, ck3_tokens_resolver(), &mut output)?;
                    if matches!(file.encoding(), ck3save::Encoding::TextZip) {
                        Ok(MeltedBuffer::Text {
                            header: Vec::new(),
                            body: output.into_inner(),
                        })
                    } else {
                        Ok(MeltedBuffer::Binary {
                            body: output.into_inner(),
                            unknown_tokens: !doc.unknown_tokens().is_empty(),
                        })
                    }
                }
            },
            PdsMeta::Imperator(file) => match file.kind() {
                ImperatorSliceFileKind::Text(_) => Ok(MeltedBuffer::Verbatim),
                ImperatorSliceFileKind::Binary(binary) => {
                    let options = imperator_save::MeltOptions::new().verbatim(true);
                    let mut output = Cursor::new(Vec::new());
                    let doc =
                        binary
                            .clone()
                            .melt(options, imperator_tokens_resolver(), &mut output)?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                ImperatorSliceFileKind::Zip(zip) => {
                    let options = imperator_save::MeltOptions::new().verbatim(true);
                    let mut output = Cursor::new(Vec::new());
                    let doc =
                        zip.meta()?
                            .melt(options, imperator_tokens_resolver(), &mut output)?;
                    if matches!(file.encoding(), imperator_save::Encoding::TextZip) {
                        Ok(MeltedBuffer::Text {
                            header: Vec::new(),
                            body: output.into_inner(),
                        })
                    } else {
                        Ok(MeltedBuffer::Binary {
                            body: output.into_inner(),
                            unknown_tokens: !doc.unknown_tokens().is_empty(),
                        })
                    }
                }
            },
            PdsMeta::Vic3(file) => match file.kind() {
                Vic3SliceFileKind::Text(_) => Ok(MeltedBuffer::Verbatim),
                Vic3SliceFileKind::Binary(binary) => {
                    let options = vic3save::MeltOptions::new().verbatim(true);
                    let mut output = Cursor::new(Vec::new());
                    let doc = binary
                        .clone()
                        .melt(options, vic3_tokens_resolver(), &mut output)?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                Vic3SliceFileKind::Zip(zip) => {
                    let options = vic3save::MeltOptions::new().verbatim(true);
                    let mut output = Cursor::new(Vec::new());
                    let doc = zip
                        .meta()?
                        .melt(options, vic3_tokens_resolver(), &mut output)?;
                    if matches!(file.encoding(), vic3save::Encoding::TextZip) {
                        Ok(MeltedBuffer::Text {
                            header: Vec::new(),
                            body: output.into_inner(),
                        })
                    } else {
                        Ok(MeltedBuffer::Binary {
                            body: output.into_inner(),
                            unknown_tokens: !doc.unknown_tokens().is_empty(),
                        })
                    }
                }
            },
        }
    }
}
