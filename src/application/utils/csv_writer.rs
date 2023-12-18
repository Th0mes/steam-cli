use crate::domain::traits::csv_record::ToCsvRecord;
use std::error::Error;

pub fn write_to_csv<T>(items: &[T], filename: &str, headers: &[&str]) -> Result<(), Box<dyn Error>>
where
    T: ToCsvRecord,
{
    let path = std::path::Path::new(filename);
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(headers)?;

    for item in items {
        let record = item.to_csv_record();
        writer.write_record(&record)?;
    }

    writer.flush()?;
    Ok(())
}
