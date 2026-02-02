use crate::{
    name::{Name, NameBuilder},
    request::Request,
    Error,
};

/// Struct for Override
#[derive(Debug)]
pub struct Override {
    pub field_id: Name,
    pub value: String,
}

impl Override {
    pub fn new(id: impl Into<String>, val: impl Into<String>) -> Self {
        let field_id = id.into();
        let field_id = NameBuilder::default().name(field_id).build();
        let value = val.into();
        Self { field_id, value }
    }
}

/// Struct for TableOveride
#[derive(Debug)]
pub struct TableOverride {
    pub field_id: Name,
    pub row: String,
}

impl TableOverride {
    pub fn new(id: impl Into<String>, row: impl Into<String>) -> Self {
        let field_id = id.into();
        let field_id = NameBuilder::default().name(field_id).build();
        let row = row.into();
        Self { field_id, row }
    }
}

/// Struct for bdp Options
#[derive(Debug, Default)]
pub struct BdpOptions {
    pub return_eids: Option<bool>,
    pub return_formatted_value: Option<bool>,
    pub use_utc: Option<bool>,
    pub force_delay: Option<bool>,
    pub return_null: Option<bool>,
    pub start_sequence_number: Option<i32>,
}

impl BdpOptions {
    /// Crate a new historical options
    pub fn new() -> Self {
        BdpOptions {
            ..BdpOptions::default()
        }
    }

    /// Set max points
    pub fn return_eids(mut self, value: bool) -> Self {
        self.return_eids = Some(value);
        self
    }

    pub fn return_formatted_value(mut self, value: bool) -> Self {
        self.return_formatted_value = Some(value);
        self
    }
    pub fn use_utc(mut self, value: bool) -> Self {
        self.use_utc = Some(value);
        self
    }
    pub fn force_delay(mut self, value: bool) -> Self {
        self.force_delay = Some(value);
        self
    }
    pub fn return_null(mut self, value: bool) -> Self {
        self.return_null = Some(value);
        self
    }
    pub fn start_sequence_number(mut self, value: i32) -> Self {
        self.start_sequence_number = Some(value);
        self
    }

    pub fn apply(&self, request: &mut Request) -> Result<(), Error> {
        let mut element = request.element();
        if let Some(eids) = self.return_eids {
            element.set("returnEids", eids)?;
        }
        if let Some(ft_v) = self.return_formatted_value {
            element.set("returnFormattedValue", ft_v)?;
        }
        if let Some(utc) = self.use_utc {
            element.set("useUTCTime", utc)?;
        }
        if let Some(fd) = self.force_delay {
            element.set("forcedDelay", fd)?;
        }
        if let Some(rnull) = self.return_null {
            element.set("returnNullValue", rnull)?;
        }
        if let Some(st_seq) = self.start_sequence_number {
            element.set("startSequenceNumber", st_seq)?;
        }

        Ok(())
    }
}
