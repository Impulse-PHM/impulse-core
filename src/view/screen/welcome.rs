//! Logic for the initial, welcome view displayed to the end-user

use vizia::prelude::*;


/// The first view an end-user will see
pub struct WelcomeScreen {}

impl WelcomeScreen {
    pub fn new(cx: &mut Context) -> Handle<'_, Self>{
        Self {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Create New User"));
                Button::new(cx, |cx| Label::new(cx, "Open Existing User"));
            })
            .class("welcome-container");
        })
    }
}

impl View for WelcomeScreen {}