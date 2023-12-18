pub trait ToCsvRecord {
    fn to_csv_record(&self) -> Vec<String>;
}
