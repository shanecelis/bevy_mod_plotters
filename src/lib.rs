#![doc(html_root_url = "https://docs.rs/bevy_mod_plotters/0.1.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

use bevy::{
    app::{App, Plugin},
    asset::{embedded_asset, Asset},
    color::LinearRgba,
    prelude::*,
    reflect::TypePath,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef, TextureFormat}
    },
};

use plotters::{
    prelude::*,
    backend::BGRXPixel,
};

/// A plugin to render BGRX images from plotters
///
/// Adds [PlotUiMaterial].
#[derive(Debug)]
pub struct PlottersPlugin;

/// prelude
pub mod prelude {
    pub use super::*;
    pub use plotters::{
        backend::BGRXPixel,
        coord::Shift,
    };
}

impl Plugin for PlottersPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "plot.wgsl");
        app.add_plugins(UiMaterialPlugin::<PlotUiMaterial>::default());
    }
}

/// The PlotUiMaterial renders a texture ignoring its alpha channel, so that it
/// will be compatible with `BGRXPixel` format that plotters uses. Instead the
/// alpha channel is taken from the given `color` field.
///
/// NOTE: The plotters crate does not abstain from overwriting the 'X' byte,
/// which for bevy is the alpha channel, so one cannot rely on setting alpha
/// once in the texture data but must set it after every render from plotters.
/// This material exists principally to avoid that alpha resetting operation.
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct PlotUiMaterial {
    /// The `color` RGB is multiplied with the texture's RGB. The `color` alpha
    /// channel is used for the alpha channel for the whole texture.
    #[uniform(0)]
    pub color: LinearRgba,
    /// Image used to represent the plot.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl PlotUiMaterial {
    /// Create a new PlotUiMaterial. Sets the color to white.
    pub fn new(texture: Handle<Image>) -> Self {
        PlotUiMaterial {
            color: LinearRgba::WHITE,
            texture
        }
    }
}

impl UiMaterial for PlotUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_mod_plotters/plot.wgsl".into()
    }
}

/// Convert an image into a plotters backend if possible.
///
/// Will fail if the texture format is not a version of BGRA8.
pub fn try_as_backend(image: &mut Image) -> Option<BitMapBackend::<BGRXPixel>> {
    let desc = &image.texture_descriptor;
    if ! image.asset_usage.contains(RenderAssetUsages::MAIN_WORLD & RenderAssetUsages::RENDER_WORLD) {
        warn!("Expected asset usages for image to include main world and render world.");
    }
    if matches!(desc.format, TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb) {
        let width = desc.size.width;
        let height = desc.size.height;
        BitMapBackend::<BGRXPixel>::with_buffer_and_format(&mut image.data, (width, height)).ok()
    } else {
        None
    }
}
