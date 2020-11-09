mod texture;
mod window;
use futures::executor::block_on;

fn main() {
    env_logger::init();

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const NUMS_PER_PIXEL: u32 = 4; // rgba

    let image_buffer = vec![128u8; (WIDTH*HEIGHT*NUMS_PER_PIXEL) as usize];

    block_on(window::render_texture(image_buffer, WIDTH, HEIGHT));
}
