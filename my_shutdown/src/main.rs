extern crate gtk;

use gtk::prelude::*;

use gtk::{Button, Window, WindowType};
use core::borrow::BorrowMut;

mod CountDown;


fn main() {
     if gtk::init().is_err() {
         println!("Failed to initialize GTK.");
         return;
     }

     let window = Window::new(WindowType::Toplevel);
     window.set_title("First GTK+ Program");
     window.set_default_size(350, 70);

     let outer_box = gtk::Box::new(gtk::Orientation::Vertical, 0);


     let tip = gtk::Label::new("打卡了吗？");
     outer_box.pack_start(&tip, true, false, 40);

     let mut btn_cancel = Button::new_with_label("(5秒后关机)取消");
     outer_box.pack_start(&btn_cancel, true, true, 0);

     window.add(&outer_box);

     window.show_all();

     window.connect_delete_event(|_, _| {
         gtk::main_quit();
         Inhibit(false)
     });


     gtk::main();

    CountDown::start(5, move |x| {
        println!("{}", x);
        btn_cancel.borrow_mut()
    });

}

