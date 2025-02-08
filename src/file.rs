use crate::{errors::LibError, melter::Melter, MeltedBuffer};
use ck3save::{
    file::{Ck3Meta, Ck3MetaKind},
    Ck3File,
};
use eu4save::{
    file::{Eu4FileEntry, Eu4FileEntryName},
    Eu4File,
};
use hoi4save::Hoi4File;
use imperator_save::{
    file::{ImperatorMeta, ImperatorMetaKind},
    ImperatorFile,
};
use vic3save::{
    file::{Vic3Meta, Vic3MetaData},
    Vic3File,
};

pub enum PdsFileResult<'a> {
    Ok(PdsFile<'a>),
    Err(LibError),
}

pub enum PdsFile<'a> {
    Eu4(Eu4File<'a>),
    Ck3(Ck3File<'a>),
    Imperator(ImperatorFile<'a>),
    Hoi4(Hoi4File<'a>),
    Vic3(Vic3File<'a>),
}

impl PdsFile<'_> {
    pub(crate) fn meta(&self) -> Option<PdsMeta> {
        match self {
            PdsFile::Eu4(file) => {
                let mut entries = file.entries();
                while let Some(entry) = entries.next_entry() {
                    if let Some(Eu4FileEntryName::Meta) = entry.name() {
                        return Some(PdsMeta::Eu4(entry));
                    }
                }

                None
            }
            PdsFile::Ck3(file) => Some(PdsMeta::Ck3(file.meta())),
            PdsFile::Imperator(file) => Some(PdsMeta::Imperator(file.meta())),
            PdsFile::Hoi4(_) => None,
            PdsFile::Vic3(file) => file.meta().ok().map(PdsMeta::Vic3),
        }
    }

    pub(crate) fn melt_file(&self) -> Result<MeltedBuffer, LibError> {
        match self {
            PdsFile::Eu4(file) => Melter::melt(file.melter()),
            PdsFile::Ck3(file) => Melter::melt(file.melter()),
            PdsFile::Imperator(file) => Melter::melt(file.melter()),
            PdsFile::Hoi4(file) => Melter::melt(file.melter()),
            PdsFile::Vic3(file) => Melter::melt(file.melter()),
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
    Eu4(Eu4FileEntry<'data>),
    Ck3(Ck3Meta<'data>),
    Imperator(ImperatorMeta<'data>),
    Vic3(Vic3Meta<'data>),
}

impl PdsMeta<'_> {
    pub(crate) fn melt(&self) -> Result<MeltedBuffer, LibError> {
        match self {
            PdsMeta::Eu4(entry) => {
                if matches!(entry.encoding(), eu4save::Encoding::Text) {
                    return Ok(MeltedBuffer::Verbatim);
                }

                Melter::melt(entry.melter())
            }
            PdsMeta::Ck3(file) => match file.kind() {
                Ck3MetaKind::InlinedText(x) => {
                    let mut out_header = Vec::new();
                    file.header().write(&mut out_header).unwrap();
                    Ok(MeltedBuffer::Text {
                        header: out_header,
                        body: x.to_vec(),
                    })
                }
                _ => Melter::melt(file.melter()),
            },
            PdsMeta::Imperator(file) => match file.kind() {
                ImperatorMetaKind::Text(x) => {
                    let mut out_header = Vec::new();
                    file.header().write(&mut out_header).unwrap();
                    Ok(MeltedBuffer::Text {
                        header: out_header,
                        body: x.to_vec(),
                    })
                }
                ImperatorMetaKind::Binary(_) => Melter::melt(file.melter()),
            },
            PdsMeta::Vic3(file) => match file.kind() {
                Vic3MetaData::Text(x) => {
                    let mut out_header = Vec::new();
                    file.header().write(&mut out_header).unwrap();
                    Ok(MeltedBuffer::Text {
                        header: out_header,
                        body: x.to_vec(),
                    })
                }
                Vic3MetaData::Binary(_) => Melter::melt(file.melter()),
            },
        }
    }
}
