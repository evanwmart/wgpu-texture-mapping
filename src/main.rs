// main.rs
use wgpu_texture_mapping::run;

fn main() {
    pollster::block_on(run());
}