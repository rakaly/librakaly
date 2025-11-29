use std::io::Cursor;

use crate::{
    errors::LibError,
    melter::Melter,
    tokens::{
        ck3_tokens_resolver, eu4_tokens_resolver, eu5_tokens_resolver, imperator_tokens_resolver,
        vic3_tokens_resolver,
    },
    MeltedBuffer,
};
use eu4save::file::{Eu4SliceFile, Eu4Zip};
use eu5save::{JominiFileKind, SaveDataKind};
use hoi4save::file::Hoi4SliceFile;

pub enum PdsFileResult<'a> {
    Ok(PdsFile<'a>),
    Err(LibError),
}

pub enum PdsFile<'a> {
    Eu4(Eu4SliceFile<'a>),
    Ck3(jomini::envelope::JominiFile<Cursor<&'a [u8]>>),
    Imperator(jomini::envelope::JominiFile<Cursor<&'a [u8]>>),
    Hoi4(Hoi4SliceFile<'a>),
    Vic3(jomini::envelope::JominiFile<Cursor<&'a [u8]>>),
    Eu5(jomini::envelope::JominiFile<Cursor<&'a [u8]>>),
}

impl PdsFile<'_> {
    pub(crate) fn meta(&self) -> Option<PdsMeta<'_>> {
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
            PdsFile::Eu5(file) => Some(PdsMeta::Eu5(file.clone())),
        }
    }

    pub(crate) fn melt_file(&self) -> Result<MeltedBuffer, LibError> {
        match self {
            PdsFile::Eu4(file) => Melter::melt(file),
            PdsFile::Hoi4(file) => Melter::melt(file),

            PdsFile::Ck3(file) => match file.kind() {
                JominiFileKind::Uncompressed(SaveDataKind::Text(_)) => Ok(MeltedBuffer::Verbatim),
                JominiFileKind::Uncompressed(SaveDataKind::Binary(binary)) => {
                    let options = ck3save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(ck3save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = ck3save::Ck3Melt::melt(
                        &mut &*binary,
                        options,
                        ck3_tokens_resolver(),
                        &mut output,
                    )?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                JominiFileKind::Zip(zip) => {
                    let options = ck3save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(ck3save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = ck3save::Ck3Melt::melt(
                        &mut &*zip,
                        options,
                        ck3_tokens_resolver(),
                        &mut output,
                    )?;
                    if file.header().kind().is_text() {
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
            PdsFile::Imperator(file) => match file.kind() {
                JominiFileKind::Uncompressed(SaveDataKind::Text(_)) => Ok(MeltedBuffer::Verbatim),
                JominiFileKind::Uncompressed(SaveDataKind::Binary(binary)) => {
                    let options = imperator_save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(imperator_save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = imperator_save::ImperatorMelt::melt(
                        &mut &*binary,
                        options,
                        imperator_tokens_resolver(),
                        &mut output,
                    )?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                JominiFileKind::Zip(zip) => {
                    let options = imperator_save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(imperator_save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = imperator_save::ImperatorMelt::melt(
                        &mut &*zip,
                        options,
                        imperator_tokens_resolver(),
                        &mut output,
                    )?;
                    if file.header().kind().is_text() {
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

            PdsFile::Vic3(file) => match file.kind() {
                JominiFileKind::Uncompressed(SaveDataKind::Text(_)) => Ok(MeltedBuffer::Verbatim),
                JominiFileKind::Uncompressed(SaveDataKind::Binary(binary)) => {
                    let options = vic3save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(vic3save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = vic3save::Vic3Melt::melt(
                        &mut &*binary,
                        options,
                        vic3_tokens_resolver(),
                        &mut output,
                    )?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                JominiFileKind::Zip(zip) => {
                    let options = vic3save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(vic3save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = vic3save::Vic3Melt::melt(
                        &mut &*zip,
                        options,
                        vic3_tokens_resolver(),
                        &mut output,
                    )?;
                    if file.header().kind().is_text() {
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

            PdsFile::Eu5(file) => match file.kind() {
                JominiFileKind::Uncompressed(SaveDataKind::Text(_)) => Ok(MeltedBuffer::Verbatim),
                JominiFileKind::Uncompressed(SaveDataKind::Binary(_)) => Err(
                    LibError::UnsupportedOperation(String::from("melting uncompressed eu5 binary")),
                ),
                JominiFileKind::Zip(zip) => {
                    let options = eu5save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(eu5save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let resolver = eu5save::SaveResolver::create(zip, eu5_tokens_resolver())?;
                    let doc = eu5save::Eu5Melt::melt(&mut &*zip, options, resolver, &mut output)?;
                    if file.header().kind().is_text() {
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

    pub(crate) fn is_binary(&self) -> bool {
        match self {
            PdsFile::Eu4(file) => file.encoding().is_binary(),
            PdsFile::Ck3(file)
            | PdsFile::Imperator(file)
            | PdsFile::Eu5(file)
            | PdsFile::Vic3(file) => file.header().kind().is_binary(),
            PdsFile::Hoi4(file) => matches!(file.encoding(), hoi4save::Encoding::Binary),
        }
    }
}

pub enum PdsMeta<'data> {
    Eu4(Box<Eu4Zip<&'data [u8]>>),
    Ck3(jomini::envelope::JominiFile<Cursor<&'data [u8]>>),
    Imperator(jomini::envelope::JominiFile<Cursor<&'data [u8]>>),
    Vic3(jomini::envelope::JominiFile<Cursor<&'data [u8]>>),
    Eu5(jomini::envelope::JominiFile<Cursor<&'data [u8]>>),
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
                JominiFileKind::Uncompressed(SaveDataKind::Text(_)) => Ok(MeltedBuffer::Verbatim),
                JominiFileKind::Uncompressed(SaveDataKind::Binary(binary)) => {
                    let options = ck3save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(ck3save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = ck3save::Ck3Melt::melt(
                        &mut &*binary,
                        options,
                        ck3_tokens_resolver(),
                        &mut output,
                    )?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                JominiFileKind::Zip(zip) => {
                    let options = ck3save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(ck3save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let mut meta = zip.meta()?;
                    let doc = ck3save::Ck3Melt::melt(
                        &mut meta,
                        options,
                        ck3_tokens_resolver(),
                        &mut output,
                    )?;
                    if file.header().kind().is_text() {
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
                JominiFileKind::Uncompressed(SaveDataKind::Text(_)) => Ok(MeltedBuffer::Verbatim),
                JominiFileKind::Uncompressed(SaveDataKind::Binary(binary)) => {
                    let options = imperator_save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(imperator_save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = imperator_save::ImperatorMelt::melt(
                        &mut &*binary,
                        options,
                        imperator_tokens_resolver(),
                        &mut output,
                    )?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                JominiFileKind::Zip(zip) => {
                    let options = imperator_save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(imperator_save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let mut meta = zip.meta()?;
                    let doc = imperator_save::ImperatorMelt::melt(
                        &mut meta,
                        options,
                        imperator_tokens_resolver(),
                        &mut output,
                    )?;
                    if file.header().kind().is_text() {
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
                JominiFileKind::Uncompressed(SaveDataKind::Text(_)) => Ok(MeltedBuffer::Verbatim),
                JominiFileKind::Uncompressed(SaveDataKind::Binary(binary)) => {
                    let options = vic3save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(vic3save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let doc = vic3save::Vic3Melt::melt(
                        &mut &*binary,
                        options,
                        vic3_tokens_resolver(),
                        &mut output,
                    )?;
                    Ok(MeltedBuffer::Binary {
                        body: output.into_inner(),
                        unknown_tokens: !doc.unknown_tokens().is_empty(),
                    })
                }
                JominiFileKind::Zip(zip) => {
                    let options = vic3save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(vic3save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let mut meta = zip.meta()?;
                    let doc = vic3save::Vic3Melt::melt(
                        &mut meta,
                        options,
                        vic3_tokens_resolver(),
                        &mut output,
                    )?;
                    if file.header().kind().is_text() {
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
            PdsMeta::Eu5(file) => match file.kind() {
                JominiFileKind::Uncompressed(SaveDataKind::Text(_)) => Ok(MeltedBuffer::Verbatim),
                JominiFileKind::Uncompressed(SaveDataKind::Binary(_)) => Err(
                    LibError::UnsupportedOperation(String::from("melting uncompressed eu5 binary")),
                ),
                JominiFileKind::Zip(zip) => {
                    let options = eu5save::MeltOptions::new()
                        .verbatim(true)
                        .on_failed_resolve(eu5save::FailedResolveStrategy::Stringify);
                    let mut output = Cursor::new(Vec::new());
                    let mut meta = zip.meta()?;
                    let resolver = eu5save::SaveResolver::create(zip, eu5_tokens_resolver())?;
                    let doc = eu5save::Eu5Melt::melt(&mut meta, options, resolver, &mut output)?;
                    if file.header().kind().is_text() {
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
