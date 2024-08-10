// use std::thread;
// use std::time::{Instant, Duration};
// use cursive::logger::log;
// use cursive::style::gradient::Linear;
use cursive::Cursive;
use cursive::traits::*;
use cursive::view::Margins;
use cursive::views::*;
// use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, ResizedView, TextView};
// use cursive_async_view::{AsyncState, AsyncView};
// mod server;

fn main() {
    let mut siv = cursive::default();

    let view = ResizedView::with_fixed_size((80, 15), 
        Dialog::text("Welcome!")
            .button("GO", login_menu));
    
    siv.add_layer(view);   

    siv.run();
}

fn login_menu(s: &mut Cursive) {    
    // let buttons = LinearLayout::horizontal()
    //     .child(Button::new("Login", login))
    //     .child(DummyView)
    //     .child(Button::new("Register", register));
    
    s.pop_layer();
    s.add_layer(Dialog::text("Please login or register.")
        .title("Welcome")
        .button("Login", login)
        .button("Register", register));
}

fn login(s: &mut Cursive) { 
    s.pop_layer();

    let labels = LinearLayout::vertical()
        .child(TextView::new("Username: "))
        .child(TextView::new("Password: "));

    let entries = LinearLayout::vertical()
        .child(EditView::new()
            .filler(" ")
            // .on_submit(submit)
            .with_name("username")
            .fixed_width(15))
        .child(EditView::new()
            .secret()
            .filler(" ")
            // .on_submit(submit)
            .with_name("password")
            .fixed_width(15));

    let login_view = LinearLayout::horizontal()
        .child(labels)
        .child(entries);

    // let buttons = LinearLayout::horizontal()
    //     .child(Button::new("Back", login_menu))
    //     .child(Button::new("Submit", Cursive::quit))
    //     .fixed_width(20);

    s.add_layer(Dialog::around(login_view)
        .title("Login")
        .padding(Margins::lr(3, 3))
        .button("Back", login_menu)
        .button("Submit", submit));
}

fn register(s: &mut Cursive) {
    s.pop_layer();

}

fn check_login(username: &str, password: &str) -> bool {
    // let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    // stream.write(format!("{}:{}",
    //     username, password).as_bytes()).unwrap();
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    // let response = String::from_utf8_lossy(&buffer[..]);
    // if response == "true" {
    //     return true;
    // }
    // false
    true
}

fn submit(s: &mut Cursive) {
    let username = 
        match s.call_on_name("username", |v: &mut EditView| { v.get_content().to_string() }) {
            Some(s) => s,
            None => String::from("")
        };
    let password = 
        match s.call_on_name("password", |v: &mut EditView| { v.get_content().to_string() }) {
            Some(s) => s,
            None => String::from("")
        };
    
    if check_login(&username, &password) == false {
        s.add_layer(Dialog::info("Invalid username or password."));
        return;
    }
        
    s.pop_layer();
    s.add_layer(TextView::new(format!("{} - {}", username, password)));
    // s.quit();
}