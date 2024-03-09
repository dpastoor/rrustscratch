use extendr_api::{prelude::*};

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}


/// Convert a vector of pairs into an R list.
/// @export
#[extendr]
fn pairs_to_list() -> Robj {
    let mut pairs: Vec<(&str, Robj)> = vec![("a", 1.into()), ("b", vec!["c", "d"].into()), ("c", 2.into())];
    pairs.push(("e", 3.into()));
    List::from_pairs(pairs).into()
}

// Convert a vector of pairs into a dataframe
// @export
#[extendr]
fn pairs_to_dataframe() -> Robj {
    let mut pairs: Vec<(&str, Robj)> = vec![("a", 1.into()), ("b", "c".into()), ("d", 2.into())];
    pairs.push(("e", 3.into()));
    let df = List::from_pairs(pairs);
    let classes = vec!["tbl_df", "tbl", "data.frame"];
    df.set_attrib("class", classes).unwrap();
    //df.set_attrib("row.names", vec!["1"].into_iter().collect::<Vec<_>>()).unwrap();
    df.set_attrib("row.names", Strings::from_values((0..1).map(|i| format!("{}", i+1)))).unwrap();
    df.into()
}
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod dynamicdataframes;
    fn pairs_to_list;
    fn pairs_to_dataframe;
    fn hello_world;
}

