use polars::prelude::*;
fn main() {
    let mut df:DataFrame = df!(
        "name" => ["Alice Archer","Ben Brown"],
        "weight" => [57.9,72.5]
    ).unwrap();
    println!("df:{}",df.clone().lazy().select([col("name")]).collect().unwrap());
}
