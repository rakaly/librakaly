use crate::{errors::LibError, melter::Melter, MeltedBuffer};
use ck3save::{
    file::{Ck3Meta, Ck3MetaKind, Ck3ParsedFileKind},
    Ck3File,
};
use eu4save::{
    file::{Eu4FileEntry, Eu4FileEntryName, Eu4ParsedFileKind},
    Eu4File,
};
use hoi4save::{file::Hoi4ParsedFileKind, Hoi4File};
use imperator_save::{
    file::{ImperatorMeta, ImperatorMetaKind, ImperatorParsedFileKind},
    ImperatorFile,
};
use vic3save::{
    file::{Vic3Meta, Vic3MetaData, Vic3ParsedFileKind},
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

impl<'a> PdsFile<'a> {
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
            PdsFile::Eu4(file) => {
                if matches!(file.encoding(), eu4save::Encoding::Text) {
                    return Ok(MeltedBuffer::Verbatim);
                }

                let mut zip_sink = Vec::new();
                let parsed = file.parse(&mut zip_sink)?;
                match parsed.kind() {
                    Eu4ParsedFileKind::Text(_) => Ok(MeltedBuffer::Text {
                        header: b"EU4txt".to_vec(),
                        body: zip_sink,
                    }),
                    Eu4ParsedFileKind::Binary(bin) => bin.melt(),
                }
            }
            PdsFile::Ck3(file) => {
                if matches!(file.encoding(), ck3save::Encoding::Text) {
                    return Ok(MeltedBuffer::Verbatim);
                }

                let mut zip_sink = Vec::new();
                let parsed = file.parse(&mut zip_sink)?;
                match parsed.kind() {
                    Ck3ParsedFileKind::Text(_) => {
                        let mut new_header = file.header().clone();
                        new_header.set_kind(ck3save::SaveHeaderKind::Text);
                        let mut out_header = Vec::new();
                        new_header.write(&mut out_header).unwrap();
                        Ok(MeltedBuffer::Text {
                            header: out_header,
                            body: zip_sink,
                        })
                    }
                    Ck3ParsedFileKind::Binary(bin) => bin.melt(),
                }
            }
            PdsFile::Imperator(file) => {
                if matches!(file.encoding(), imperator_save::Encoding::Text) {
                    return Ok(MeltedBuffer::Verbatim);
                }

                let mut zip_sink = Vec::new();
                let parsed = file.parse(&mut zip_sink)?;
                match parsed.kind() {
                    ImperatorParsedFileKind::Text(_) => {
                        let mut new_header = file.header().clone();
                        new_header.set_kind(imperator_save::SaveHeaderKind::Text);
                        let mut out_header = Vec::new();
                        new_header.write(&mut out_header).unwrap();
                        Ok(MeltedBuffer::Text {
                            header: out_header,
                            body: zip_sink,
                        })
                    }
                    ImperatorParsedFileKind::Binary(bin) => bin.melt(),
                }
            }
            PdsFile::Hoi4(file) => {
                if matches!(file.encoding(), hoi4save::Encoding::Plaintext) {
                    return Ok(MeltedBuffer::Verbatim);
                }

                let parsed = file.parse()?;
                match parsed.kind() {
                    Hoi4ParsedFileKind::Text(_) => Ok(MeltedBuffer::Verbatim),
                    Hoi4ParsedFileKind::Binary(bin) => bin.melt(),
                }
            }
            PdsFile::Vic3(file) => {
                if matches!(file.encoding(), vic3save::Encoding::Text) {
                    return Ok(MeltedBuffer::Verbatim);
                }

                let mut zip_sink = Vec::new();
                let parsed = file.parse(&mut zip_sink)?;
                match parsed.kind() {
                    Vic3ParsedFileKind::Text(_) => {
                        let mut new_header = file.header().clone();
                        new_header.set_kind(vic3save::SaveHeaderKind::Text);
                        let mut out_header = Vec::new();
                        new_header.write(&mut out_header).unwrap();
                        Ok(MeltedBuffer::Text {
                            header: out_header,
                            body: zip_sink,
                        })
                    }
                    Vic3ParsedFileKind::Binary(bin) => bin.melt(),
                }
            }
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

impl<'data> PdsMeta<'data> {
    pub(crate) fn melt(&self) -> Result<MeltedBuffer, LibError> {
        match self {
            PdsMeta::Eu4(entry) => {
                let mut zip_sink = Vec::new();
                let parsed_entry = entry.parse(&mut zip_sink)?;
                match parsed_entry.kind() {
                    Eu4ParsedFileKind::Text(_) => Ok(MeltedBuffer::Text {
                        header: Vec::new(),
                        body: zip_sink,
                    }),
                    Eu4ParsedFileKind::Binary(binary) => binary.melt(),
                }
            }
            PdsMeta::Ck3(file) => match file.kind() {
                Ck3MetaKind::Text(x) => {
                    let mut out_header = Vec::new();
                    file.header().write(&mut out_header).unwrap();
                    Ok(MeltedBuffer::Text {
                        header: out_header,
                        body: x.to_vec(),
                    })
                }
                Ck3MetaKind::Binary(_) => file.parse()?.as_binary().unwrap().melt(),
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
                ImperatorMetaKind::Binary(_) => file.parse()?.as_binary().unwrap().melt(),
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
                Vic3MetaData::Binary(_) => file.parse()?.as_binary().unwrap().melt(),
            },
        }
    }
}
