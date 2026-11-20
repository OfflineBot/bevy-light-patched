# How to use

Add to the workspace `Cargo.toml` (bevy has to be exactly 0.20.1):

```toml
[workspace.dependencies]
bevy = { version = "=0.20.1" }

[patch.crates-io]
bevy_pbr = { git = "https://github.com/OfflineBot/bevy-light-patched", branch = "v0.20.1-light-layers" }
bevy_render = { git = "https://github.com/OfflineBot/bevy-light-patched", branch = "v0.20.1-light-layers" }
```

Put `LightLayers` on lights and meshes. A light only shades meshes it shares a layer with.

```rust
commands.spawn((DirectionalLight::default(), LightLayers::layer(1)));
commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), LightLayers::layer(1)));
commands.spawn((SpotLight::default(), LightLayers::from_layers(&[0, 1])));
```

- lights without `LightLayers` shine on every mesh
- meshes without `LightLayers` are on layer 0
- layers 0 to 24
- shadows are not affected, every mesh still casts into every shadow map
