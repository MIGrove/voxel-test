use macroquad::prelude::*;
use color_eyre::eyre::Result;

fn window_conf() -> Conf {
    Conf {
        window_title: "Voxel Test".to_owned(),
        window_height: 250,
        window_width: 250,
        window_resizable: false,
        ..Default::default()
    }
}

fn error_conf() -> Result<()> {
    color_eyre::install()
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    error_conf()?;
    
    next_frame().await;
    
    loop {
        clear_background(BEIGE);

        for x in (0u8..150u8).step_by(10) {
            for y in (0u8..150u8).step_by(10) {
                let random_colour: u8 = rand::gen_range(0, 255);
                draw_rectangle_ex(x as f32, y as f32, 10.0, 10.0, DrawRectangleParams {
                    color: Color::from_rgba(random_colour, random_colour, random_colour, 255), ..Default::default()
                });
            }
        }

        draw_text("Hello world?", 25.0, 120.0, 20.0, GRAY);
        
        next_frame().await
    }
}

