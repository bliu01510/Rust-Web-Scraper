use std::{io::Write, fs::OpenOptions};
use thirtyfour_sync::{prelude as wbd, WebDriverCommands};
mod Item;
mod Price;
mod Shop;
use Item::ItemMethods;

fn main() {
    // Creating the webdriver object
    let driver =
        wbd::WebDriver::new("http://localhost:4444", wbd::DesiredCapabilities::chrome()).unwrap();

    // Performing product price scraping on the challenger website
    let result = Item::ChallengerItem::scrape_webpage(&driver).unwrap();
    println!("Number of items retrieved: {}", result.len());

    // Writing to a local json file
    {

        // Opening the file object
        let opened_file = match OpenOptions::new().read(true).append(true).create(true).open("data/product_prices.json"){
            Ok(opened_file) => opened_file,
            Err(err) => panic!("Issue during the file opening process. Terminating program.")
        };

        // Appending each struct to the file object
        serde_json::to_writer(&opened_file, &result).expect("Failed to write to json file.")

    } // Provide a bounded scope to ensure that the opened file object will be automatically closed.

    // Performing the shutdown of the browser window
    driver.quit().unwrap();
}
