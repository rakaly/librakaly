use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibError {
    #[error("eu4 error: {0}")]
    Eu4(#[from] eu4save::Eu4Error),

    #[error("ck3 error: {0}")]
    Ck3(#[from] ck3save::Ck3Error),

    #[error("imperator error: {0}")]
    Imperator(#[from] imperator_save::ImperatorError),

    #[error("hoi4 error: {0}")]
    Hoi4(#[from] hoi4save::Hoi4Error),

    #[error("vic3 error: {0}")]
    Vic3(#[from] vic3save::Vic3Error),

    #[error("eu5 error: {0}")]
    Eu5(#[from] eu5save::Eu5Error),

    #[error("panic! Error message may be on stdout/stderr")]
    Panic,
}

pub struct PdsError {
    msg: String,
}

impl PdsError {
    pub fn msg(&self) -> &str {
        self.msg.as_str()
    }
}

impl<'a> From<&'a LibError> for PdsError {
    fn from(value: &'a LibError) -> Self {
        PdsError {
            msg: value.to_string(),
        }
    }
}
