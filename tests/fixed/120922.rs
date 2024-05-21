use serde::{Serialize, Serializer};

fn serialize_price_matrix<T, S>(_: &Vec<Vec<f64>>, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
{
    serializer.serialize_f64(0.0)
}

#[derive(Serialize)]
pub(crate) struct Matrix {
    #[serde(serialize_with = "serialize_price_matrix")]
    matrix: Vec<Vec<f64>>,
}

fn main() {
}
