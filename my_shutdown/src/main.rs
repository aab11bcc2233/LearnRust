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


extern crate gio;
extern crate gtk;
//extern crate chrono;
//extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
//use chrono::Local;
//use gtk::Button;

//use std::sync::{Mutex, Arc};
use std::process::Command;
//use std::thread;


//fn current_time() -> String {
//    return format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
//}

//fn deep_main_quit(n: usize) {
//    if n == 0 {
//        return;
//    }
//    gtk::main_quit();
//    gtk::idle_add(move || {
//        deep_main_quit(n - 1);
//        gtk::Continue(false)
//    });
//}


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("打卡了吗？");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600, 400);

    let mut countdown = 9u32;
//    let time = current_time();
//    let time = countdown.to_string();
//    let label = gtk::Label::new(None);
//    label.set_text(&time);
//
//    window.add(&label);

    let outer_box = gtk::Box::new(gtk::Orientation::Vertical, 0);


    let tip = gtk::Label::new("打卡了吗？\n打卡了吗？\n打卡了吗？");
    outer_box.pack_start(&tip, true, false, 40);

    let close_tip = gtk::Label::new("");
    outer_box.pack_start(&close_tip, true, true, 0);

//    let btn_cancel = Button::new_with_label("取消");
//    outer_box.pack_start(&btn_cancel, true, true, 0);

//    let is_cancel: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));


//    let is_cancel_btn = Arc::clone(&is_cancel);
//    btn_cancel.connect_clicked(move |_| {
//        println!("Clicked!");
////        let mut cancel = is_cancel_btn.lock().unwrap();
////        *cancel = true;
//
////        thread::spawn(|| {
////            glib::idle_add(|| {
////                println!("Quit");
////                deep_main_quit(2);
////                gtk::Continue(false)
////            });
////        });
////
////        println!("Run");
////        gtk::main();
//
//        gtk::Continue(false);
//        gtk::main_quit();
//        Inhibit(false);
//    });

    window.add(&outer_box);

    window.show_all();

    // we are using a closure to capture the label (else we could also use a normal function)
//    let is_cancel_tick = Arc::clone(&is_cancel);
    let tick = move || {
//        let is_cancel = is_cancel_tick.lock().unwrap();
//        if *is_cancel {
//            return gtk::Continue(false);
//        }

//        let time = current_time();
//        label.set_text(&time);

        if countdown == 0 {
            return gtk::Continue(false);
        }

        countdown -= 1;
//        label.set_text(&countdown.to_string());

        let btn_text = format!("{}秒后关机", countdown);
//        btn_cancel.set_label(&btn_text);
        close_tip.set_label(&btn_text);

        if countdown == 0 {
            let _output = Command::new("poweroff")
//                .args(&[])
                .output()
                .expect("failed to execute process");
            return gtk::Continue(false);
        }

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
