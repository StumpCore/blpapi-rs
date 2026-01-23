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
