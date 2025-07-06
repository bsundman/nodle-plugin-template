# Nodle Plugin Development Guide

## Overview

This guide explains how to develop safe and stable plugins for Nodle using the proper patterns and avoiding common pitfalls.

## Critical: Safe Wrapper Pattern

**IMPORTANT:** All Nodle plugins MUST use the safe wrapper pattern to avoid undefined behavior and crashes.

### The Problem: Trait Objects and FFI

Passing trait objects (like `Box<dyn NodePlugin>`) directly through `extern "C"` functions causes undefined behavior in Rust. This leads to:

- Random crashes when calling methods
- Memory corruption
- Unpredictable behavior
- Vtable corruption

### The Solution: PluginHandle and PluginNodeHandle

Nodle provides safe wrapper types that encapsulate trait objects for FFI transfer:

```rust
#[repr(C)]
pub struct PluginHandle {
    plugin: *mut dyn NodePlugin,
}

#[repr(C)]
pub struct PluginNodeHandle {
    node: *mut dyn PluginNode,
}
```

## Required Export Functions

### Plugin Entry Point

```rust
/// Plugin entry point - MUST use PluginHandle for safe FFI transfer
/// This avoids undefined behavior when passing trait objects through extern "C"
#[no_mangle]
pub extern "C" fn create_plugin() -> PluginHandle {
    PluginHandle::new(Box::new(YourPlugin))
}

/// Plugin cleanup - MUST use PluginHandle for safe FFI transfer
#[no_mangle]
pub extern "C" fn destroy_plugin(handle: PluginHandle) {
    // Plugin will be dropped when handle goes out of scope
    let _ = unsafe { handle.into_plugin() };
}
```

### Node Factory Implementation

```rust
impl NodeFactory for YourNodeFactory {
    fn metadata(&self) -> NodeMetadata {
        // Return node metadata
    }
    
    fn create_node(&self, position: Pos2) -> PluginNodeHandle {
        PluginNodeHandle::new(Box::new(YourNode::new(position)))
    }
}
```

## Modern Plugin Interface

Plugins should use the data-driven UI approach instead of direct egui rendering:

### Instead of `render_parameters` (deprecated):
```rust
// OLD WAY - DO NOT USE
fn render_parameters(&mut self, ui: &mut Ui) -> Vec<ParameterChange> {
    // Direct egui rendering - causes crashes
}
```

### Use `get_parameter_ui` and `handle_ui_action`:
```rust
fn get_parameter_ui(&self) -> ParameterUI {
    let mut elements = Vec::new();
    
    elements.push(UIElement::Heading("Your Node".to_string()));
    elements.push(UIElement::Separator);
    
    elements.push(UIElement::TextEdit {
        label: "Parameter Name".to_string(),
        value: self.parameter_value.clone(),
        parameter_name: "parameter_name".to_string(),
    });
    
    ParameterUI { elements }
}

fn handle_ui_action(&mut self, action: UIAction) -> Vec<ParameterChange> {
    let mut changes = Vec::new();
    
    match action {
        UIAction::ParameterChanged { parameter, value } => {
            match parameter.as_str() {
                "parameter_name" => {
                    if let Some(val) = value.as_string() {
                        self.parameter_value = val.to_string();
                        changes.push(ParameterChange {
                            parameter: "parameter_name".to_string(),
                            value: NodeData::String(self.parameter_value.clone()),
                        });
                    }
                }
                _ => {}
            }
        }
        UIAction::ButtonClicked { action } => {
            // Handle button clicks
        }
    }
    
    changes
}
```

## Available UI Elements

The plugin SDK provides these UI elements:

- `UIElement::Heading(String)` - Section headings
- `UIElement::Label(String)` - Read-only text
- `UIElement::Separator` - Visual separator
- `UIElement::TextEdit` - Text input field
- `UIElement::Checkbox` - Boolean checkbox
- `UIElement::Button` - Clickable button
- `UIElement::Slider` - Numeric slider
- `UIElement::Vec3Edit` - 3D vector editor
- `UIElement::ColorEdit` - Color picker
- `UIElement::Horizontal(Vec<UIElement>)` - Horizontal layout
- `UIElement::Vertical(Vec<UIElement>)` - Vertical layout

## Node Data Types

Supported parameter types:

```rust
pub enum NodeData {
    Float(f32),
    Vector3([f32; 3]),
    Color([f32; 3]),
    String(String),
    Boolean(bool),
}
```

## Best Practices

1. **Always use PluginHandle and PluginNodeHandle** - Never pass trait objects directly
2. **Use data-driven UI** - Describe your interface, don't render directly
3. **Handle panics gracefully** - The core will catch panics and display error messages
4. **Test thoroughly** - Plugin crashes can affect the entire application
5. **Follow naming conventions** - Use clear, descriptive names for nodes and parameters

## Common Mistakes to Avoid

1. **DON'T** return `Box<dyn PluginNode>` from `create_node`
2. **DON'T** export `extern "C"` functions that return trait objects
3. **DON'T** use `render_parameters` method (deprecated)
4. **DON'T** assume plugins have access to egui directly
5. **DON'T** pass complex types through FFI boundaries

## Building Your Plugin

1. Add `nodle-plugin-sdk` dependency to `Cargo.toml`
2. Implement `NodePlugin` trait for your plugin struct
3. Implement `NodeFactory` trait for each node type
4. Implement `PluginNode` trait for each node implementation
5. Export `create_plugin` and `destroy_plugin` functions with proper signatures
6. Build as a `cdylib` crate type

## Testing

Always test your plugins thoroughly:

1. Load the plugin in Nodle
2. Create nodes from your plugin
3. Interact with the parameter interface
4. Connect nodes and process data
5. Test edge cases and error conditions

## Why This Pattern Works

The safe wrapper pattern works because:

1. **ABI Stability**: `#[repr(C)]` ensures consistent memory layout
2. **Controlled Lifetime**: Handles manage trait object lifetimes safely
3. **Panic Safety**: The core can catch and handle plugin panics
4. **Type Safety**: Rust's type system prevents many common errors
5. **Data-Driven**: UI description separates concerns cleanly

This pattern ensures that plugins are stable, safe, and maintainable while providing full functionality for node-based workflows.