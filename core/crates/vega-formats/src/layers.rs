//! Layer metadata serialization
//!
//! Provides serialization of layer metadata to the .vega file format.

use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use zip::{write::FileOptions, ZipWriter};
use vega_core::{NodeId, Node};
use crate::Result;
use crate::vega::FormatError;

/// Layer metadata for serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerMetadata {
    /// Layer ID.
    pub id: String,
    /// Layer name.
    pub name: String,
    /// Layer type.
    pub layer_type: String,
    /// Visibility.
    pub visible: bool,
    /// Opacity (0.0 to 1.0).
    pub opacity: f32,
    /// Blend mode.
    pub blend_mode: String,
    /// Transform (translation x, y, scale x, y, rotation).
    pub transform: [f32; 5],
    /// Bounds (x, y, width, height).
    pub bounds: [u32; 4],
}

impl LayerMetadata {
    /// Creates layer metadata from a node.
    pub fn from_node(id: NodeId, node: &Node) -> Self {
        let props = node.properties();
        let bounds = node.bounds();

        Self {
            id: id.to_string(),
            name: props.name.clone(),
            layer_type: node.node_type().to_string(),
            visible: props.visible,
            opacity: props.opacity,
            blend_mode: props.blend_mode.to_string(),
            transform: [
                props.transform.translation.0,
                props.transform.translation.1,
                props.transform.scale.0,
                props.transform.scale.1,
                props.transform.rotation,
            ],
            bounds: [
                bounds.x as u32,
                bounds.y as u32,
                bounds.width,
                bounds.height,
            ],
        }
    }
}

/// Layer metadata serializer.
pub struct LayerSerializer;

impl LayerSerializer {
    /// Serializes all layer metadata to the ZIP archive.
    pub fn serialize_layers<W: Write>(
        nodes: &std::collections::HashMap<vega_core::NodeId, Node>,
        zip: &mut ZipWriter<W>,
    ) -> Result<()> {
        for (id, node) in nodes {
            let metadata = LayerMetadata::from_node(*id, node);
            let metadata_json = serde_json::to_string_pretty(&metadata)?;
            
            let path = format!("layers/{}/metadata.json", id);
            zip.start_file(&path, FileOptions::default())?;
            zip.write_all(metadata_json.as_bytes())?;
        }

        Ok(())
    }

    /// Reads layer metadata from the ZIP archive.
    pub fn read_layer_metadata<R: std::io::Read + std::io::Seek>(
        zip: &mut zip::ZipArchive<std::io::BufReader<R>>,
        layer_id: NodeId,
    ) -> Result<LayerMetadata> {
        let path = format!("layers/{}/metadata.json", layer_id);
        let file = zip.by_name(&path)
            .map_err(|_| FormatError::MissingFile(path.clone()))?;

        let mut metadata_json = String::new();
        BufReader::new(file).read_to_string(&mut metadata_json)?;

        let metadata: LayerMetadata = serde_json::from_str(&metadata_json)
            .map_err(|e| FormatError::Corrupted(e.to_string()))?;

        Ok(metadata)
    }

    /// Lists all layer IDs in the ZIP archive.
    pub fn list_layers<R: std::io::Read + std::io::Seek>(
        zip: &mut zip::ZipArchive<std::io::BufReader<R>>,
    ) -> Result<Vec<NodeId>> {
        let mut layer_ids = Vec::new();

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;
            let name = file.name();

            if name.starts_with("layers/") && name.ends_with("/metadata.json") {
                // Extract layer ID from path: "layers/{id}/metadata.json"
                let parts: Vec<&str> = name.split('/').collect();
                if parts.len() >= 2 {
                    if let Ok(id) = parts[1].parse::<u64>() {
                        layer_ids.push(NodeId::from(id));
                    }
                }
            }
        }

        Ok(layer_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vega_core::{ColorSpace, Size, Document, document::nodes::NodeType, commands::base::CommandExecutor, layers::CreateLayerCommand};
    use tempfile::TempDir;

    #[test]
    fn test_layer_metadata_from_node() {
        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut cmd = CreateLayerCommand::new(None, "Layer 1".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd).unwrap();

        let layer_id = cmd.layer_id().unwrap();
        let node = document.get_node(layer_id).unwrap();
        let metadata = LayerMetadata::from_node(layer_id, node);

        assert_eq!(metadata.name, "Layer 1");
        assert!(metadata.visible);
    }

    #[test]
    fn test_layer_metadata_serialization() {
        let metadata = LayerMetadata {
            id: "1".to_string(),
            name: "Test Layer".to_string(),
            layer_type: "Raster".to_string(),
            visible: true,
            opacity: 1.0,
            blend_mode: "Normal".to_string(),
            transform: [0.0, 0.0, 1.0, 1.0, 0.0],
            bounds: [0, 0, 100, 100],
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let restored: LayerMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.name, "Test Layer");
    }
}
