use mystic_light_ha::{log::start_logging, AppBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_logging();

    let app = AppBuilder::new().with_random_port(false).build()?;

    app.listen()?;

    Ok(())
}
