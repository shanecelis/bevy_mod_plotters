use bevy::{
    app::{App, Plugin},
    asset::{embedded_asset, Asset},
    color::LinearRgba,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

/// A plugin for to render BGRX images from plotters
///
/// Adds [PlotUiMaterial].
#[derive(Debug)]
pub struct PlottersPlugin;

// prelude
pub mod prelude {
    pub use super::*;
    pub use plotters::{
        // prelude::*,
        // prelude::{Circle, Text},
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
    /// Color multiplied with the image
    #[uniform(0)]
    pub color: LinearRgba,
    /// Image used to represent graph
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
        "embedded://bevy_plotters/plot.wgsl".into()
    }
}
