use serde::{Deserialize};
use crate::cmn;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubLayer {
    pub id: i32,
    pub name: String,
    pub layer_type: String,
    pub alias: Option<String>,
    pub discipline: Option<String>,
    pub model_name: Option<String>,
    pub visibility: Option<bool>,
    #[serde(rename="sublayers")]
    pub sub_layers: Option<Vec<SubLayer>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub summary: Vec<AttributeStats>,
}

#[derive(Debug, Deserialize)]
pub enum MostFrequentValueTypeOptions {
    Str(String),
    Int(i32),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStats {
    pub field_name: String,
    pub sub_layer_ids: Vec<i32>,
    pub label: Option<String>,
    pub model_name: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub most_frequent_values: Option<Vec<MostFrequentValueTypeOptions>>
}

fn default_layer_type() -> String {
    "Building".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfo {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub alias: String,
    #[serde(default = "default_layer_type")]
    pub layer_type: String,
    pub filters: Vec<Filter>,
    #[serde(rename="sublayers")]
    pub sub_layers: Vec<SubLayer>,
    pub full_extent: cmn::FullExtent,
    pub spatial_reference: cmn::SpatialReference,
    pub descrition: Option<String>,
    pub copyright_text: Option<String>,
    pub height_model_info: Option<cmn::HeightModelInfo>,
    pub active_filter_id: Option<String>,
    #[serde(rename="statisticsHRef")]
    pub statistics_href: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub id: String,
    pub name: String,
    pub description: String,
    pub filter_bloccks: Vec<FilterBlock>,
    pub is_default_filter: Option<bool>,
    pub is_visible: Option<bool>,
    pub filter_authoring_info: Option<FilterAuthoringInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterBlock {
    pub title: String,
    pub filter_mode: FilterMode,
    pub filter_expression: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterAuthoringInfo {}


fn default_solid_filter_mode_type() -> String {
    "solid".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterModeSolid {
    #[serde(rename="type", default = "default_solid_filter_mode_type")]
    pub filter_type: String,
}

fn default_wire_frame_filter_mode_type() -> String {
    "wireFrame".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterModeWireFrame {
    #[serde(rename="type", default = "default_wire_frame_filter_mode_type")]
    pub filter_type: String,
    pub edges: Option<Edges>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edges {
    #[serde(rename = "type")]
    pub type_field: String,
    pub color: Option<Vec<i64>>,
    pub size: Option<f64>,
    pub transparency: Option<i64>,
    pub extension_length: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Solid(FilterModeSolid),
    WireFrame(FilterModeWireFrame),
}