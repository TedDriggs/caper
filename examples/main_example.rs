extern crate clock_ticks;

#[macro_use]
extern crate caper;

use std::thread;
use caper::renderer::{ Renderer, RenderItem, CamState, FIXED_TIME_STAMP };
use caper::utils::load_wavefront;
use caper::input::Input;
use caper::shader::Shaders;

fn main() {
    // load the models in to vec<Vertex>
    // for efficiency all the verts with the same shader should be one RenderItem
    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_index: 0,
            instance_positions: vec![
                (0.0, (0.0 as f32).sin(), 0.0),
                (0.0f32.sin(), 0.0, 0.0f32.cos())
            ]
        },
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/floor.obj")),
            shader_index: 1,
            instance_positions: vec![
                (0.0, 0.0, 0.0),
                (10.0, 0.0, 0.0)
            ]
        }
    ];

    game_loop! {
        render_items,

        let update_time = clock_ticks::precise_time_ns() as f32;
        // update some items
        render_items[0].instance_positions[0] = 
            (0.0, (update_time / 30.0).sin(), 0.0);
        render_items[0].instance_positions[1] = 
            ((update_time / 40.0).sin() * 3.0, 0.0, (update_time / 40.0).cos() * 3.0);
    }

}
