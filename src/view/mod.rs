//! Types that provide the graphical user interface (GUI)

pub mod component;
pub mod section;
pub mod screen;

use vizia::prelude::*;

use crate::{model::ImpulseCore, view::screen::welcome::WelcomeScreen};


/// Represents the GUI application
/// 
/// This is a layer of abstraction over the internal GUI [`Application`] so that more code is 
/// handled here instead of in src/main.rs (which I prefer to keep as simple as possible).
pub struct GuiApplication {
    gui: Application,
    impulse_core: ImpulseCore
}

impl GuiApplication {
    // TODO: On close in debug mode, the two databases should be deleted
    pub fn new(impulse_core: ImpulseCore) -> Self {
        let gui = Application::new(|cx| {
            // Panic if the CSS cannot be loaded as it's considered unrecoverable here
            cx.add_stylesheet(include_style!("embed/css/style.css"))
                .expect("Failed to load the stylesheet");

            WelcomeScreen::new(cx);
        })
            .title("Impulse PHM")
            .inner_size((300, 300));

        Self {
            gui: gui,
            impulse_core
        }
    }

    /// Consume the instance and run the GUI application
    /// 
    /// Consuming the instance matches the behavior of the internal [`Application`].
    /// 
    /// # Panics
    /// Panics if the GUI fails to run
    pub fn run(self) {
        if let Err(e) = self.gui.run() {
            log::error!("Failed to run the GUI application: {}", e);
            // Any error here represents an unrecoverable error, and it will have to be resolved 
            // outside of this project.
            panic!("Failed to run the GUI application: {}", e);
        }
    }
}