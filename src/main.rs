use impulse_phm::{environment, model::ImpulseCore, view::GuiApplication};


fn main() {
    environment::setup_logging()
        .expect("Failed to initialize the logger");

    let impulse_core: ImpulseCore = environment::setup_environment()
        .expect("Failed to setup a new, or use an existing, environment");

    let app = GuiApplication::new(impulse_core);
    app.run();
}
