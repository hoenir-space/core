use std::fs;
use crate::hde::data_structure::{GraphData};
use yaserde::ser::{Config, to_string_with_config};


pub fn to_xml_str(data: GraphData ) -> String {

    let conf = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: Some("\t".parse().unwrap()),
    };

    let xml_str = to_string_with_config(&data, &conf).unwrap();

    return xml_str;
}

pub fn to_file_path(xml_str:&str, path:&str) {

    fs::write(path, &xml_str).expect("Saving to file failed: hoenir_core::hde::save_data::to_file_path()");

}