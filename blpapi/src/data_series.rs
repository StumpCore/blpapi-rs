use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct DataSeries<R> {
    pub ticker: String,
    pub data: R,
}

#[derive(Default, Debug)]
pub struct DataSeriesBuilder<R> {
    pub ticker: String,
    pub values: Vec<R>,
}

impl<R> DataSeriesBuilder<R> {
    /// Create a new timeseries with given capacity
    pub fn with_capacity(capacity: usize, ticker: String) -> Self {
        DataSeriesBuilder {
            ticker,
            values: Vec::with_capacity(capacity),
        }
    }

    fn iter_entries(self, ticker: String) -> impl Iterator<Item = DataSeries<R>> {
        self.values.into_iter().map(move |data| DataSeries {
            data,
            ticker: ticker.to_string(),
        })
    }

    pub fn to_rows(self) -> Vec<DataSeries<R>> {
        let ticker = self.ticker.clone();
        self.iter_entries(ticker).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldSeries {
    pub id: String,
    pub mnemonic: String,
    pub desc: String,
    pub data_type: Option<String>,
    pub field_type: Option<String>,
    pub field_category: Option<String>,
    pub field_documentation: Option<String>,
    pub field_property: HashMap<String, String>,
    pub field_default_formatting: HashMap<String, String>,
    pub field_error: HashMap<String, String>,
    pub other: HashMap<String, String>,
    pub overrides: Vec<String>,
}

#[derive(Default, Debug)]
pub struct FieldSeriesBuilder {
    pub id: String,
    pub mnemonic: String,
    pub desc: String,
    pub data_type: Option<String>,
    pub field_type: Option<String>,
    pub field_category: Option<String>,
    pub field_documentation: Option<String>,
    pub field_property: HashMap<String, String>,
    pub field_default_formatting: HashMap<String, String>,
    pub field_error: HashMap<String, String>,
    pub other: HashMap<String, String>,
    pub overrides: Vec<String>,
}

impl FieldSeriesBuilder {
    pub fn id(&mut self, id: String) {
        self.id = id;
    }
    pub fn mnemonic(&mut self, value: String) {
        self.mnemonic = value;
    }
    pub fn desc(&mut self, value: String) {
        self.desc = value;
    }
    pub fn data_type(&mut self, value: String) {
        self.data_type = Some(value);
    }
    pub fn field_type(&mut self, value: String) {
        self.field_type = Some(value);
    }
    pub fn field_property(&mut self, key: String, value: String) {
        self.field_property.insert(key, value);
    }
    pub fn field_category(&mut self, value: String) {
        self.field_category = Some(value);
    }
    pub fn field_default_formatting(&mut self, key: String, value: String) {
        self.field_default_formatting.insert(key, value);
    }
    pub fn field_error(&mut self, key: String, value: String) {
        self.field_error.insert(key, value);
    }
    pub fn field_documentation(&mut self, value: String) {
        self.field_documentation = Some(value);
    }
    pub fn other(&mut self, name: String, value: String) {
        self.other.insert(name, value);
    }
    pub fn overrides(&mut self, value: String) {
        self.overrides.push(value);
    }
    pub fn build(self) -> FieldSeries {
        FieldSeries {
            id: self.id,
            mnemonic: self.mnemonic,
            desc: self.desc,
            data_type: self.data_type,
            field_type: self.field_type,
            field_category: self.field_category,
            field_default_formatting: self.field_default_formatting,
            field_documentation: self.field_documentation,
            other: self.other,
            field_property: self.field_property,
            field_error: self.field_error,
            overrides: self.overrides,
        }
    }
}
