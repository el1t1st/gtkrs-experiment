use gdk::prelude::ActionMapExt;
use gtk::gio;
use gtk::glib;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};
use gtk::traits::GtkApplicationExt;
use gtk::traits::WidgetExt;

static APP_ID: &str = "org.gtk.testing";

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .width_request(360)
        .build();

    // Add action close to window taking a parameter
    let action_close = gio::SimpleAction::new("close", None);

    let window_cloned = window.clone();
    action_close.connect_activate(move |_, _| {
        window_cloned.close();
    });
    // action_close.connect_activate(glib::clone!(@weak window => move |_, _| {
    //     window.close();
    // }));
    window.add_action(&action_close);

    // Create an EventController
    let eventctl = gtk::EventControllerKey::new();

    eventctl.connect_key_pressed(|_eventctl, keyval, keycode, state| {
        println!("{:?}", keyval.to_unicode());
        println!("{:?}", keycode);
        println!("{:?}", state);
        gtk::Inhibit(true)
    });
    // connect_key_pressed
    window.add_controller(eventctl);

    window.present();
}

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    // Set Shortcuts for Actions
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);
    app.run()
}
