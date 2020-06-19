use offbrand::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "CERES-16", about = "16bit fantasy console")]
struct Opt {
    /// times the screen should be scaled by
    #[structopt(short = "s", long = "scale", default_value = "4")]
    scale_factor: usize,
}

fn wrapper() -> Result<()> {
    let opt = Opt::from_args();

    let mut ctx = offbrand::Context::new(
        ceres_sys::SCREEN_WIDTH,
        ceres_sys::SCREEN_HEIGHT,
        "CERES-16".to_owned(),
        Some(opt.scale_factor),
    )?;

    let mut sys = ceres_sys::System::init();

    let mut colors: Vec<u16> = Vec::new();

    for r in 0x0..0xf {
        for g in 0x0..0xf {
            for b in 0x0..0xf {
                colors.push((r << 8) | (g << 4) | b);
            }
        }
    }

    colors.iter().enumerate().for_each(|(addr, color)| {
        sys.video_memory[addr as u16] = *color;
    });

    while ctx.is_open() {
        ctx.clear(None);

        for x in 0..ceres_sys::SCREEN_WIDTH {
            for y in 0..ceres_sys::SCREEN_HEIGHT {
                let color =
                    color_from_u16(sys.video_memory[(y * ceres_sys::SCREEN_WIDTH + x) as u16]);
                ctx.insert_pixel(x, y, color);
            }
        }

        ctx.present()?;
    }

    Ok(())
}

fn color_from_u16(color: u16) -> Color {
    let r = (((color >> 8) & 0x000f) as u8) * 17;
    let g = (((color >> 4) & 0x000f) as u8) * 17;
    let b = ((color & 0x000f) as u8) * 17;
    Color { r, g, b }
}

fn main() {
    if let Err(err) = wrapper() {
        eprintln!("error: {:?}", err);
    }
}
