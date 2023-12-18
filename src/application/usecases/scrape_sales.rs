use scraper::{ElementRef, Selector};

use crate::domain::models::steam_sales::SteamSales;

pub fn scrape_sales(html_sale: ElementRef) -> SteamSales {
    fn extract_text(selector: &Selector, element: &ElementRef) -> Option<String> {
        element
            .select(selector)
            .next()
            .map(|el| el.text().collect())
    }

    let url = html_sale.attr("href").map(String::from);
    let title = extract_text(&Selector::parse("span.title").unwrap(), &html_sale);
    let discount_pct = extract_text(&Selector::parse(".discount_pct").unwrap(), &html_sale);
    let original_price = extract_text(
        &Selector::parse(".discount_original_price").unwrap(),
        &html_sale,
    );
    let discount_price = extract_text(
        &Selector::parse(".discount_final_price").unwrap(),
        &html_sale,
    );

    SteamSales {
        url,
        title,
        discount_pct,
        original_price,
        discount_price,
    }
}
