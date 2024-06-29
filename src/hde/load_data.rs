use std::fs;
use crate::hde::data_structure::{GraphData};
use yaserde::de::from_str;


pub fn by_xml_str(str: &str ) -> GraphData {

    let data: GraphData = from_str(str).unwrap();
    return data;
}

pub fn by_xml_path(path:&str) -> GraphData {

    let data_str = fs::read_to_string(path).unwrap();
    let data = by_xml_str(&data_str);
    return data;
}