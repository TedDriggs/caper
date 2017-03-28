Caper
========
Small game framework using [rust](https://www.rust-lang.org/) and [glium](https://github.com/tomaka/glium).

[**Documentation**](https://shockham.github.io/caper/caper/)

[Example](https://github.com/shockham/caper/blob/master/examples/simple.rs) of a basis for a game:
```rust
#[macro_use]
extern crate caper;

use caper::types::{ RenderItem, Transform, PhysicsType };
use caper::mesh::gen_cube;

fn main() {
    // define some items to be rendered
    let mut render_items = vec![
        RenderItem {
            vertices: gen_cube(),
            shader_name: String::from("dist"),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (-0.5, 0.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        },
    ];

    // define a vector for potential text items
    let text_items = Vec::new();

    game_loop! {
        // following are identities for access to the frameworks systems
        Input => input,
        Renderer => renderer,
        CamState => cam_state,
        RenderItems => render_items,
        TextItems => text_items,
        // define a block for start
        start => {
            println!("{:?}", cam_state.cam_pos);
        },
        // define block for update
        update => {
            input.handle_fp_inputs(&mut cam_state);
        },
        // block for ui rendering
        ui => {

        }
    }
}
```

Another [example](https://github.com/shockham/caper/blob/master/examples/game.rs) using the **Game** struct:

```rust
extern crate caper;

use caper::types::{ RenderItem, Transform, PhysicsType };
use caper::game::Game;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItem {
            vertices: gen_cube(),
            shader_name: "dist".to_string(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (-0.5, 0.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        });

    loop {
        // run the engine update
        game.update(|ui:&Ui|{ });

        // update the first person inputs
        game.input.handle_fp_inputs(&mut game.cam_state);

        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
    }
}
```

Check out the [examples](https://github.com/shockham/caper/tree/master/examples) and run with:
```
cargo run --example transforms
```

[License](https://github.com/shockham/caper/blob/master/LICENSE.md)
