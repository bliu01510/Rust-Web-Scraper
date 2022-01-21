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

// fn test_scraping_func() -> wbd::WebDriverResult<()>{
//     let webdriver_capabilities = wbd::DesiredCapabilities::chrome();
//     let driver = wbd::WebDriver::new("http://localhost:4444", &webdriver_capabilities)?;

//     driver.get("https://www.challenger.sg/apple/iphone-m")?;

//     //  Selecting the maximum number of items to be loaded (144)
//     let item_count_selector = driver.find_element(wbd::By::XPath("/html/body/div[1]/main/div[2]/div/div/div[2]/div/div[3]/div[2]/div/div/div[1]")).unwrap();
//     item_count_selector.click()?;

//     let item_count_selector = driver.find_element(wbd::By::XPath("/html/body/div[1]/main/div[2]/div/div/div[2]/div/div[3]/div[2]/div/div/div[2]/ul[2]/li[3]")).unwrap();
//     item_count_selector.click()?;

//     let element_nodes = driver.find_elements(wbd::By::Css("div.item-body"))?;

//     for element_node in element_nodes {
//         let item_price = element_node.find_element(wbd::By::Css("p.text-red-600")).unwrap();
//         println!("{}",item_price.text().unwrap());
//     }

//     // println!("{}",element_nodes.len().to_string());

//     // let source = driver.page_source()?;
//     // let mut file = std::fs::File::create("HTML_Output.txt").unwrap();
//     // file.write_all(source.as_bytes());

//     driver.quit()?;

//     Ok(())
// }
