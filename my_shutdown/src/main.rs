//extern crate gtk;
//
//use gtk::prelude::*;
//
//use gtk::{Button, Window, WindowType};
//use core::borrow::BorrowMut;
//
//mod CountDown;
//
//
//fn main() {
//     if gtk::init().is_err() {
//         println!("Failed to initialize GTK.");
//         return;
//     }
//
//     let window = Window::new(WindowType::Toplevel);
//     window.set_title("First GTK+ Program");
//     window.set_default_size(350, 70);
//
//     let outer_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
//
//
//     let tip = gtk::Label::new("打卡了吗？");
//     outer_box.pack_start(&tip, true, false, 40);
//
//     let mut btn_cancel = Button::new_with_label("(5秒后关机)取消");
//     outer_box.pack_start(&btn_cancel, true, true, 0);
//
//     window.add(&outer_box);
//
//     window.show_all();
//
//     window.connect_delete_event(|_, _| {
//         gtk::main_quit();
//         Inhibit(false)
//     });
//
//    CountDown::start(5, move |x| {
//        println!("{}", x);
//        btn_cancel.set_label(&x.to_string());
//    });
//
//     gtk::main();
//
//
//
//}
//


//! # Clock Sample
//!
//! This sample demonstrates how to use gtk::timeout_add_seconds to run
//! a periodic task, implementing a clock in this example.

extern crate gio;
extern crate gtk;
extern crate chrono;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use chrono::Local;
use gtk::Button;


fn current_time() -> String {
    return format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
}


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Clock");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(260, 40);

    let mut countdown = 5u32;
//    let time = current_time();
//    let time = countdown.to_string();
//    let label = gtk::Label::new(None);
//    label.set_text(&time);
//
//    window.add(&label);

    let outer_box = gtk::Box::new(gtk::Orientation::Vertical, 0);


    let tip = gtk::Label::new("打卡了吗？");
    outer_box.pack_start(&tip, true, false, 40);

    let btn_cancel = Button::new_with_label("取消");
    outer_box.pack_start(&btn_cancel, true, true, 0);

    btn_cancel.connect_clicked(|_| {
        println!("Clicked!");
    });

    window.add(&outer_box);

    window.show_all();

    // we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {

//        let time = current_time();
//        label.set_text(&time);
        countdown -= 1;
//        label.set_text(&countdown.to_string());

        let btn_text = format!("({}秒后关机) 取消", countdown);

        btn_cancel.set_label(&btn_text);

        // we could return gtk::Continue(false) to stop our clock after this tick
        gtk::Continue(true)
    };

    // executes the closure once every second
    gtk::timeout_add_seconds(1, tick);
}

fn main() {
    let application = gtk::Application::new("com.github.gtk-rs.examples.clock",
                                            Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
