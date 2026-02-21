mod connection;

use cursive::{
    self,
    align::HAlign,
    traits::*,
    view::Margins,
    views::*,
    Cursive,
    CursiveExt,
};

use hackrschat_common::protocol::{Request, Response};
use connection::ServerConnection;

const T_HEIGHT: usize = 20;
const T_WIDTH: usize = 80;

const LOGO_HEIGHT: usize = 11;
const LOGO_WIDTH: usize = 64;

const ENTRY_WIDTH: usize = 15;

fn main() {
    let conn = match ServerConnection::connect("localhost:8080") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            std::process::exit(1);
        }
    };

    cursive::logger::init();

    let mut siv = Cursive::default();
    siv.set_user_data(conn);

    let logo: &str = include_str!("../assets/logo_full.txt");

    let view = ResizedView::with_fixed_size(
        (LOGO_WIDTH, LOGO_HEIGHT),
        Dialog::text(logo).button("GO", login_menu),
    );

    siv.add_layer(view);

    siv.run();
}

fn send_request(s: &mut Cursive, req: &Request) -> Option<Response> {
    let result = s.with_user_data(|conn: &mut ServerConnection| conn.send(req));
    match result {
        Some(Ok(resp)) => Some(resp),
        _ => {
            s.add_layer(Dialog::info("Connection to server lost."));
            None
        }
    }
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

    s.add_layer(register);
}

fn validate_register_fields(username: &str, password: &str, password_confirm: &str) -> Result<(), &'static str> {
    if username.is_empty() {
        Err("Invalid username.")
    } else if password.is_empty() {
        Err("Invalid password.")
    } else if password != password_confirm {
        Err("Passwords do not match.")
    } else {
        Ok(())
    }
}

fn submit_register_with_arg(s: &mut Cursive, _: &str) {
    submit_register(s);
}

fn submit_register(s: &mut Cursive) {
    let username = s.call_on_name("username", |v: &mut EditView| v.get_content().to_string())
        .unwrap_or_default();
    let password = s.call_on_name("password", |v: &mut EditView| v.get_content().to_string())
        .unwrap_or_default();
    let pwd_confirm = s.call_on_name("password_confirm", |v: &mut EditView| v.get_content().to_string())
        .unwrap_or_default();

    if let Err(msg) = validate_register_fields(&username, &password, &pwd_confirm) {
        s.add_layer(Dialog::info(msg));
        return;
    }

    let resp = send_request(s, &Request::CheckUser { username: username.clone() });
    match resp {
        Some(Response::UserExists { exists: true }) => {
            s.add_layer(Dialog::info("Username is taken."));
            return;
        }
        Some(Response::UserExists { exists: false }) => {
            // Username available — proceed with registration stub
            let resp = send_request(s, &Request::Register { username, password });
            match resp {
                Some(Response::RegisterSuccess) => {
                    main_menu(s);
                    return;
                }
                Some(Response::Error { message, .. }) => {
                    s.add_layer(Dialog::info(message));
                    return;
                }
                _ => return,
            }
        }
        Some(Response::Error { message, .. }) => {
            s.add_layer(Dialog::info(message));
            return;
        }
        _ => return,
    }
}

fn check_login(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

fn submit_login_with_arg(s: &mut Cursive, _: &str) {
    submit_login(s);
}

fn submit_login(s: &mut Cursive) {
    let username = s.call_on_name("username", |v: &mut EditView| v.get_content().to_string())
        .unwrap_or_default();
    let password = s.call_on_name("password", |v: &mut EditView| v.get_content().to_string())
        .unwrap_or_default();

    if !check_login(&username, &password) {
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

    let users = match send_request(s, &Request::GetUsers) {
        Some(Response::UserList(list)) => list
            .into_iter()
            .map(|u| (u.username.clone(), u.display_info()))
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };

    let select = SelectView::new()
        .with_all(users)
        .on_submit(|s, item: &String| chat(s, item))
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
    if message.starts_with('!') {
        terminal_command(s, &message[1..]);
        return;
    }

    s.call_on_name("input", |v: &mut EditView| {
        v.set_content("");
    });

    s.call_on_name("output", |v: &mut TextView| {
        if !message.is_empty() {
            v.append(format!("{}\n", message));
        }
    });
}

fn terminal_command(s: &mut Cursive, command: &str) {
    let output: String = match command {
        "h" | "help" => "Available commands: [h]elp, [q]uit, [l]ist, [cl]ear, join, create".to_string(),
        "q" | "quit" => {
            s.quit();
            String::new()
        }
        "l" | "list" => {
            match s.call_on_name("chats", |v: &mut SelectView<String>| {
                v.iter()
                    .map(|(s, _)| String::from(s))
                    .collect::<Vec<String>>()
            }) {
                Some(c) => format!(
                    "Available chats: {}",
                    c.iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<&str>>()
                        .join(", ")
                ),
                None => "No chats available.".to_string(),
            }
        }
        "cl" | "clear" => {
            let content = match s.call_on_name("output", |v: &mut TextView| {
                v.get_content().to_owned().into_source()
            }) {
                Some(c) => format!("{}\n", c.lines().next().unwrap()),
                None => String::new(),
            };
            s.call_on_name("output", |v: &mut TextView| {
                v.set_content(content);
            });
            String::new()
        }
        "join" => "Please enter a chat name to join.".to_string(),
        cmd if cmd.starts_with("join ") => {
            let param = &cmd[5..];
            if param.is_empty() {
                "Invalid chat name.".to_string()
            } else {
                let chat_name = match s.call_on_name("chats", |v: &mut SelectView<String>| {
                    match v.iter().find(move |(s, _)| *s == param) {
                        Some((s, d)) => (String::from(s), d.clone()),
                        None => (String::new(), String::new()),
                    }
                }) {
                    Some((_, d)) => d,
                    None => String::new(),
                };

                if chat_name.is_empty() {
                    format!("Chat '{}' not found.", param)
                } else {
                    chat(s, chat_name.as_str());
                    String::new()
                }
            }
        }
        "create" => "Creating chat...".to_string(),
        _ => "Unknown command. Type 'help' for a list of commands.".to_string(),
    };

    s.call_on_name("input", |v: &mut EditView| {
        v.set_content("");
    });

    s.call_on_name("output", |v: &mut TextView| {
        if !output.is_empty() {
            v.append(format!("$ {}\n{}\n", command, output));
        }
    });
}
