//! # Clock Sample
//!
//! This sample demonstrates how to use gtk::timeout_add_seconds to run
//! a periodic task, implementing a clock in this example.

extern crate gio;
extern crate glib;
extern crate gtk;
extern crate chrono;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use chrono::Local;


fn current_time() -> String {
    return format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Clock");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(260, 40);

    let time = current_time();
    let label = gtk::Label::new(None);
    label.set_text(&time);
    label.set_margin_top(10);
    label.set_margin_bottom(10);
    label.set_margin_start(10);
    label.set_margin_end(10);

    window.add(&label);

    window.show();

    // we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        // we could return gtk::Continue(false) to stop our clock after this tick
        glib::Continue(true)
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
}

fn main() {
    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.clock"),
                                            Default::default())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
