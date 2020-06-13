use offbrand::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "CERES-16", about = "16bit fantasy console")]
struct Opt {
    /// times the screen should be scaled by
    #[structopt(short = "s", long = "scale", default_value = "2")]
    scale_factor: usize,
}

fn wrapper() -> Result<()> {
    let opt = Opt::from_args();
    let mut ctx = offbrand::Context::new(
        ceres_sys::SCREEN_WIDTH * opt.scale_factor,
        ceres_sys::SCREEN_HEIGHT * opt.scale_factor,
        "CERES-16".to_owned(),
    )?;
    let sys = ceres_sys::System::init();
    println!("{}", sys.registers);

    while ctx.is_open() {
        ctx.clear(None);

        ctx.present()?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = wrapper() {
        eprintln!("error: {:?}", err);
    }
}
