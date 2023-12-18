use crate::domain::traits::csv_record::ToCsvRecord;

pub struct SteamSales {
    pub url: Option<String>,
    pub title: Option<String>,
    pub discount_pct: Option<String>,
    pub original_price: Option<String>,
    pub discount_price: Option<String>,
}

impl ToCsvRecord for SteamSales {
    fn to_csv_record(&self) -> Vec<String> {
        vec![
            self.url.as_ref().unwrap().to_owned(),
            self.title.as_ref().unwrap().to_owned(),
            self.discount_pct.as_ref().unwrap().to_owned(),
            self.original_price.as_ref().unwrap().to_owned(),
            self.discount_price.as_ref().unwrap().to_owned(),
        ]
    }
}
