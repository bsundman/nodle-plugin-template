# Nodle Plugin Template

This is a template for creating plugins for the Nodle node editor. It demonstrates how to create custom nodes that can be dynamically loaded at runtime.

## Building the Plugin

```bash
cargo build --release
```

The plugin will be built as a dynamic library:
- **Linux**: `target/release/libnodle_plugin_template.so`
- **macOS**: `target/release/libnodle_plugin_template.dylib`
- **Windows**: `target/release/nodle_plugin_template.dll`

## Installing the Plugin

Copy the built library to one of these directories:
- `~/.nodle/plugins/` (user plugins)
- `./plugins/` (local plugins)

## Example Nodes

This template includes two example nodes:

### HelloWorld Node
- **Category**: Utility
- **Description**: Simple greeting node with editable message
- **Outputs**: Message (String)

### Math Add Node
- **Category**: Math
- **Description**: Adds two numbers together
- **Inputs**: A (Float), B (Float)
- **Outputs**: Result (Float)

## Plugin Structure

### 1. Plugin Implementation

Every plugin must implement the `NodePlugin` trait:

```rust
impl NodePlugin for ExamplePlugin {
    fn plugin_info(&self) -> PluginInfo { ... }
    fn register_nodes(&self, registry: &mut dyn NodeRegistryTrait) { ... }
    fn on_load(&self) -> Result<(), PluginError> { ... }
    fn on_unload(&self) -> Result<(), PluginError> { ... }
}
```

### 2. Node Factory

Each node type needs a factory that implements `NodeFactory`:

```rust
impl NodeFactory for HelloWorldNodeFactory {
    fn metadata(&self) -> NodeMetadata { ... }
    fn create_node(&self, position: Pos2) -> PluginNodeHandle { ... }
}
```

### 3. Node Implementation

The actual node logic implements `PluginNode`:

```rust
impl PluginNode for HelloWorldNode {
    fn id(&self) -> String { ... }
    fn position(&self) -> Pos2 { ... }
    fn set_position(&mut self, position: Pos2) { ... }
    fn get_parameter_ui(&self) -> ParameterUI { ... }
    fn handle_ui_action(&mut self, action: UIAction) -> Vec<ParameterChange> { ... }
    fn get_parameter(&self, name: &str) -> Option<NodeData> { ... }
    fn set_parameter(&mut self, name: &str, value: NodeData) { ... }
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> { ... }
}
```

### 4. Export Functions

**IMPORTANT: Use Safe Wrapper Types**

The plugin must export these C functions using safe wrapper types:

```rust
#[no_mangle]
pub extern "C" fn create_plugin() -> PluginHandle {
    PluginHandle::new(Box::new(ExamplePlugin))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(handle: PluginHandle) {
    let _ = unsafe { handle.into_plugin() };
}
```

**⚠️ NEVER use the old pattern that returns trait objects directly - it causes crashes!**

## Development Tips

1. **Use the SDK**: Always depend on `nodle-plugin-sdk` for interfaces
2. **Use Safe Wrappers**: Always use `PluginHandle` and `PluginNodeHandle` - never pass trait objects directly
3. **Data-Driven UI**: Use `get_parameter_ui()` and `handle_ui_action()` instead of direct egui rendering
4. **Node Metadata**: Provide rich metadata for better integration
5. **Workspace Compatibility**: Specify which workspaces support your nodes
6. **Error Handling**: Use `PluginError` for consistent error reporting
7. **Resource Management**: Clean up properly in `on_unload()`

## ⚠️ Critical Safety Requirements

**READ THE PLUGIN_DEVELOPMENT.md FILE** for detailed safety requirements. Key points:

- Never return `Box<dyn PluginNode>` from `create_node()` - use `PluginNodeHandle`
- Never export `extern "C"` functions that return trait objects directly
- Always use the data-driven UI pattern with `get_parameter_ui()` and `handle_ui_action()`
- Passing trait objects through FFI boundaries causes undefined behavior and crashes

## Customization

To create your own plugin:

1. Copy this template
2. Update `Cargo.toml` with your plugin name
3. Implement your `NodePlugin` struct
4. Create your node factories and implementations
5. Update the `register_nodes()` method to register your nodes

## Testing

Build and copy the plugin to test it:

```bash
cargo build --release
mkdir -p ~/.nodle/plugins
cp target/release/libnodle_plugin_template.dylib ~/.nodle/plugins/
```

Then run Nodle - your nodes should appear in the appropriate workspace menus!