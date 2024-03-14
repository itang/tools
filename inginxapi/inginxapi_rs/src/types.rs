#[derive(Debug)]
pub struct IndicesValues {
    pub request_time: f64,
    pub response_time: f64,
}

#[derive(Debug)]
pub struct Pair {
    pub name: String,
    pub value: f64,
}

#[derive(Debug)]
pub struct IndicesResult {
    pub name: String,
    pub avg: f64,
    pub max: f64,
    pub min: f64,
    pub percent_list: Vec<Pair>,
}

pub struct StatResult {
    pub count: u64,
    pub indices_items: Vec<IndicesResult>,
}
