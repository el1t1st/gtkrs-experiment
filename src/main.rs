use gtk::glib;
use gtk::prelude::*;
use gtk::Application;
use std::cell::Cell;
use std::rc::Rc;

static APP_ID: &str = "org.gtkrs-experiment";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(application: &Application) {
    let number = Rc::new(Cell::new(0));

    let number_inc = number.clone();
    let button_inc = gtk::Button::builder().label("Plus 1").build();

    let label_info = gtk::Label::builder()
        .label(format!("The number is {}", number.get()))
        .build();

    // The widgets don't have any data-state connection so you need to
    // perform all changes by hand
    let label_info_2 = label_info.clone();

    button_inc.connect_clicked(move |_| {
        number_inc.set(number_inc.get() + 1);
        // move the cloned label and set_text to update the text inside the label
        label_info_2.set_text(&number_inc.get().to_string());
        println!("The number_inc: {}", number_inc.get());
    });

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    vbox.append(&label_info);
    vbox.append(&button_inc);

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("GTK Counter Experiment")
        .child(&vbox)
        .build();

    window.present()
}
