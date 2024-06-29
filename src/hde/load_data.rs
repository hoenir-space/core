use serde_xml_rs::{from_str};
use crate::hde::data_structure::{GraphData};

pub fn by_str(str: &str ) -> GraphData {

    let data: GraphData = from_str(str).unwrap();
    return data;
}