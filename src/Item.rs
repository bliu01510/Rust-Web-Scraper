use self::_InternalMethods::_InternalItemMethods;
use crate::Price;
use crate::Shop;
use serde::Serialize;
use thirtyfour_sync::prelude::ElementWaitable;
use std::result;
use thirtyfour_sync;
use thirtyfour_sync::Capabilities;
use thirtyfour_sync::WebDriverCommands;

pub trait ItemMethods {
    fn get_item_name(&self) -> &str;

    fn get_item_description(&self) -> &str;

    fn get_url_link(&self) -> &str;
}

mod _InternalMethods {
    use crate::Price;
    use crate::Shop;

    pub trait _InternalItemMethods {
        // Note that these methods are used to provide internal implementation functionality (E.g. Unwrapping of enums etc)
        fn _get_shop_enum(&self) -> &Shop::ShopNames;

        fn _get_item_price_enum(&self) -> &Price::ItemPrice;
    }
}

pub fn get_shop_name<T>(item: &T) -> &str
where
    T: _InternalMethods::_InternalItemMethods,
{
    match item._get_shop_enum() {
        Shop::ShopNames::TestShop => "Test Shop",
        Shop::ShopNames::ChallengerShop => "Challenger Online Store",
    }
}

pub fn get_item_price<T>(item: &T) -> f64
where
    T: _InternalMethods::_InternalItemMethods,
{
    match item._get_item_price_enum() {
        Price::ItemPrice::USD(i) => *i,
        Price::ItemPrice::SGD(i) => *i,
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TestItem {
    shop_name: Shop::ShopNames,
    url_link: String,
    item_name: String,
    item_price: Price::ItemPrice,
    item_description: String,
}

impl TestItem {
    fn new(
        shop_name: Shop::ShopNames,
        url_link: String,
        item_name: String,
        item_price: Price::ItemPrice,
        item_description: String,
    ) -> TestItem {
        TestItem {
            shop_name: shop_name,
            url_link: url_link,
            item_name: item_name,
            item_price: item_price,
            item_description: item_description,
        }
    }

    pub fn scrape_webpage(
        webdriver: &thirtyfour_sync::GenericWebDriver<
            thirtyfour_sync::http::reqwest_sync::ReqwestDriverSync,
        >,
    ) -> std::result::Result<Vec<TestItem>, thirtyfour_sync::error::WebDriverError> {
        // Constructing the result Vec to be returned
        let mut result: Vec<TestItem> = Vec::new();

        // Loading the corresponding webpage on the headless chrome service
        let url_link: &'static str =
            "https://webscraper.io/test-sites/e-commerce/allinone/computers/laptops";
        webdriver.get(url_link)?; // Propagates a WebDriverError to the calling function

        // Locating each of the element nodes
        let element_nodes =
            webdriver.find_elements(thirtyfour_sync::By::Css("div.col-sm-4.col-lg-4.col-md-4"))?; // Returns a Vec with each element being a WebElement containing the individual item information

        println!("Number of element nodes found: {}", element_nodes.len());

        // Performing the extraction of the useful information for each WebElement
        for element_node in element_nodes {
            // Constructing the TestItem object
            let shop_name = Shop::ShopNames::TestShop;
            let url_link = url_link.to_string();
            let item_name = match element_node
                .find_element(thirtyfour_sync::By::Css("a.title"))?
                .get_property("title")?
            {
                Some(i) => i,
                None => String::from(""),
            };
            let item_description = element_node
                .find_element(thirtyfour_sync::By::Css("p.description"))?
                .text()?;
            let item_price = Price::ItemPrice::USD(
                element_node
                    .find_element(thirtyfour_sync::By::Css("h4.pull-right.price"))?
                    .text()?[1..]
                    .parse::<f64>()
                    .unwrap(),
            );

            // Constructing the TestItem object
            let item = TestItem {
                shop_name: shop_name,
                url_link: url_link,
                item_name: item_name,
                item_price: item_price,
                item_description: item_description,
            };

            result.push(item);
        }

        Ok(result)
    }
}

impl ItemMethods for TestItem {
    fn get_item_name(&self) -> &str {
        &self.item_name
    }

    fn get_item_description(&self) -> &str {
        &self.item_description
    }

    fn get_url_link(&self) -> &str {
        &self.url_link
    }
}

impl _InternalMethods::_InternalItemMethods for TestItem {
    fn _get_item_price_enum(&self) -> &Price::ItemPrice {
        &self.item_price
    }

    fn _get_shop_enum(&self) -> &Shop::ShopNames {
        &self.shop_name
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChallengerItem {
    shop_name: Shop::ShopNames,
    url_link: String,
    item_name: String,
    item_price: Price::ItemPrice,
    item_description: String,
    is_sold_out: bool,
}

impl ChallengerItem {
    fn get_availability_status(&self) -> &bool {
        &self.is_sold_out
    }

    fn new(
        shop_name: Shop::ShopNames,
        url_link: String,
        item_name: String,
        item_price: Price::ItemPrice,
        item_description: String,
        is_sold_out: bool,
    ) -> ChallengerItem {
        ChallengerItem {
            shop_name: shop_name,
            url_link: url_link,
            item_name: item_name,
            item_price: item_price,
            item_description: item_description,
            is_sold_out: is_sold_out,
        }
    }

    pub fn scrape_webpage(
        webdriver: &thirtyfour_sync::GenericWebDriver<
            thirtyfour_sync::http::reqwest_sync::ReqwestDriverSync,
        >,
    ) -> std::result::Result<Vec<ChallengerItem>, thirtyfour_sync::error::WebDriverError> {
        // Defining the collection used to collect the results
        let mut result: Vec<ChallengerItem> = Vec::new();

        // Loading the corresponding webpage on the headless chrome service
        let url_link: &'static str = "https://www.challenger.sg/apple/iphone-m";
        webdriver.get(url_link)?;

        // Locating the dropdown box for the sorting arrangement
        webdriver.find_element(thirtyfour_sync::By::XPath("//div[@class='ais-SortBy']//i[@class='ivu-icon ivu-icon-ios-arrow-down ivu-select-arrow']"))?
        .click()?;
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Selecting the "Price Low to High Options"
        webdriver.find_element(thirtyfour_sync::By::XPath("//div[@class='ais-HitsPerPage']//li[@class='ivu-select-item' and normalize-space(text())='Price Low to High']"))?
        .click()?;
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Locating the dropdown box for the page item count
        webdriver.find_element(thirtyfour_sync::By::XPath("//div[@class='ais-HitsPerPage']//i[@class='ivu-icon ivu-icon-ios-arrow-down ivu-select-arrow']"))?
        .click()?;
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Selecting the "144 per page"
        webdriver.find_element(thirtyfour_sync::By::XPath("//div[@class='ais-HitsPerPage']//li[@class='ivu-select-item' and normalize-space(text())='144 per page']"))?
        .click()?;
        println!("Selected items to be displayed = 144!");
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Locating each element node
        let element_nodes =
            webdriver.find_elements(thirtyfour_sync::By::Css("div.search-item-box"))?;
        println!("Number of element nodes found: {}", element_nodes.len());

        for element_node in element_nodes {
            // Extracting the required information
            let shop_name = Shop::ShopNames::ChallengerShop;
            let url_link = match element_node
                .find_element(thirtyfour_sync::By::ClassName("item-body"))?
                .find_element(thirtyfour_sync::By::Css("a"))?
                .get_property("href")?
            {
                Some(i) => i,
                None => String::from(""),
            };
            let item_name = match element_node
                .find_element(thirtyfour_sync::By::ClassName("product-name"))?
                .find_element(thirtyfour_sync::By::XPath("./div/span/span"))?
                .get_attribute("aria-label")?
            {
                Some(i) => i,
                None => String::from(""),
            };
            let item_price = match element_node
                .find_element(thirtyfour_sync::By::ClassName("product-price"))?
                .find_element(thirtyfour_sync::By::ClassName("text-red-600"))?
                .text()?[12..]
                .to_string()
                .parse::<f64>()
            {
                Ok(i) => i,
                Err(i) => 0.0,
            };
            // let item_price = item_price[13..].to_string();
            let item_description = String::from("");
            let is_sold_out = match element_node
                .find_element(thirtyfour_sync::By::ClassName("product-flag text-red-600"))
            {
                Ok(i) => true,
                Err(e) => false,
            };

            // Constructing the item
            let item = ChallengerItem::new(
                shop_name,
                url_link,
                item_name,
                Price::ItemPrice::SGD(item_price),
                item_description,
                is_sold_out,
            );

            // Appending to the result vector
            result.push(item);
        }

        Ok(result)
    }
}

impl ItemMethods for ChallengerItem {
    fn get_item_name(&self) -> &str {
        &self.item_name
    }

    fn get_item_description(&self) -> &str {
        &self.item_description
    }

    fn get_url_link(&self) -> &str {
        &self.url_link
    }
}

impl _InternalMethods::_InternalItemMethods for ChallengerItem {
    fn _get_item_price_enum(&self) -> &Price::ItemPrice {
        &self.item_price
    }

    fn _get_shop_enum(&self) -> &Shop::ShopNames {
        &self.shop_name
    }
}
