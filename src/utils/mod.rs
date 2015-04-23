
extern crate genmesh;
extern crate clock_ticks;
extern crate obj;

use std::thread;
use std::f32::consts::PI;
use std::ops::{Add, Mul};
use std::num::Zero;

pub enum Action {
    Stop,
    Continue,
}


pub const FIXED_TIME_STAMP: u64 = 16666667;

pub fn start_loop<F>(mut callback: F, update: fn()) where F: FnMut() -> Action {
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;
            //call the update function
            update();
        }

        thread::sleep_ms(((FIXED_TIME_STAMP - accumulator) / 1000000) as u32);
    }
}



#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_wavefront( data: &[u8]) -> Vec<Vertex> {

    implement_vertex!(Vertex, position, normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::Obj::load(&mut data);

    let mut vertex_data = Vec::new();

    for shape in data.object_iter().next().unwrap().group_iter().flat_map(|g| g.indices().iter()) {
        match shape {
            &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
                for v in [v1, v2, v3].iter() {
                    let position = data.position()[v.0];
                    let texture = v.1.map(|index| data.texture()[index]);
                    let normal = v.2.map(|index| data.normal()[index]);

                    let texture = texture.unwrap_or([0.0, 0.0]);
                    let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                    vertex_data.push(Vertex {
                        position: position,
                        normal: normal,
                        texture: texture,
                    })
                }
            },
            _ => unimplemented!()
        }
    }

    //glium::vertex::VertexBuffer::new(display, vertex_data).into_vertex_buffer_any()
    vertex_data
}


pub fn build_persp_proj_mat(fov:f32,aspect:f32,znear:f32,zfar:f32) -> [[f32; 4]; 4] {
    let ymax = znear * (fov * (PI/360.0)).tan();
    let ymin = -ymax;
    let xmax = ymax * aspect;
    let xmin = ymin * aspect;

    let width = xmax - xmin;
    let height = ymax - ymin;

    let depth = zfar - znear;
    let q = -(zfar + znear) / depth;
    let qn = -2.0 * (zfar * znear) / depth;

    let w = 2.0 * znear / width;
    let h = 2.0 * znear / height;
    
    let mut m:[[f32; 4]; 4] = [[0.0f32; 4]; 4];
    m[0]  = [w, 0.0f32, 0.0f32, 0.0f32];
    m[1]  = [0.0f32, h, 0.0f32, 0.0f32];
    m[2]  = [0.0f32, 0.0f32, q, -1.0f32];
    m[3] = [0.0f32, 0.0f32, qn, 0.0f32];

    return m;
}

pub fn dotp<T>(this: &[T], other: &[T]) -> T where T:Add<T, Output=T> + Mul<T, Output=T> + Zero + Copy {
    assert!(this.len() == other.len(), "The dimensions must be equal");

    let zero : T = Zero::zero();
    this.iter().zip(other.iter())
             .map(|(&a, &b)| a * b)
             .fold(zero, |sum, n| sum + n)
}