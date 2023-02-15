use scraper::{Html, Selector};
use reqwest::blocking::get;

// Define a struct to hold the table data
#[derive(Debug)]
struct TableData {
    lz_houston: Vec<f32>,
    lz_north: Vec<f32>,
    lz_south: Vec<f32>,
    lz_west: Vec<f32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make a GET request to the URL
    let url = "https://www.ercot.com/content/cdr/html/20230213_dam_spp.html";
    let body = get(url)?.text()?;
    //println!("{}", body);
    // Parse the HTML
    let document = Html::parse_document(&body);
    //println!("{:#?}", document);

    
    // Define a CSS selector for the table rows
    let row_selector = Selector::parse("table.tableStyle tr").unwrap();

    // Define CSS selectors for the desired table cells
    let lz_houston_selector = Selector::parse("td:nth-child(13)").unwrap();
    let lz_north_selector = Selector::parse("td:nth-child(15)").unwrap();
    let lz_south_selector = Selector::parse("td:nth-child(17)").unwrap();
    let lz_west_selector = Selector::parse("td:nth-child(19)").unwrap();


    // Iterate over the table rows and extract the desired cells
    let mut lz_houston = Vec::new();
    let mut lz_north = Vec::new();
    let mut lz_south = Vec::new();
    let mut lz_west = Vec::new();

    for row in document.select(&row_selector) {
        //println!("{:#?}", row);
        lz_houston.push(row.select(&lz_houston_selector).next().unwrap().text().next().unwrap().parse().unwrap());
        lz_north.push(row.select(&lz_north_selector).next().unwrap().text().next().unwrap().parse().unwrap());
        lz_south.push(row.select(&lz_south_selector).next().unwrap().text().next().unwrap().parse().unwrap());
        lz_west.push(row.select(&lz_west_selector).next().unwrap().text().next().unwrap().parse().unwrap());
    }

    // Create the struct with the extracted data
    let _table_data = TableData {
        lz_houston,
        lz_north,
        lz_south,
        lz_west,
    };

    // Print the struct for testing purposes
    println!("{:#?}", _table_data);

    Ok(())
}