use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct GraphData {
    graph: Graph
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Graph {
    id:String,
    r#type:String,
    #[serde(rename = "$value")]
    vertices: Vec<Vertex>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Vertex {
    id:String,
    r#type:String,
    #[serde(rename = "$value")]
    content: Vec<VertexContent>
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum VertexContent {
    Property(Property),
    Edge(Edge),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Property {
    r#type:String,
    #[serde(rename(serialize = "value", deserialize = "$value"))]
    value:String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Edge {
    r#type:String,
    value:String,
    target:String,
}