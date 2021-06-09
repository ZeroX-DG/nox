mod cli;

use futures::executor::block_on;
use image::{ImageBuffer, Rgba};
use simplelog::*;
use std::io::Read;

fn main() {
    let config = ConfigBuilder::new()
        .add_filter_ignore_str("wgpu")
        .add_filter_ignore_str("gfx_backend_vulkan")
        .set_target_level(LevelFilter::Info)
        .build();
    TermLogger::init(
        LevelFilter::Debug,
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();
    block_on(async_main());
}

fn read_file(path: String) -> String {
    let mut file = std::fs::File::open(path).expect("Unable to open file");
    let mut result = String::new();

    file.read_to_string(&mut result)
        .expect("Unable to read file!");

    return result;
}

async fn async_main() {
    let action = cli::get_action(cli::accept_cli());

    match action {
        cli::Action::RenderOnce(params) => {
            let html_code = read_file(params.html_path);
            let viewport = params.viewport_size;
            let output_path = params.output_path;

            let render_output = render::render_once(html_code, viewport).await;

            let (width, height) = viewport;

            if let Some(bitmap) = render_output {
                let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, bitmap).unwrap();
                buffer.save(output_path).unwrap();
            }
        }
    }
}
