/*!

Minimalist game framework.
Currently has systems for:
- Rendering ([glium](https://github.com/tomaka/glium))
- Input ([winit](https://github.com/tomaka/winit)
    via [volition](https://github.com/shockham/volition))
- Physics ([nphysics](https://github.com/sebcrozet/nphysics))
- Audio ([rodio](https://github.com/tomaka/rodio))

[Example](https://github.com/shockham/caper/blob/master/examples/simple.rs) of a basis for a game:

```no_run
extern crate caper;

use caper::types::{RenderItemBuilder, TransformBuilder, DefaultTag};
use caper::game::*;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::handle_fp_inputs;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<DefaultTag>::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((-0.5, 0.0, -5.0))
                    .build()
                    .unwrap()
            ])
            .build()
            .unwrap());

    loop {
        // run the engine update
        game.update(|_:&Ui|{ });

        // update the first person inputs
        handle_fp_inputs(&mut game.input, &mut game.cams[0]);

        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
    }
}
```

*/

#![deny(missing_docs)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
pub extern crate glium;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub extern crate image;
pub extern crate imgui;
pub extern crate impose as audio;
pub extern crate nalgebra;
pub extern crate ncollide;
pub extern crate nphysics3d;
pub extern crate volition as input;

extern crate bincode;
extern crate fps_counter;
extern crate gif;
extern crate glium_text_rusttype as glium_text;
extern crate imgui_glium_renderer;
extern crate noise;
extern crate rayon;
extern crate serde;
extern crate time;

/// Module for utility functions for textures
#[macro_use]
pub mod texture;
/// A module for rendering items
pub mod renderer;
/// Utility functions and macros
pub mod utils;
/// Module for dealing with shaders
pub mod shader;
/// Module for procedurally generated meshes
pub mod mesh;
/// Rendering post processing effects
pub mod posteffect;
/// All of the caper types
pub mod types;
/// Simple collision detection
pub mod collision;
/// Module for saving and loading data
pub mod persist;
/// Module represent another way of creating a game
pub mod game;
/// Module for the lighting system
pub mod lighting;
