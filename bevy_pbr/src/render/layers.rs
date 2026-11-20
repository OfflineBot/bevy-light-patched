use bevy_camera::visibility::{Layer, RenderLayers};
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};

/// Which lights reach which meshes.
///
/// On a light it lists the layers the light shines on, on a mesh the layers the
/// mesh takes light from. A light only shades a mesh when they share a layer.
///
/// A light without `LightLayers` shines on every mesh, a mesh without it is on
/// layer 0. Only layers `0..=24` take part, an entity on a higher layer
/// interacts with every light. Shadows are not affected.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, Clone, PartialEq)]
pub struct LightLayers(pub RenderLayers);

impl LightLayers {
    /// A single layer.
    pub const fn layer(layer: Layer) -> Self {
        Self(RenderLayers::layer(layer))
    }

    /// Several layers.
    pub fn from_layers(layers: &[Layer]) -> Self {
        Self(RenderLayers::from_layers(layers))
    }
}

/// Light flags keep their feature bits in `0..=6`, the layers sit above them.
/// Keep in sync with `LIGHT_LAYERS_SHIFT` in `mesh_view_types.wesl`.
pub(crate) const LIGHT_LAYERS_SHIFT: u32 = 7;
const MASK_BITS: u32 = u32::BITS - LIGHT_LAYERS_SHIFT;
const ALL_LAYERS: u32 = (1 << MASK_BITS) - 1;

/// Packed layers of a light. Lights without [`LightLayers`] shine on everything.
pub(crate) fn light_mask(layers: Option<&LightLayers>) -> u32 {
    layers.map_or(ALL_LAYERS, |layers| pack(&layers.0))
}

/// Packed layers of a mesh. Meshes without [`LightLayers`] are on layer 0.
pub(crate) fn mesh_mask(layers: Option<&LightLayers>) -> u32 {
    layers.map_or(1, |layers| pack(&layers.0))
}

fn pack(layers: &RenderLayers) -> u32 {
    let bits = layers.bits();
    let low = bits.first().copied().unwrap_or_default();
    let unsupported = (low >> MASK_BITS) != 0 || bits.iter().skip(1).any(|&extra| extra != 0);

    if unsupported {
        ALL_LAYERS
    } else {
        low as u32
    }
}
