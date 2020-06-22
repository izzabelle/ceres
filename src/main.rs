use offbrand::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "CERES-16", about = "16bit fantasy console")]
struct Opt {
    /// times the screen should be scaled by
    #[structopt(short = "s", long = "scale", default_value = "4")]
    _scale_factor: usize,
}

fn wrapper() -> Result<()> {
    // load options
    let _opt = Opt::from_args();

    // initialize the system
    let _sys = ceres_sys::System::init();

    let data = std::fs::read_to_string("test.asm").unwrap();
    let mut assembler = ceres_asm::Assembler::new(&data);
    let asm = assembler.assemble().unwrap();
    asm.0.iter().for_each(|instruction| println!("0b{:032b}", instruction));
    asm.write_to_file(None).unwrap();

    /*
    // create a new graphics context
    let mut ctx = offbrand::Context::new(
        ceres_sys::SCREEN_WIDTH,
        ceres_sys::SCREEN_HEIGHT,
        "CERES-16".to_owned(),
        Some(opt.scale_factor),
    )?;

    // loop while the context window is open
    while ctx.is_open() {
        // clear the context
        ctx.clear(None);

        // print a pixel for ever word in the video memory buffer
        for x in 0..ceres_sys::SCREEN_WIDTH {
            for y in 0..ceres_sys::SCREEN_HEIGHT {
                let color =
                    color_from_u16(sys.video_memory[(y * ceres_sys::SCREEN_WIDTH + x) as u16]);
                ctx.insert_pixel(x, y, color);
            }
        }

        // render the context
        ctx.present()?;
    }
    */

    Ok(())
}

fn main() {
    if let Err(err) = wrapper() {
        eprintln!("error: {:?}", err);
    }
}
