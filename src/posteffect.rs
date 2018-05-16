use glium::backend::Facade;
use glium::framebuffer::SimpleFrameBuffer;
use glium::index::{IndexBuffer, PrimitiveType};
use glium::texture::{DepthFormat, DepthTexture2d, MipmapsOption, Texture2d};
use glium::vertex::VertexBuffer;
use glium::Surface;

use types::Vertex;

use time;

/// struct representing a post effect
pub struct PostEffect {
    /// The vertex buffer to render
    pub vertex_buffer: VertexBuffer<Vertex>,
    /// The index buffer to render
    pub index_buffer: IndexBuffer<u16>,
    /// The current shader being used for post processing
    pub current_shader: &'static str,
    /// The time the post effect was initialised
    pub start_time: f32,
    /// The scale factor that the scene will be rendered
    /// in relation to to the full window resolution
    pub downscale_factor: f32,
    /// Options for the default post shader
    pub post_shader_options: PostShaderOptions,
}

impl PostEffect {
    /// creates a new instance of a post effect
    pub fn new<F>(facade: &F) -> PostEffect
    where
        F: Facade + Clone,
    {
        let vert_arr = [
            Vertex {
                position: [-1.0, -1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [1.0, 0.0],
            },
        ];

        let ind_arr = [1 as u16, 2, 0, 3];

        PostEffect {
            vertex_buffer: VertexBuffer::new(facade, &vert_arr).unwrap(),
            index_buffer: IndexBuffer::new(facade, PrimitiveType::TriangleStrip, &ind_arr).unwrap(),
            current_shader: "default",
            start_time: time::precise_time_s() as f32,
            downscale_factor: 1.0f32,
            post_shader_options: PostShaderOptionsBuilder::default().build().unwrap(),
        }
    }
}

/// Contains all the options for the default post shader
#[derive(Builder, Clone, PartialEq)]
pub struct PostShaderOptions {
    /// The offset for the chromatic aberration
    #[builder(default = "0.003f32")]
    pub chrom_offset: f32,
    /// The mix amount for the chromatic aberration, if this is at 0.0 chromatic aberration is off
    #[builder(default = "0.0f32")]
    pub chrom_amt: f32,
    /// Whether blur is on
    #[builder(default = "false")]
    pub blur: bool,
    /// The amount of blur
    #[builder(default = "1.0f32")]
    pub blur_amt: f32,
    /// The radius of the blur
    #[builder(default = "1.0f32")]
    pub blur_radius: f32,
    /// The weight of the blur
    #[builder(default = "1.0f32")]
    pub blur_weight: f32,
    /// Whether bokeh is on
    #[builder(default = "false")]
    pub bokeh: bool,
    /// Bokeh focal depth
    #[builder(default = "0.2f32")]
    pub bokeh_focal_depth: f32,
    /// Bokeh focal width
    #[builder(default = "0.2f32")]
    pub bokeh_focal_width: f32,
    /// Colour grading
    #[builder(default = "(1f32, 1f32, 1f32, 1f32)")]
    pub color_offset: (f32, f32, f32, f32),
    /// Greyscale
    #[builder(default = "false")]
    pub greyscale: bool,
    /// Noise
    #[builder(default = "0f32")]
    pub noise: f32,
}

/// Renders the post effect on to the scene rendered in the draw FnMut
pub fn render_to_texture<T, F, C>(
    system: &PostEffect,
    context: &C,
    target: &mut T,
    mut draw: F,
) -> (Texture2d, DepthTexture2d)
where
    T: Surface,
    F: FnMut(&mut SimpleFrameBuffer),
    C: Facade + Clone,
{
    let target_dimensions = target.get_dimensions();

    let target_color = Texture2d::empty(
        context,
        (target_dimensions.0 as f32 * system.downscale_factor) as u32,
        (target_dimensions.1 as f32 * system.downscale_factor) as u32,
    ).unwrap();

    let target_depth = DepthTexture2d::empty_with_format(
        context,
        DepthFormat::F32,
        MipmapsOption::NoMipmap,
        (target_dimensions.0 as f32 * system.downscale_factor) as u32,
        (target_dimensions.1 as f32 * system.downscale_factor) as u32,
    ).unwrap();

    // first pass draw the scene into a buffer
    draw(&mut SimpleFrameBuffer::with_depth_buffer(context, &target_color, &target_depth).unwrap());

    (target_color, target_depth)
}
