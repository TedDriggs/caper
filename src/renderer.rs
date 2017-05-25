use glium::{ Display, DrawParameters, DisplayBuild, Surface, Depth, Blend };
use glium::index::{ NoIndices, PrimitiveType };
use glium::DepthTest::IfLess;
use glium::vertex::VertexBuffer;
use glium::glutin::{ WindowBuilder, get_primary_monitor, GlRequest, Api };
use glium::glutin::CursorState::Hide;//{ Grab, Hide };
use glium::draw_parameters::{ DepthClamp, BackfaceCullingMode };
use glium::texture::RawImage2d;

use glium_text;
use glium_text::{ TextSystem, FontTexture, TextDisplay };

use time;
use std::default::Default;
use fps_counter::FPSCounter;

use imgui::{ ImGui, Ui };
use imgui::glium_renderer::Renderer as ImGuiRenderer;

use image;
use gif;
use gif::SetParameter;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::thread;


use shader::Shaders;
use utils::*;
use posteffect::*;
use types::*;
use lighting::Lighting;


/// struct for abstracting the render state
pub struct Renderer {
    /// The glium display used for rendering
    pub display: Display,
    /// The glium_text system used for rendering TextItem
    pub text_system: TextSystem,
    default_font: FontTexture,
    imgui: ImGui,
    imgui_rend: ImGuiRenderer,
    /// Instance of PostEffect used for rendering post processing
    pub post_effect: PostEffect,
    /// The render/engine start time
    pub start_time: f64,
    /// The shaders that can be used for rendering
    pub shaders: Shaders,
    fps_counter: FPSCounter,
    /// The current frames per second the Renderer is drawing at
    pub fps: f32,
    gif_info: Option<GifInfo>,
    /// The lighting system
    pub lighting: Lighting,
    /// The number items rendered in the last drawn frame
    pub render_count: usize,
}

struct GifInfo {
    encoder: gif::Encoder<File>,
    path: &'static str,
}

impl Renderer {
    /// Creates new Renderer instance
    pub fn new(title:String) -> Renderer {
        // create a diplay instance
        let display = WindowBuilder::new()
            .with_depth_buffer(24)
            .with_title(title)
            .with_vsync()
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 0)))
            .with_fullscreen(get_primary_monitor())
            .build_glium()
            .unwrap();

        // create a text system instance and font
        let text_system = TextSystem::new(&display);
        let font = FontTexture::new(&display, &include_bytes!("./resources/font.ttf")[..],
        100, glium_text::FontTexture::ascii_character_list()).unwrap();

        let mut imgui = ImGui::init();
        let imgui_rend = ImGuiRenderer::init(&mut imgui, &display).unwrap();

        let shaders = Shaders::new(&display);
        let post_fx = PostEffect::new(&display);
        let lighting = Lighting::new(&display);

        let fps_counter = FPSCounter::new();

        let renderer = Renderer {
            display: display,
            text_system: text_system,
            default_font: font,
            imgui: imgui,
            imgui_rend: imgui_rend,
            post_effect: post_fx,
            start_time: time::precise_time_s(),
            shaders: shaders,
            fps_counter: fps_counter,
            fps: 0f32,
            gif_info: None,
            lighting: lighting,
            render_count: 0usize,
        };

        renderer.setup();

        renderer
    }

    /// Sets up the render window
    pub fn setup(&self) {
        // get the window for various values
        let window = self.display.get_window().unwrap();
        window.set_cursor_state(Hide).ok();
    }

    /// Update imgui's interal input state
    pub fn update_imgui_input(&mut self, pos: (i32, i32), btns: (bool, bool, bool)) {
        self.imgui.set_mouse_pos(pos.0 as f32, pos.1 as f32);
        self.imgui.set_mouse_down(&[btns.0, btns.1, btns.2, false, false]);
        //self.imgui.set_mouse_wheel(self.mouse_wheel);
    }

    /// Test whether an object is in the view frustrum
    fn frustrum_test(pos: Vector3, radius: f32, frustrum_planes: Vec<(f32, f32, f32, f32)>) -> bool {
        for plane in frustrum_planes {
            if dotp(&[pos.0, pos.1, pos.2], &[plane.0, plane.1, plane.2]) + plane.3 <= -radius {
                // sphere not in frustrum
                return false;
            }
        }

        true
    }

    /// Helper function that converts viewing matrix into frustum planes
    fn get_frustum_planes(matrix: Matrix4) -> Vec<(f32, f32, f32, f32)> {
        let mut planes = Vec::new();

        // column-major
        // Left clipping plane
        planes.push((matrix[3][0] + matrix[0][0],
                     matrix[3][1] + matrix[0][1],
                     matrix[3][2] + matrix[0][2],
                     matrix[3][3] + matrix[0][3]));
        // Right clipping plane
        planes.push((matrix[3][0] - matrix[0][0],
                     matrix[3][1] - matrix[0][1],
                     matrix[3][2] - matrix[0][2],
                     matrix[3][3] - matrix[0][3]));
        // Top clipping plane
        planes.push((matrix[3][0] - matrix[1][0],
                     matrix[3][1] - matrix[1][1],
                     matrix[3][2] - matrix[1][2],
                     matrix[3][3] - matrix[1][3]));
        // Bottom clipping plane
        planes.push((matrix[3][0] + matrix[1][0],
                     matrix[3][1] + matrix[1][1],
                     matrix[3][2] + matrix[1][2],
                     matrix[3][3] + matrix[1][3]));
        // Near clipping plane
        planes.push((matrix[3][0] + matrix[2][0],
                     matrix[3][1] + matrix[2][1],
                     matrix[3][2] + matrix[2][2],
                     matrix[3][3] + matrix[2][3]));
        // Far clipping plane
        planes.push((matrix[3][0] - matrix[2][0],
                     matrix[3][1] - matrix[2][1],
                     matrix[3][2] - matrix[2][2],
                     matrix[3][3] - matrix[2][3]));

        planes
    }

    /// Draws a frame
    pub fn draw<F: FnMut(&Ui)>(&mut self,
                               cam_state: &CamState,
                               render_items: &Vec<RenderItem>,
                               text_items: &Vec<TextItem>,
                               mut f: F) {
        // get display dimensions
        let (width, height) = self.display.get_framebuffer_dimensions();

        // draw parameters
        let params = DrawParameters {
            depth: Depth {
                test: IfLess,
                write: true,
                clamp: DepthClamp::Clamp,
                .. Default::default()
            },
            blend: Blend::alpha_blending(),
            backface_culling: BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        // uniforms passed to the shaders
        let projection_matrix = build_persp_proj_mat(60f32, width as f32/height as f32, 0.01f32, 1000f32);
        let modelview_matrix = build_fp_view_matrix(cam_state);
        let cam_pos = cam_state.cam_pos;
        let time = (time::precise_time_s() - self.start_time) as f32;

        // calc frustum places for culling
        let combo_matrix = mul_mat4(projection_matrix, modelview_matrix);
        let frustum_planes = Renderer::get_frustum_planes(combo_matrix);

        // drawing a frame
        let mut target = self.display.draw();
        let mut render_count = 0usize;

        render_post(&self.post_effect,
                    &self.shaders.post_shaders.get(self.post_effect.current_shader).unwrap(),
                    &mut target,
                    |target| {
                        // clear the colour and depth buffers
                        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);


                        // drawing the render items (with more than one instance)
                        for item in render_items.iter().filter(|r| r.active && r.instance_transforms.len() > 0) {
                            // building the vertex and index buffers
                            let vertex_buffer = VertexBuffer::new(&self.display, &item.vertices).unwrap();

                            // add positions for instances
                            let per_instance = {
                                let data = item.instance_transforms.iter().filter(|t| {
                                    t.active &&
                                        Renderer::frustrum_test(t.pos,
                                                                t.scale.0.max(t.scale.1.max(t.scale.2)) * 2.5f32,
                                                                frustum_planes.clone())
                                }).map(|t| {
                                    Attr {
                                        world_position: t.pos,
                                        world_rotation: t.rot,
                                        world_scale: t.scale
                                    }
                                }).collect::<Vec<_>>();

                                // if there are no active transforms skip ri
                                if data.len() <= 0 {
                                    continue;
                                }

                                // add instances to render_count
                                render_count += data.len();

                                VertexBuffer::dynamic(&self.display, &data).unwrap()
                            };

                            let tex_name = item.material.texture_name.clone().unwrap_or("default".to_string());
                            let normal_tex_name = item.material.normal_texture_name
                                .clone()
                                .unwrap_or("default_normal".to_string());

                            let dir_lights = self.lighting.directional_tex.borrow();

                            let uniforms = uniform! {
                                projection_matrix: projection_matrix,
                                modelview_matrix: modelview_matrix,
                                cam_pos: cam_pos,
                                time: time,
                                tex: self.shaders.textures.get(tex_name.as_str()).unwrap(),
                                normal_tex: self.shaders.textures.get(normal_tex_name.as_str()).unwrap(),
                                dir_lights: &*dir_lights,
                            };

                            target.draw((&vertex_buffer, per_instance.per_instance().unwrap()),
                            &NoIndices(PrimitiveType::Patches { vertices_per_patch: 3 }),
                            &self.shaders.shaders.get(item.material.shader_name.as_str()).unwrap(),
                            &uniforms,
                            &params).unwrap();
                        }
                    });

        self.render_count = render_count;

        // drawing the text items
        for text_item in text_items.iter().filter(|r| r.active) {
            // create the matrix for the text
            let matrix = [[0.02 * text_item.scale.0, 0.0, 0.0, 0.0],
            [0.0, 0.02 * text_item.scale.1 * (width as f32) / (height as f32), 0.0, 0.0],
            [0.0, 0.0, 0.02 * text_item.scale.2, 0.0],
            [text_item.pos.0, text_item.pos.1, text_item.pos.2, 1.0f32]];

            // create TextDisplay for item, TODO change this to not be done every frame
            let text = TextDisplay::new(&self.text_system, &self.default_font,
                                        text_item.text.as_str());

            // draw the text
            let _ = glium_text::draw(&text,
                                     &self.text_system,
                                     &mut target,
                                     matrix,
                                     text_item.color);
        }

        // imgui elements
        let ui = self.imgui.frame((width, height), (width, height), 0.1);
        f(&ui);
        self.imgui_rend.render(&mut target, ui).unwrap();

        match target.finish() {
            Ok(_) => { self.fps = self.fps_counter.tick() as f32; },
            Err(e) => println!("{:?}", e),
        };
    }

    /// Saves out a screenshot from in-game
    pub fn save_screenshot(&self) {
        // reading the front buffer into an image
        let image: RawImage2d<u8> = self.display.read_front_buffer();

        thread::spawn(move || {
            let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
            let image = image::DynamicImage::ImageRgba8(image).flipv();
            let mut output = File::create(&Path::new(format!("./screenshot_{}.png",
                                                             time::precise_time_s()).as_str())).unwrap();
            image.save(&mut output, image::ImageFormat::PNG).unwrap();
        });
    }

    /// When called with the same path adds a frame to a gif at the path
    pub fn save_add_to_gif(&mut self, path:&'static str) {
        // reading the front buffer into a gif frame
        let image: RawImage2d<u8> = self.display.read_front_buffer();

        let (w, h) = (image.width, image.height);
        let image = image::ImageBuffer::from_raw(w, h, image.data.into_owned()).unwrap();
        let mut image = image::DynamicImage::ImageRgba8(image).flipv();
        let image = image.as_mut_rgba8().unwrap();
        let mut image = image.clone().into_raw();
        let frame = gif::Frame::from_rgba(w as u16, h as u16, image.as_mut_slice());

        // if there is no encoder present create one
        let new_file = {
            match self.gif_info.as_ref() {
                Some(gi_ref) => gi_ref.path != path,
                None => false,
            }
        };
        if self.gif_info.is_none() || new_file {
            let output = OpenOptions::new().write(true).create(true).open(path).unwrap();
            let mut encoder = gif::Encoder::new(output, w as u16, h as u16, &[]).unwrap();
            encoder.set(gif::Repeat::Infinite).unwrap();

            let info = GifInfo {
                encoder: encoder,
                path: path,
            };

            self.gif_info = Some(info);
        }
        // Write frame to file
        if let Some(ref mut info) = self.gif_info {
            info.encoder.write_frame(&frame).unwrap();
        }
    }
}
