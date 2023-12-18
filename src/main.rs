use std::error::Error;

use scraper::{Html, Selector};
use steam_cli::{
    application::{usecases::scrape_sales::scrape_sales, utils::csv_writer::write_to_csv},
    domain::models::steam_sales::SteamSales,
};

fn main() -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get("https://store.steampowered.com/search/?specials=1")?;
    let html_content = response.text()?;
    let document = Html::parse_document(&html_content);
    let html_sale_selector = Selector::parse("a.search_result_row").unwrap();
    let html_sales = document.select(&html_sale_selector);

    let mut steam_sales: Vec<SteamSales> = Vec::new();

    for html_sale in html_sales {
        let steam_sale = scrape_sales(html_sale);
        steam_sales.push(steam_sale);
    }

    write_to_csv(
        &steam_sales,
        "sales.csv",
        &[
            "url",
            "title",
            "discount_pct",
            "original_price",
            "discount_price",
        ],
    )?;
    Ok(())
}
