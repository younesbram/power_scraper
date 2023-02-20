use scraper::{Html, Selector};
use reqwest::blocking::get;
use serde::Serialize;
use std::fs::File;
use chrono::{Local, Timelike, Datelike};

// Define a struct to hold the table data
#[derive(Debug)]
struct TableData {
    lz_houston: Vec<f64>,
    lz_north: Vec<f64>,
    lz_south: Vec<f64>,
    lz_west: Vec<f64>,
}

// Define serde structs for serialize(csv/json)
#[derive(Serialize)]
struct Record {
    location: String,
    prices: Vec<f32>,
}

#[derive(Serialize)]
struct TableDataCSV {
    data: Vec<Record>,
}

/// Sends a GET request to the specified URL and returns the response body as a `String`.
/// 
/// # Arguments
///
/// * `url` - A string slice that holds the URL to send the GET request to.
///
/// # Errors
///
/// This function will return an error if the GET request fails or if there is an issue with the response body.
///
/// # Examples
///
/// ```
/// let url = "https://www.example.com";
/// let body = fetch_data(url).unwrap();
/// println!("{}", body);
/// ```
fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Make a GET request to the URL
    let body = get(url)?.text()?;
    Ok(body)
}

/// Parses the HTML body and extracts the desired data.
///
/// # Arguments
///
/// * `body` - A reference to a string containing the HTML body to parse
///
/// # Errors
///
/// Returns an error if there is an issue parsing the HTML or converting the text to a numerical format.
///
/// # Examples
///
/// ```
/// let body = "<html>...</html>";
/// let table_data = parse_data(&body).unwrap();
/// ```
///
fn parse_data(body: &str) -> Result<TableData, Box<dyn std::error::Error>> {
    // Parse the HTML
    let document = Html::parse_document(&body);
    // Define a CSS selector for the table rows
    let row_selector = Selector::parse("table.tableStyle tr").unwrap();

    // Define CSS selectors for the desired table cells based on their position in the table structure.
    // This is done using the nth-child selector in combination with the td tag. While it's possible to
    // select elements based on their text content, this can be less reliable since the text content can
    // vary depending on formatting or other factors.
    let lz_houston_selector = Selector::parse("td:nth-child(12)").unwrap();
    let lz_north_selector = Selector::parse("td:nth-child(14)").unwrap();
    let lz_south_selector = Selector::parse("td:nth-child(16)").unwrap();
    let lz_west_selector = Selector::parse("td:nth-child(17)").unwrap();

    // Iterate over the table rows and extract the desired cells
    let mut lz_houston = Vec::new();
    let mut lz_north = Vec::new();
    let mut lz_south = Vec::new();
    let mut lz_west = Vec::new();

    for row in document.select(&row_selector) {
        // Get the desired cells for the current row
        let houston_cell = row.select(&lz_houston_selector).next();
        let north_cell = row.select(&lz_north_selector).next();
        let south_cell = row.select(&lz_south_selector).next();
        let west_cell = row.select(&lz_west_selector).next();

        // Parse the text of the cells and push the values onto the respective vectors
        if let Some(cell) = houston_cell {
            let text = cell.text().next().unwrap().trim();
            lz_houston.push(text.parse().unwrap());
        }

        if let Some(cell) = north_cell {
            let text = cell.text().next().unwrap().trim();
            lz_north.push(text.parse().unwrap());
        }

        if let Some(cell) = south_cell {
            let text = cell.text().next().unwrap().trim();
            lz_south.push(text.parse().unwrap());
        }

        if let Some(cell) = west_cell {
            let text = cell.text().next().unwrap().trim();
            lz_west.push(text.parse().unwrap());
        }
    }

    // Create the struct with the extracted data
    let table_data = TableData {
        lz_houston,
        lz_north,
        lz_south,
        lz_west,
    };

    Ok(table_data)
}

/// Saves the table data as a CSV file with the filename `powerfullyScraped.csv`.
///
/// # Arguments
///
/// * `data` - A reference to the `TableData` struct containing the table data to be saved.
///
/// # Errors
///
/// Returns an error if there is a problem creating or writing to the file.
fn save_data_csv(data: &TableData) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_writer(File::create("powerfullyScraped.csv")?);
    writer.write_record(&["LZ Houston", "LZ North", "LZ South", "LZ West"])?;
    
    for i in 0..data.lz_houston.len() {
        writer.write_record(&[        
            data.lz_houston[i].to_string(),
            data.lz_north[i].to_string(),
            data.lz_south[i].to_string(),
            data.lz_west[i].to_string(),
        ])?;
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.ercot.com/content/cdr/html/20230214_dam_spp.html";

    let body = fetch_data(url)?;
    let table_data = parse_data(&body)?;

    save_data_csv(&table_data)?;

    Ok(())
}