//! Example Nodle Plugin
//! 
//! This is a template for creating Nodle plugins. It demonstrates the basic
//! structure and provides examples of implementing custom nodes.

use nodle_plugin_sdk::*;
use std::collections::HashMap;

/// The main plugin struct
pub struct ExamplePlugin;

impl NodePlugin for ExamplePlugin {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Example Plugin".to_string(),
            version: "0.1.0".to_string(),
            author: "Plugin Developer".to_string(),
            description: "An example plugin demonstrating basic functionality".to_string(),
            compatible_version: "0.1.0".to_string(),
        }
    }
    
    fn register_nodes(&self, registry: &mut dyn NodeRegistryTrait) {
        // Register our custom nodes
        registry.register_node_factory(Box::new(HelloWorldNodeFactory)).unwrap();
        registry.register_node_factory(Box::new(MathAddNodeFactory)).unwrap();
    }
    
    fn on_load(&self) -> Result<(), PluginError> {
        println!("Example Plugin loaded successfully!");
        Ok(())
    }
    
    fn on_unload(&self) -> Result<(), PluginError> {
        println!("Example Plugin unloaded");
        Ok(())
    }
}

/// Factory for HelloWorld node
pub struct HelloWorldNodeFactory;

impl NodeFactory for HelloWorldNodeFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "HelloWorld",
            "Hello World",
            NodeCategory::utility(),
            "A simple node that displays a greeting"
        )
        .with_workspace_compatibility(vec!["3D", "General"])
        .with_color(Color32::from_rgb(100, 200, 100))
        .with_icon("👋")
        .with_outputs(vec![
            PortDefinition::required("Message", DataType::String)
        ])
    }
    
    fn create_node(&self, position: Pos2) -> PluginNodeHandle {
        PluginNodeHandle::new(Box::new(HelloWorldNode::new(position)))
    }
}

/// HelloWorld node implementation
pub struct HelloWorldNode {
    id: String,
    position: Pos2,
    message: String,
}

impl HelloWorldNode {
    fn new(position: Pos2) -> Self {
        Self {
            id: format!("hello_world_{}", uuid()),
            position,
            message: "Hello from plugin!".to_string(),
        }
    }
}

impl PluginNode for HelloWorldNode {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn position(&self) -> Pos2 {
        self.position
    }
    
    fn set_position(&mut self, position: Pos2) {
        self.position = position;
    }
    
    fn get_parameter_ui(&self) -> ParameterUI {
        let mut elements = Vec::new();
        
        elements.push(UIElement::Heading("Hello World Node".to_string()));
        elements.push(UIElement::Separator);
        
        elements.push(UIElement::TextEdit {
            label: "Message".to_string(),
            value: self.message.clone(),
            parameter_name: "message".to_string(),
        });
        
        ParameterUI { elements }
    }
    
    fn handle_ui_action(&mut self, action: UIAction) -> Vec<ParameterChange> {
        let mut changes = Vec::new();
        
        match action {
            UIAction::ParameterChanged { parameter, value } => {
                match parameter.as_str() {
                    "message" => {
                        if let Some(msg) = value.as_string() {
                            self.message = msg.to_string();
                            changes.push(ParameterChange {
                                parameter: "message".to_string(),
                                value: NodeData::String(self.message.clone()),
                            });
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        changes
    }
    
    fn get_parameter(&self, name: &str) -> Option<NodeData> {
        match name {
            "message" => Some(NodeData::String(self.message.clone())),
            _ => None,
        }
    }
    
    fn set_parameter(&mut self, name: &str, value: NodeData) {
        match name {
            "message" => {
                if let Some(msg) = value.as_string() {
                    self.message = msg.to_string();
                }
            }
            _ => {}
        }
    }
    
    fn process(&mut self, _inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        let mut outputs = HashMap::new();
        outputs.insert("Message".to_string(), NodeData::String(self.message.clone()));
        outputs
    }
}

/// Factory for Math Add node
pub struct MathAddNodeFactory;

impl NodeFactory for MathAddNodeFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "PluginMathAdd",
            "Add (Plugin)",
            NodeCategory::math(),
            "Adds two numbers together (from plugin)"
        )
        .with_workspace_compatibility(vec!["3D", "General"])
        .with_color(Color32::from_rgb(100, 150, 255))
        .with_icon("➕")
        .with_inputs(vec![
            PortDefinition::required("A", DataType::Float),
            PortDefinition::required("B", DataType::Float),
        ])
        .with_outputs(vec![
            PortDefinition::required("Result", DataType::Float)
        ])
    }
    
    fn create_node(&self, position: Pos2) -> PluginNodeHandle {
        PluginNodeHandle::new(Box::new(MathAddNode::new(position)))
    }
}

/// Math Add node implementation
pub struct MathAddNode {
    id: String,
    position: Pos2,
    a: f32,
    b: f32,
}

impl MathAddNode {
    fn new(position: Pos2) -> Self {
        Self {
            id: format!("math_add_{}", uuid()),
            position,
            a: 0.0,
            b: 0.0,
        }
    }
}

impl PluginNode for MathAddNode {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn position(&self) -> Pos2 {
        self.position
    }
    
    fn set_position(&mut self, position: Pos2) {
        self.position = position;
    }
    
    fn get_parameter_ui(&self) -> ParameterUI {
        let mut elements = Vec::new();
        
        elements.push(UIElement::Heading("Math Add Node (Plugin)".to_string()));
        elements.push(UIElement::Separator);
        
        elements.push(UIElement::Slider {
            label: "A".to_string(),
            value: self.a,
            min: -100.0,
            max: 100.0,
            parameter_name: "a".to_string(),
        });
        
        elements.push(UIElement::Slider {
            label: "B".to_string(),
            value: self.b,
            min: -100.0,
            max: 100.0,
            parameter_name: "b".to_string(),
        });
        
        elements.push(UIElement::Label(format!("Result: {}", self.a + self.b)));
        
        ParameterUI { elements }
    }
    
    fn handle_ui_action(&mut self, action: UIAction) -> Vec<ParameterChange> {
        let mut changes = Vec::new();
        
        match action {
            UIAction::ParameterChanged { parameter, value } => {
                match parameter.as_str() {
                    "a" => {
                        if let Some(val) = value.as_float() {
                            self.a = val;
                            changes.push(ParameterChange {
                                parameter: "a".to_string(),
                                value: NodeData::Float(val),
                            });
                        }
                    }
                    "b" => {
                        if let Some(val) = value.as_float() {
                            self.b = val;
                            changes.push(ParameterChange {
                                parameter: "b".to_string(),
                                value: NodeData::Float(val),
                            });
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        changes
    }
    
    fn get_parameter(&self, name: &str) -> Option<NodeData> {
        match name {
            "a" => Some(NodeData::Float(self.a)),
            "b" => Some(NodeData::Float(self.b)),
            _ => None,
        }
    }
    
    fn set_parameter(&mut self, name: &str, value: NodeData) {
        match name {
            "a" => {
                if let Some(val) = value.as_float() {
                    self.a = val;
                }
            }
            "b" => {
                if let Some(val) = value.as_float() {
                    self.b = val;
                }
            }
            _ => {}
        }
    }
    
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        // Update from inputs if available
        if let Some(NodeData::Float(a)) = inputs.get("A") {
            self.a = *a;
        }
        if let Some(NodeData::Float(b)) = inputs.get("B") {
            self.b = *b;
        }
        
        let mut outputs = HashMap::new();
        outputs.insert("Result".to_string(), NodeData::Float(self.a + self.b));
        outputs
    }
}

/// Simple UUID generation for demo purposes
fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)
}

/// Plugin entry point - MUST use PluginHandle for safe FFI transfer
/// This avoids undefined behavior when passing trait objects through extern "C"
#[no_mangle]
pub extern "C" fn create_plugin() -> PluginHandle {
    PluginHandle::new(Box::new(ExamplePlugin))
}

/// Plugin cleanup - MUST use PluginHandle for safe FFI transfer
#[no_mangle]
pub extern "C" fn destroy_plugin(handle: PluginHandle) {
    // Plugin will be dropped when handle goes out of scope
    let _ = unsafe { handle.into_plugin() };
}