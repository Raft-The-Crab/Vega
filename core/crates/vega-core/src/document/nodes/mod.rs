//! Node types for the document graph.

use crate::color::ColorSpace;
use crate::document::ids::structure::NodeId;
use crate::geometry::{Point, Rect, Size};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The type of a node in the document graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    /// A raster/pixel layer.
    Raster,

    /// A group of other nodes.
    Group,

    /// An adjustment layer (non-destructive adjustment).
    Adjustment,

    /// A text layer.
    Text,

    /// A vector shape layer.
    Shape,

    /// A mask node.
    Mask,

    /// An embedded/linked asset (smart object).
    Asset,
}

/// Blend mode for compositing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::Normal
    }
}

/// Common properties shared by all nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeProperties {
    /// The unique ID of this node.
    pub id: NodeId,

    /// The type of this node.
    pub node_type: NodeType,

    /// Display name.
    pub name: String,

    /// Whether the node is visible.
    pub visible: bool,

    /// Whether the node is locked (cannot be edited).
    pub locked: bool,

    /// Opacity (0.0 to 1.0).
    pub opacity: f32,

    /// Blend mode.
    pub blend_mode: BlendMode,

    /// Transform (position, scale, rotation).
    pub transform: Transform,
}

impl NodeProperties {
    /// Create new node properties.
    pub fn new(id: NodeId, node_type: NodeType, name: String) -> Self {
        Self {
            id,
            node_type,
            name,
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: BlendMode::default(),
            transform: Transform::default(),
        }
    }
}

/// Transform for a node (simplified for now).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    /// Translation (x, y).
    pub translation: (f32, f32),

    /// Scale (x, y).
    pub scale: (f32, f32),

    /// Rotation in degrees.
    pub rotation: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: (0.0, 0.0),
            scale: (1.0, 1.0),
            rotation: 0.0,
        }
    }
}

/// A node in the document graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    /// A raster layer with pixel data.
    Raster {
        properties: NodeProperties,
        bounds: Rect,
        color_space: ColorSpace,
        // Pixel data will be stored separately in tile storage
    },

    /// A group containing child nodes.
    Group {
        properties: NodeProperties,
        children: Vec<NodeId>,
    },

    /// An adjustment layer with non-destructive adjustments.
    Adjustment {
        properties: NodeProperties,
        adjustment_type: AdjustmentType,
        parameters: HashMap<String, f32>,
    },

    /// A text layer.
    Text {
        properties: NodeProperties,
        text: String,
        font_family: String,
        font_size: f32,
        color: crate::color::Rgba8,
    },

    /// A vector shape layer.
    Shape {
        properties: NodeProperties,
        shape_type: ShapeType,
        fill: Option<crate::color::Rgba8>,
        stroke: Option<crate::color::Rgba8>,
        stroke_width: f32,
    },

    /// A mask node.
    Mask {
        properties: NodeProperties,
        mask_data: MaskData,
    },

    /// An embedded asset.
    Asset {
        properties: NodeProperties,
        asset_id: String,
        linked: bool,
    },
}

impl Node {
    /// Create a new node with the given name and type.
    pub fn new(name: String, node_type: NodeType) -> Self {
        let id = NodeId::generate();
        let properties = NodeProperties::new(id, node_type, name);

        match node_type {
            NodeType::Raster => Node::Raster {
                properties,
                bounds: Rect::default(),
                color_space: ColorSpace::default(),
            },
            NodeType::Group => Node::Group {
                properties,
                children: Vec::new(),
            },
            NodeType::Adjustment => Node::Adjustment {
                properties,
                adjustment_type: AdjustmentType::BrightnessContrast,
                parameters: HashMap::new(),
            },
            NodeType::Text => Node::Text {
                properties,
                text: String::new(),
                font_family: "Arial".to_string(),
                font_size: 12.0,
                color: crate::color::Rgba8::BLACK,
            },
            NodeType::Shape => Node::Shape {
                properties,
                shape_type: ShapeType::Rectangle,
                fill: Some(crate::color::Rgba8::WHITE),
                stroke: None,
                stroke_width: 1.0,
            },
            NodeType::Mask => Node::Mask {
                properties,
                mask_data: MaskData::Raster {
                    bounds: Rect::default(),
                },
            },
            NodeType::Asset => Node::Asset {
                properties,
                asset_id: String::new(),
                linked: false,
            },
        }
    }

    /// Get the node's properties.
    pub fn properties(&self) -> &NodeProperties {
        match self {
            Node::Raster { properties, .. } => properties,
            Node::Group { properties, .. } => properties,
            Node::Adjustment { properties, .. } => properties,
            Node::Text { properties, .. } => properties,
            Node::Shape { properties, .. } => properties,
            Node::Mask { properties, .. } => properties,
            Node::Asset { properties, .. } => properties,
        }
    }

    /// Get a mutable reference to the node's properties.
    pub fn properties_mut(&mut self) -> &mut NodeProperties {
        match self {
            Node::Raster { properties, .. } => properties,
            Node::Group { properties, .. } => properties,
            Node::Adjustment { properties, .. } => properties,
            Node::Text { properties, .. } => properties,
            Node::Shape { properties, .. } => properties,
            Node::Mask { properties, .. } => properties,
            Node::Asset { properties, .. } => properties,
        }
    }

    /// Get the node's ID.
    pub fn id(&self) -> NodeId {
        self.properties().id
    }

    /// Get the node's type.
    pub fn node_type(&self) -> NodeType {
        self.properties().node_type
    }

    /// Get the node's name.
    pub fn name(&self) -> &str {
        &self.properties().name
    }

    /// Set the node's name.
    pub fn set_name(&mut self, name: String) {
        self.properties_mut().name = name;
    }
}

/// Types of adjustments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdjustmentType {
    BrightnessContrast,
    HueSaturation,
    Levels,
    Curves,
    ColorBalance,
    Exposure,
    Vibrance,
    Invert,
    Grayscale,
    Sepia,
}

/// Types of shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShapeType {
    Rectangle,
    Ellipse,
    Line,
    Polygon { sides: u32 },
    Path,
}

/// Mask data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaskData {
    /// Raster mask with pixel data.
    Raster {
        bounds: Rect,
        // Pixel data stored separately
    },
    /// Vector mask with path data.
    Vector {
        path: String, // Simplified path representation
    },
}
