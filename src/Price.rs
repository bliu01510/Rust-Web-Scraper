#[derive(serde::Serialize, serde::Deserialize)]
pub enum ItemPrice {
    SGD(f64),
    USD(f64),
}
