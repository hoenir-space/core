use yaserde::*;


#[derive(Debug, Clone, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    root = "data",
    rename = "data",
    namespace = "https://hoenir.space/resource/schema/data_graph_v1",
)]
pub struct GraphData {
    #[yaserde(child)]
    graph: Graph
}

#[derive(Debug, Clone, PartialEq, YaDeserialize, YaSerialize)]
pub struct Graph {
    #[yaserde(attribute)]
    id:String,
    #[yaserde(attribute, rename = "type")]
    r#type:String,
    #[yaserde(child)]
    vertex: Vec<Vertex>
}

#[derive(Debug, Clone, PartialEq, YaDeserialize, YaSerialize)]
pub struct Vertex {
    #[yaserde(attribute)]
    id:String,
    #[yaserde(attribute, rename = "type")]
    r#type:String,
    #[yaserde(child)]
    property: Vec<Property>,
    #[yaserde(child)]
    edge: Vec<Edge>
}


#[derive(Debug, Clone, PartialEq, YaDeserialize, YaSerialize)]
pub struct Property {
    #[yaserde(attribute, rename = "type")]
    r#type:String,
    #[yaserde(text)]
    value:String,
}

#[derive(Debug, Clone, PartialEq, YaDeserialize, YaSerialize)]
pub struct Edge {
    #[yaserde(attribute, rename = "type")]
    r#type:String,
    #[yaserde(attribute)]
    value:String,
    #[yaserde(attribute)]
    target:String,
}