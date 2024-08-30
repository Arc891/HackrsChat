use std::{
    io::{Read, Write},
    net::TcpStream,
    // time::{Instant, Duration};
};

use cursive::{
    self, 
    align::HAlign, 
    traits::*, 
    view::Margins, 
    views::*, 
    Cursive, 
    CursiveExt
    // theme::ColorStyle;
    // theme::PaletteStyle;
    // logger::log;
    // style::gradient::Linear;
};

use serde::{Serialize, Deserialize};

// use tokio::{
//   io::{
//     AsyncReadExt,
//     AsyncWriteExt,
//   },
//   // net::TcpStream,
//   // time::Instant,
// };

// use cursive_async_view::{
//   AsyncState, AsyncView,
//   AsyncProgressView, AsyncProgressState
// };

mod user;
use user::User;

const T_HEIGHT: usize = 20;
const T_WIDTH: usize = 80;

const LOGO_HEIGHT: usize = 11;
const LOGO_WIDTH: usize = 64;

const ENTRY_WIDTH: usize = 15;

#[tokio::main]
async fn main() {
    cursive::logger::init();

    let mut siv = Cursive::default();

    // Read logo from assets/logo_full.txt
    let logo: &str = include_str!("../assets/logo_full.txt");

    let view = ResizedView::with_fixed_size(
        (LOGO_WIDTH, LOGO_HEIGHT),
        Dialog::text(logo).button("GO", login_menu),
    );

    siv.add_layer(view);

    siv.run();
}

fn login_menu(s: &mut Cursive) {
    s.pop_layer();

    let logo: &str = include_str!("../assets/logo_full.txt");

    let logo_text = TextView::new(logo).h_align(HAlign::Center).scrollable();

    let info_text = TextView::new("Please login or register.")
        .h_align(HAlign::Center)
        .scrollable();

    let logo_view = Dialog::around(
        LinearLayout::vertical()
            .child(logo_text)
            .child(DummyView)
            .child(info_text),
    )
    .title("Welcome")
    .padding(Margins::lr(3, 3))
    .button("Login", login)
    .button("Register", register)
    .button("Quit", Cursive::quit);

    s.add_layer(logo_view);

    // s.add_layer(Dialog::text("Please login or register.")
    //   .title("Welcome")
    //   .button("Login", login)
    //   .button("Register", register));
}

fn login(s: &mut Cursive) {
    s.pop_layer();

    let labels = LinearLayout::vertical()
        .child(TextView::new("Username: "))
        .child(TextView::new("Password: "));

    let entries = LinearLayout::vertical()
        .child(
            EditView::new()
                .filler(" ")
                .on_submit(submit_login_with_arg)
                .with_name("username")
                .fixed_width(ENTRY_WIDTH),
        )
        .child(
            EditView::new()
                .secret()
                .filler(" ")
                .on_submit(submit_login_with_arg)
                .with_name("password")
                .fixed_width(ENTRY_WIDTH),
        );

    let login_view = LinearLayout::horizontal()
        .child(labels)
        .child(entries);

    // let buttons = LinearLayout::horizontal()
    //   .child(Button::new("Back", login_menu))
    //   .child(Button::new("Submit", Cursive::quit))
    //   .fixed_width(20);

    s.add_layer(
        Dialog::around(login_view)
            .title("Login")
            .padding(Margins::lr(3, 3))
            .button("Back", login_menu)
            .button("Register", register)
            .button("Submit", submit_login),
    );
}

fn register(s: &mut Cursive) {
    s.pop_layer();

    let labels = LinearLayout::vertical()
        .child(TextView::new("Username: "))
        .child(TextView::new("Password: "))
        .child(TextView::new("Confirm password: "));

    let entries = LinearLayout::vertical()
        .child(
            EditView::new()
                .filler(" ")
                .on_submit(submit_register_with_arg)
                .with_name("username")
                .fixed_width(ENTRY_WIDTH),
        )
        .child(
            EditView::new()
                .secret()
                .filler(" ")
                .on_submit(submit_register_with_arg)
                .with_name("password")
                .fixed_width(ENTRY_WIDTH),
        )
        .child(
            EditView::new()
                .secret()
                .filler(" ")
                .on_submit(submit_register_with_arg)
                .with_name("password_confirm")
                .fixed_width(ENTRY_WIDTH),
        );

    let registering_view = LinearLayout::horizontal().child(labels).child(entries);

    let register = Dialog::around(registering_view)
        .title("Register")
        .padding(Margins::lr(3, 3))
        .button("Back", login_menu)
        .button("Login", login)
        .button("Submit", submit_register);

    // let async_register: AsyncView<DummyView> = AsyncView::new(s, move || {
    //   let instant = Instant::now();
    //   if instant.elapsed() > Duration::from_secs(1) {
    //     return AsyncState::Available(DummyView);
    //   } else {
    //     return AsyncState::Pending;
    //   }
    // });

    s.add_layer(register);
}

fn check_register(username: &str, password: &str, password_confirm: &str) -> u32 {
    let username_taken = send_db_command(&format!("check_user {}\n", username))
        .trim()
        .to_string()
        .eq("true");
    if username.len() < 1 {
        1
    } else if username_taken {
        2
    } else if password != password_confirm {
        3
    } else if password.len() < 1 {
        4
    } else {
        0
    }
}

fn submit_register_with_arg(s: &mut Cursive, _: &str) {
    submit_register(s);
}

fn submit_register(s: &mut Cursive) {
    let username = match s.call_on_name("username", |v: &mut EditView| v.get_content().to_string())
    {
        Some(s) => s,
        None => String::from(""),
    };
    let password = match s.call_on_name("password", |v: &mut EditView| v.get_content().to_string())
    {
        Some(s) => s,
        None => String::from(""),
    };
    let pwd_confirm = match s.call_on_name("password_confirm", |v: &mut EditView| v.get_content().to_string())
    {
        Some(s) => s,
        None => String::from(""),
    };

    let register_status = check_register(&username, &password, &pwd_confirm);
    if register_status > 0 {
        s.add_layer(Dialog::info(match register_status {
            1 => "Invalid username.",
            2 => "Username is taken.",
            3 => "Passwords do not match.",
            4 => "Invalid password.",
            0 | 5..=u32::MAX => "Unknown error.",
        }));
        return;
    }

    main_menu(s);
}

fn check_login(username: &str, password: &str) -> bool {
    return username != "" && password != "";
}

fn submit_login_with_arg(s: &mut Cursive, _: &str) {
    submit_login(s);
}

fn submit_login(s: &mut Cursive) {
    let username = match s.call_on_name("username", |v: &mut EditView| v.get_content().to_string())
    {
        Some(s) => s,
        None => String::from(""),
    };
    let password = match s.call_on_name("password", |v: &mut EditView| v.get_content().to_string())
    {
        Some(s) => s,
        None => String::from(""),
    };

    if check_login(&username, &password) == false {
        s.add_layer(Dialog::info("Invalid username or password."));
        return;
    }

    main_menu(s);
}

fn main_menu(s: &mut Cursive) {
    s.pop_layer();
    let terminal_input = EditView::new()
        .filler(" ")
        .on_submit(terminal_command)
        .with_name("input")
        .fixed_width(T_WIDTH);

    let terminal_prefix = TextView::new("$ ").fixed_width(2);

    let terminal = LinearLayout::vertical()
        .child(
            TextView::new("-- Terminal --\n")
                .with_name("output")
                .scrollable()
                .fixed_height(T_HEIGHT),
        )
        .child(
            LinearLayout::horizontal()
                .child(terminal_prefix)
                .child(terminal_input),
        );
    
        
    let users = db_command_with_ret_vec("get_users\n").into_iter().map(|u| (u.get_username(), u.display_info())).collect::<Vec<(String, String)>>();
        
    // let _chats = vec![
    //     ("Chat 1".to_string(), "Chat 1 description".to_string()),
    //     ("Chat 2".to_string(), "Chat 2 description".to_string()),
    //     ("Chat 3".to_string(), "Chat 3 description".to_string()),
    // ];
    let select = SelectView::new()
        .with_all(std::iter::repeat(users).take(1).flatten())
        //.on_submit(|s, item: &str| s.add_layer(Dialog::info(format!("Selected: {}", item))))
        .on_submit(|s, item: &String| chat(s, &item))
        .with_name("chats");

    let horizontal_line = std::iter::repeat(String::from("|\n"))
        .take(T_HEIGHT + 1)
        .collect::<String>();

    let main_menu = LinearLayout::horizontal()
        .child(select)
        .child(DummyView)
        .child(TextView::new(horizontal_line))
        .child(terminal);

    s.add_layer(
        Dialog::around(main_menu)
            .title("Main menu")
            .button("Logout", login_menu)
            .button("Quit", Cursive::quit),
    );
}

fn chat(s: &mut Cursive, chat_title: &str) {
    s.pop_layer();
    let chat_input = EditView::new()
        .filler(" ")
        // .style(ColorStyle::tertiary())
        .on_submit(chat_message)
        .with_name("input")
        .fixed_width(T_WIDTH);

    let chat_prefix = TextView::new("> ").fixed_width(2);

    let chat = LinearLayout::vertical()
        .child(
            TextView::new(format!("-- {chat_title} --\n"))
                .with_name("output")
                .scrollable()
                .fixed_height(T_HEIGHT),
        )
        .child(
            LinearLayout::horizontal()
                .child(chat_prefix)
                .child(chat_input),
        );

    let chat_menu = LinearLayout::horizontal().child(chat);

    s.add_layer(
        Dialog::around(chat_menu)
            .title("Chat")
            .button("Back", main_menu)
            .button("Quit", Cursive::quit),
    );
}

fn chat_message(s: &mut Cursive, message: &str) {
    if message.starts_with("!") {
        terminal_command(s, &message[1..]);
        return;
    }

    s.call_on_name("input", |v: &mut EditView| {
        v.set_content("");
    });

    s.call_on_name("output", |v: &mut TextView| {
        if message != "" {
            v.append(format!("{}\n", message));
        }
    });
}

fn terminal_command(s: &mut Cursive, command: &str) {
    let output = match command {
        "h" | "help" => "Available commands: [h]elp, [q]uit, [l]ist, [cl]ear, join, create",
        "q" | "quit" => {
            s.quit();
            ""
        }
        "l" | "list" => {
            match s.call_on_name("chats", |v: &mut SelectView<&str>| {
                v.iter()
                    .map(|(s, _)| String::from(s))
                    .collect::<Vec<String>>()
            }) {
                Some(c) => &format!(
                    "Available chats: {}",
                    c.iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<&str>>()
                        .join(", ")
                ),
                None => "No chats available.",
            }
        }
        "cl" | "clear" => {
            let content = match s.call_on_name("output", |v: &mut TextView| {
                v.get_content().to_owned().into_source()
            }) {
                // Only grab the first line of the content
                Some(c) => format!("{}\n", c.lines().next().unwrap().to_string()),
                None => String::from(""),
            };
            s.call_on_name("output", |v: &mut TextView| {
                v.set_content(content);
            });
            ""
        }
        "join" => "Please enter a chat name to join.",
        cmd if cmd.starts_with("join ") => {
            let param = &cmd[5..];
            if param == "" {
                "Invalid chat name."
            } else {
                let chat_name = match s.call_on_name("chats", |v: &mut SelectView<&str>| {
                    match v.iter().find(move |(s, _)| *s == param) {
                        Some((s, d)) => (String::from(s), String::from(*d)),
                        None => ("".to_string(), "".to_string()),
                    }
                }) {
                    Some((_, d)) => String::from(d),
                    None => "".to_string(),
                };

                if chat_name == "" {
                    &format!("Chat '{}' not found.", param)
                } else {
                    chat(s, chat_name.as_str());
                    // &format!("Joining chat: {}", param)
                    ""
                }
            }
        }
        "create" => "Creating chat...",
        _ => "Unknown command. Type 'help' for a list of commands.",
    };

    s.call_on_name("input", |v: &mut EditView| {
        v.set_content("");
    });

    s.call_on_name("output", |v: &mut TextView| {
        if output != "" {
            v.append(format!("$ {}\n{}\n", command, output));
        }
    });
}

fn send_db_command(command: &str) -> String {
    let mut stream = TcpStream::connect("localhost:8080").expect("Could not connect to server.");
    let mut buffer = [0; 4096];
    let mut response = String::new();

    stream
        .write_all(command.as_bytes())
        .expect("Could not write to stream.");
    let n = stream
        .read(&mut buffer)
        .expect("Could not read from stream.");
    response
        .push_str(std::str::from_utf8(&buffer[..n]).expect("Could not convert buffer to string."));

    response
}

fn db_command_with_ret(command: &str) -> User {
    let response = send_db_command(command);
    let user: Result<User, serde_json::Error> = serde_json::from_str(&response);
    match user {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error: {}", e);
            User::new("err".to_string(), "err".to_string())
        }
    }
}

fn db_command_with_ret_vec(command: &str) -> Vec<User> {
    let response = send_db_command(command);
    let users: Result<Vec<User>, serde_json::Error> = serde_json::from_str(&response);
    match users {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error: {}", e);
            let err_user = User::new("err".to_string(), "err".to_string());
            // vec![err_user.clone(), err_user.clone(), err_user.clone(), err_user.clone()]
            std::iter::repeat(err_user).take(4).collect::<Vec<User>>()
        }
    }
}