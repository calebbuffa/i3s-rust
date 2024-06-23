use std::error::Error;

use crate::{io, Rest};

use serde::Deserialize;
use serde_json;

pub fn get_node_page_index(node_index: &f32, nodes_per_page: &f32) -> i32 {
    (node_index / nodes_per_page).floor() as i32
}

pub fn get_node_index_in_node_page(node_index: &f32, nodes_per_page: &f32) -> i32 {
    (node_index % nodes_per_page) as i32
}

fn default_texture_value() -> f32 {
    1.0
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfo {
    pub id: usize,
    pub layer_type: String,
    pub capabilities: Vec<String>,
    pub store: Store,
    pub geometry_definitions: Vec<GeometryDefinition>,
    #[serde(default)]
    pub node_pages: NodePageDefinition,
    #[serde(default)]
    pub name: String,
    pub full_extent: Option<FullExtent>,
    pub spatial_reference: Option<SpatialReference>,
    #[serde(default)]
    pub alias: String,
    pub service_update_time_stamp: Option<ServiceUpdateTimeStamp>,
    pub height_model_info: Option<HeightModelInfo>,
    pub drawing_info: Option<DrawingInfo>,
    pub material_definitions: Option<Vec<MaterialDefinitions>>,
    pub texture_set_definitions: Option<Vec<TextureSetDefinition>>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub copyright_text: String,
    #[serde(default)]
    pub href: String,
    pub cached_drawing_info: Option<CachedDrawingInfo>,
    #[serde(rename = "disablePopup")]
    pub disable_pop_up: Option<bool>,
    pub attribute_storage_info: Option<Vec<AttributeStorageInfo>>,
    pub statistics_info: Option<Vec<StatisticsInfo>>,
    pub z_factor: Option<f32>,
    #[serde(rename = "popupInfo")]
    pub pop_up_info: Option<PopUpInfo>,
    pub fields: Option<Vec<Field>>,
    pub elevation_info: Option<ElevationInfo>,
}

impl io::SLPKReader for SceneLayerInfo {}
impl Default for SceneLayerInfo {
    fn default() -> Self {
        Self {
            id: 0,
            layer_type: String::new(),
            capabilities: vec![],
            store: Store::default(),
            geometry_definitions: vec![],
            node_pages: NodePageDefinition::default(),
            name: String::new(),
            full_extent: None,
            spatial_reference: None,
            alias: String::new(),
            service_update_time_stamp: None,
            height_model_info: None,
            drawing_info: None,
            material_definitions: None,
            texture_set_definitions: None,
            description: String::new(),
            copyright_text: String::new(),
            href: String::new(),
            cached_drawing_info: None,
            disable_pop_up: None,
            attribute_storage_info: None,
            statistics_info: None,
            z_factor: None,
            pop_up_info: None,
            fields: None,
            elevation_info: None,
        }
    }
}

impl SceneLayerInfo {
    fn rest_path() -> String {
        format!("layers/{}", 0)
    }

    pub async fn from_rest(stream: &Rest) -> Result<Self, Box<dyn Error>> {
        let resp = stream.get(Self::rest_path().as_str()).await?;
        let result = resp.json::<Self>().await?;
        Ok(result)
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevationInfo {
    #[serde(default)]
    pub mode: String,
    pub offset: Option<f32>,
    #[serde(default)]
    pub unit: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub alias: String,
    pub domain: Option<Domain>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    #[serde(rename = "type")]
    pub domain_type: String,
    pub name: String,
    pub coded_values: Option<Vec<DomainCodedValue>>,
    #[serde(default)]
    pub description: String,
    pub range: Option<[f32; 2]>,
    #[serde(default)]
    pub field_type: String,
    #[serde(default)]
    pub merge_policy: String,
    #[serde(default)]
    pub split_policy: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainCodedValue {
    pub name: String,
    pub code: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopUpInfo {
    #[serde(default)]
    pub title: String,
    pub media_infos: Option<Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub description: String,
    pub field_infos: Option<Vec<FieldInfo>>,
    pub popup_elements: Option<Vec<PopupElement>>,
    pub expression_infos: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    #[serde(default)]
    pub field_name: String,
    pub visible: Option<bool>,
    pub is_editable: Option<bool>,
    #[serde(default)]
    pub label: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopupElement {
    #[serde(default)]
    pub text: String,
    #[serde(rename = "type", default)]
    pub popup_element_type: String,
    pub field_infos: Option<Vec<FieldInfo>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsInfo {
    pub key: String,
    pub name: String,
    pub href: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStorageInfo {
    pub key: usize,
    pub name: String,
    pub header: Vec<HeaderValue>,
    pub ordering: Option<Vec<String>>,
    pub attribute_values: Option<Value>,
    pub attribute_byte_counts: Option<Value>,
    pub object_ids: Option<Value>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderValue {
    pub value_type: String,
    pub property: String,
}

fn default_time_encoding() -> String {
    "ECMA_ISO8601".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value_type: String,
    #[serde(default)]
    pub encoding: String,
    #[serde(default = "default_time_encoding")]
    pub time_encoding: String,
    pub values_per_element: Option<f32>,
}

impl Default for Value {
    fn default() -> Self {
        Self {
            value_type: String::new(),
            encoding: String::new(),
            time_encoding: default_time_encoding(),
            values_per_element: None,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CachedDrawingInfo {
    pub color: bool,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawingInfo {
    pub renderer: Renderer,
    pub scale_symbols: Option<bool>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Renderer {
    #[serde(rename = "type", default)]
    pub renderer_type: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub description: String,
    pub symbol: Option<Symbol>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    #[serde(rename = "type", default)]
    pub symbol_type: String,
    pub symbol_layers: Option<Vec<SymbolLayer>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolLayer {
    #[serde(rename = "type", default)]
    pub symbol_layer_type: String,
    pub material: Option<Material>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Material {
    pub color: Option<Vec<i32>>,
    pub transparency: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullExtent {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
    pub spatial_reference: Option<SpatialReference>,
}

impl Default for FullExtent {
    fn default() -> Self {
        Self {
            xmin: -1.0,
            xmax: -1.0,
            ymin: -1.0,
            ymax: -1.0,
            zmin: -1.0,
            zmax: -1.0,
            spatial_reference: None,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialReference {
    pub latest_vcs_wkid: Option<i32>,
    pub latest_wkid: Option<i32>,
    pub vcs_wkid: Option<i32>,
    pub wkid: Option<i32>,
    #[serde(default)]
    pub wkt: String,
}

fn default_geometry_definition_topology() -> String {
    "triangle".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryDefinition {
    pub geometry_buffers: Vec<GeometryBuffer>,
    #[serde(default = "default_geometry_definition_topology")]
    pub topology: String,
}

impl Default for GeometryDefinition {
    fn default() -> Self {
        Self {
            geometry_buffers: vec![],
            topology: default_geometry_definition_topology(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryPosition {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
}

impl GeometryPosition {
    pub fn new() -> Self {
        Self {
            dtype: "Float32".to_string(),
            component: 3,
        }
    }
}

impl Default for GeometryPosition {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryNormal {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
}

impl GeometryNormal {
    pub fn new() -> Self {
        Self {
            dtype: "Float32".to_string(),
            component: 3,
        }
    }
}

impl Default for GeometryNormal {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryUV {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
}

impl GeometryUV {
    pub fn new() -> Self {
        Self {
            dtype: "Float32".to_string(),
            component: 2,
        }
    }
}

impl Default for GeometryUV {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryColor {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
}

impl GeometryColor {
    pub fn new() -> Self {
        Self {
            dtype: "Uint8".to_string(),
            component: 3,
        }
    }
}

impl Default for GeometryColor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryFeatureID {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
    pub binding: String,
}

impl GeometryFeatureID {
    pub fn new() -> Self {
        Self {
            dtype: "UInt32".to_string(),
            component: 1,
            binding: "per-feature".to_string(),
        }
    }
}

impl Default for GeometryFeatureID {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryFaceRange {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
    pub binding: String,
}

impl GeometryFaceRange {
    pub fn new(component: Option<i32>) -> Self {
        if let Some(component) = component {
            return Self {
                dtype: "Uint32".to_string(),
                component: component,
                binding: "per-feature".to_string(),
            };
        } else {
            return Self {
                dtype: "Uint32".to_string(),
                component: -1,
                binding: "per-feature".to_string(),
            };
        }
    }
}

impl Default for GeometryFaceRange {
    fn default() -> Self {
        Self::new(None)
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryBuffer {
    #[serde(default)]
    pub offset: i32,
    #[serde(default)]
    pub position: GeometryPosition,
    #[serde(default)]
    pub normal: GeometryNormal,
    #[serde(default)]
    pub uv0: GeometryUV,
    #[serde(default)]
    pub feature_id: GeometryFeatureID,
    #[serde(default)]
    pub face_range: GeometryFaceRange,
    #[serde(default)]
    pub compressed_attributes: CompressedAttributes,
}

fn default_compressed_attributes_encoding() -> String {
    "draco".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompressedAttributes {
    #[serde(default = "default_compressed_attributes_encoding")]
    pub encoding: String,
    pub attributes: Option<Vec<String>>,
}

impl CompressedAttributes {
    pub fn new() -> Self {
        Self {
            encoding: default_compressed_attributes_encoding(),
            attributes: None,
        }
    }
}

impl Default for CompressedAttributes {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GeometryBufferMetadata {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
    #[serde(default)]
    pub binding: String,
    #[serde(default)]
    pub encoding: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeightModelInfo {
    #[serde(default)]
    pub height_model: String,
    #[serde(rename = "vertCRS", default)]
    pub vert_crs: String,
    #[serde(default)]
    pub height_unit: String,
}

fn default_alpha_cutoff() -> f32 {
    0.25
}

fn default_cull_face() -> String {
    "none".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinitions {
    #[serde(default)]
    pub pbr_metallic_roughness: PbrMetallicRoughness,
    pub normal_texture: Option<MaterialTexture>,
    pub occlusion_texture: Option<MaterialTexture>,
    pub emissive_texture: Option<MaterialTexture>,
    pub emissive_factor: Option<[f32; 3]>,
    #[serde(default)]
    pub alpha_mode: String,
    #[serde(default = "default_alpha_cutoff")]
    pub alpha_cutoff: f32,
    #[serde(default)]
    pub double_sided: bool,
    #[serde(default = "default_cull_face")]
    pub cull_face: String,
}

impl Default for MaterialDefinitions {
    fn default() -> Self {
        Self {
            pbr_metallic_roughness: PbrMetallicRoughness::default(),
            normal_texture: None,
            occlusion_texture: None,
            emissive_texture: None,
            emissive_factor: None,
            alpha_mode: String::new(),
            alpha_cutoff: default_alpha_cutoff(),
            double_sided: false,
            cull_face: default_cull_face(),
        }
    }
}

fn default_base_color_factor() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    #[serde(default = "default_base_color_factor")]
    pub base_color_factor: [f32; 4],
    #[serde(default = "default_texture_value")]
    pub metallic_factor: f32,
    #[serde(default = "default_texture_value")]
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<MaterialTexture>,
    pub base_color_texture: Option<MaterialTexture>,
}

impl Default for PbrMetallicRoughness {
    fn default() -> Self {
        Self {
            base_color_factor: default_base_color_factor(),
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            base_color_texture: None,
            metallic_roughness_texture: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTexture {
    pub texture_set_definition_id: i32,
    #[serde(default = "default_texture_value")]
    pub factor: f32,
    #[serde(default)]
    #[deprecated]
    pub tex_coord: i32,
}

impl Default for MaterialTexture {
    fn default() -> Self {
        Self {
            texture_set_definition_id: -1,
            factor: default_texture_value(),
            tex_coord: 0,
        }
    }
}

fn default_nodes_per_page() -> i32 {
    64
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePageDefinition {
    #[serde(default = "default_nodes_per_page")]
    pub nodes_per_page: i32,
    pub lod_selection_metric_type: String,
    #[serde(default)]
    pub root_index: usize, // default is 0
}

impl Default for NodePageDefinition {
    fn default() -> Self {
        Self {
            nodes_per_page: default_nodes_per_page(),
            lod_selection_metric_type: String::new(),
            root_index: 0,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUpdateTimeStamp {
    pub last_update: i64, // Unix epoch from 1 January 1970 in milliseconds
}

impl Default for ServiceUpdateTimeStamp {
    fn default() -> Self {
        Self { last_update: -1 }
    }
}

fn default_root_node_path() -> String {
    "./nodes/root".to_string()
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub profile: String,
    pub version: String,
    #[serde(default = "default_root_node_path")]
    pub root_node: String,
    #[serde(default)]
    pub id: String,
    pub resource_pattern: Option<Vec<String>>,
    pub extent: Option<Vec<f64>>,
    #[serde(rename = "indexCRS", default)]
    pub index_crs: String,
    #[serde(rename = "vertexCRS", default)]
    pub vertex_crs: String,
    #[serde(default)]
    pub normal_reference_frame: String,
    pub default_geometry_schema: Option<DefaultGeometrySchema>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub nid_encoding: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub feature_encoding: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub geometry_encoding: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub attribute_encoding: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub texture_encoding: Option<Vec<String>>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub lod_type: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub lod_model: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub indexing_scheme: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub default_texture_definition: Option<Vec<Texture>>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub default_material_definition: Option<MaterialDefinition>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinition {
    #[deprecated(note = "MaterialDefinition was deprecated in 1.7")]
    pub identifier: MaterialDefinitionInfo,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinitionInfo {
    pub name: String,
    #[serde(rename = "type", default)]
    pub dtype: String,
    #[serde(default)]
    pub href: String,
    pub params: Option<MaterialParams>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialParams {
    pub render_mode: String,
    #[serde(default)]
    pub vertex_colors: bool,
    #[serde(default)]
    pub vertex_regions: bool,
    #[serde(default)]
    pub use_vertex_color_alpha: bool,
    pub transparency: Option<i32>,
    pub reflectivity: Option<i32>,
    pub shininess: Option<i32>,
    pub ambient: Option<Vec<i32>>,
    pub diffuse: Option<Vec<i32>>,
    pub specular: Option<Vec<f32>>,
    pub cast_shadows: Option<bool>,
    pub receive_shadows: Option<bool>,
    #[serde(default)]
    pub cull_face: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    pub encoding: Option<Vec<String>>,
    pub wrap: Option<Vec<String>>,
    pub atlas: Option<bool>,
    #[serde(default)]
    pub uv_set: String,
    pub channels: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultGeometrySchema {
    pub header: Vec<HeaderAttribute>,
    pub topology: String,
    pub ordering: Vec<String>,
    pub vertex_attributes: VertexAttribute,
    pub feature_attribute_order: Vec<String>,
    pub feature_attributes: FeatureAttribute,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureAttribute {
    pub id: Option<Value>,
    pub face_range: Option<Value>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryAttribute {
    pub value_type: String,
    pub values_per_element: i32,
    #[serde(default)]
    pub byte_offset: i32,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct HeaderAttribute {
    pub property: String,
    #[serde(rename = "type")]
    pub dtype: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct VertexAttribute {
    pub position: Option<GeometryAttribute>,
    pub uv0: Option<GeometryAttribute>,
    pub normal: Option<GeometryAttribute>,
    pub color: Option<GeometryAttribute>,
    pub region: Option<GeometryAttribute>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct TextureSetDefinition {
    pub formats: Vec<TextureSetDefinitionFormat>,
    pub atlas: Option<bool>,
}

impl TextureSetDefinition {
    pub fn has_compressed_textures(&self) -> Result<bool, &str> {
        match self.formats.len() {
            1 => Ok(false),
            2 => Ok(true),
            _ => Err("Invalid number of texture formats"),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct TextureSetDefinitionFormat {
    pub name: String,
    pub format: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePage {
    pub nodes: Vec<Node>,
}

impl io::SLPKReader for NodePage {}

fn default_children() -> Vec<usize> {
    vec![]
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub index: usize,
    pub obb: OBB,
    #[serde(default = "default_children")]
    pub children: Vec<usize>,
    pub parent_index: Option<usize>,
    pub lod_threshold: Option<f32>,
    pub mesh: Option<Mesh>,
}

impl Node {
    pub fn is_root(&self) -> bool {
        self.parent_index.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn parent<'a>(&self, nodes: &'a Vec<Node>) -> Option<&'a Node> {
        match self.parent_index {
            Some(parent_index) => Some(&nodes[parent_index]),
            None => None,
        }
    }

    pub fn children<'a>(&self, nodes: &'a Vec<Node>) -> Vec<&'a Node> {
        let mut children = vec![];
        for child_index in &self.children {
            children.push(&nodes[*child_index]);
        }
        children
    }

    pub fn geometry(&self) {
        unimplemented!()
    }

    pub fn texture(&self) {
        unimplemented!()
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OBB {
    pub center: [f64; 3],
    pub half_size: [f64; 3],
    pub quanternion: Option<[f64; 4]>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Mesh {
    #[serde(default)]
    pub material: MeshMaterial,
    #[serde(default)]
    pub geometry: MeshGeometry,
    #[serde(default)]
    pub attribute: MeshAttribute,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshMaterial {
    definition: usize,
    resource: usize,
    texel_count_hint: i32,
}

impl Default for MeshMaterial {
    fn default() -> Self {
        Self {
            definition: 0,
            resource: 0,
            texel_count_hint: -1,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshGeometry {
    definition: usize,
    resource: usize,
    vertex_count: usize,
    feature_count: usize,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct MeshAttribute {
    resource: usize,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "I3SVersion")]
    pub i3s_version: String,
    pub node_count: usize,
}

impl Metadata {
    fn rest_path() -> String {
        "metadata.json".to_string()
    }

    pub async fn from_rest(stream: &Rest) -> Result<Self, Box<dyn Error>> {
        let resp = stream.get(Self::rest_path().as_str()).await?;
        let result = resp.json::<Self>().await?;
        Ok(result)
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureData {
    pub id: usize,
    pub position: Vec<f64>,
    pub pivot_offset: [f64; 3],
    pub mbb: [f64; 6],
    pub layer: String,
    pub attributes: FeatureAttribute,
    pub geometries: Geometry,
}

impl Default for FeatureData {
    fn default() -> Self {
        Self {
            id: 0,
            position: vec![],
            pivot_offset: [0.0, 0.0, 0.0],
            mbb: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            layer: String::new(),
            attributes: FeatureAttribute::default(),
            geometries: Geometry::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub id: usize,
    #[serde(rename = "type")]
    pub geometry_type: String,
    pub transformation: [f64; 16],
    pub params: GeometryParams,
}

impl Default for Geometry {
    fn default() -> Self {
        Self {
            id: 0,
            geometry_type: String::new(),
            transformation: [0.0; 16],
            params: GeometryParams::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GeometryParams {
    Reference(GeometryReferenceParams),
    Vested(VestedGeometryParams),
    SingleComponent(SingleComponentParams),
}

impl Default for GeometryParams {
    fn default() -> Self {
        Self::Reference(GeometryReferenceParams::default())
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryReferenceParams {
    pub href: String,
    #[serde(rename = "type", default)]
    pub geometry_type: String,
    pub face_range: Option<Vec<i32>>,
    pub lod_geometry: Option<bool>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VestedGeometryParams {
    pub geometry_type: String,
    pub topology: String,
    pub vertex_attributes: VertexAttribute,
    pub faces: GeometryAttribute,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleComponentParams {
    pub id: usize,
    #[serde(default)]
    pub material: String,
    #[serde(default)]
    pub texture: String,
    pub material_id: Option<usize>,
    pub texture_id: Option<[usize; 1]>,
    pub region_id: Option<[usize; 1]>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub stats: AttributeStatistics,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStatistics {
    pub total_values_count: Option<usize>,
    pub min: Option<f32>,
    pub max: Option<f32>,
    #[serde(default)]
    pub min_time_str: String,
    #[serde(default)]
    pub max_time_str: String,
    pub count: Option<usize>,
    pub sum: Option<f32>,
    pub avg: Option<f32>,
    pub stddev: Option<f32>,
    pub variance: Option<f32>,
    pub histogram: Option<Histogram>,
    pub most_frequent_values: Option<Vec<ValueCount>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    minimum: f64,
    maximum: f64,
    counts: Vec<u16>, // will never be more than 256
}

impl Default for Histogram {
    fn default() -> Self {
        Self {
            minimum: -1.0,
            maximum: -1.0,
            counts: vec![],
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueCount {
    value: String,
    count: usize,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeIndexDocument {}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SharedResource {}
