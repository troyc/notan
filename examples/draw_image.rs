use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    img: Texture,
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/ferris.png"))
        .build()
        .unwrap();
    State { img: texture,
        font: gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap(), }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.image(&state.img).position(250.0, 200.0);
    draw.text(
        &state.font,
        &format!(
            "{} -> ({:.6})",
            app.timer.fps().round(),
            app.timer.delta_f32()
        ));
    gfx.render(&draw);
}
