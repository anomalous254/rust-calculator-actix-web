use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct CalcQuery {
    pub operation: String,
    pub num1: f64,
    pub num2: f64
}


#[derive(Debug, Serialize)]
pub struct CalcResponse{
    pub result: f64
}