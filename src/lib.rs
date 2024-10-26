use bevy::{
    app::{App, Plugin},
    asset::{embedded_asset, Asset},
    color::LinearRgba,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
// use plotters::prelude::*;

#[derive(Debug)]
pub struct PlottersPlugin;

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

impl UiMaterial for PlotUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_plotters/plot.wgsl".into()
    }
}
