use cursive::Cursive;
use cursive::traits::*;
use cursive::view::Margins;
use cursive::views::*;
use cursive::align::HAlign;
// use cursive::theme::ColorStyle;
// use cursive::theme::PaletteStyle;
// use std::time::{Instant, Duration};
// use cursive::logger::log;
// use cursive::style::gradient::Linear;
// use cursive_async_view::{AsyncState, AsyncView};

fn main() {
  let mut siv = cursive::default();

  // Read logo from assets/logo_full.txt
  let logo: &str = include_str!("../assets/logo_full.txt");

  let view = ResizedView::with_fixed_size((64, 11), 
    Dialog::text(logo)
      .button("GO", login_menu));
  
  siv.add_layer(view);   

  siv.run();
}

fn login_menu(s: &mut Cursive) {  
  s.pop_layer();

  let logo: &str = include_str!("../assets/logo_full.txt");

  let logo_text = TextView::new(logo)
    .h_align(HAlign::Center)
    .scrollable();

  let info_text = TextView::new("Please login or register.")
    .h_align(HAlign::Center)
    .scrollable();

  let logo_view = Dialog::around(LinearLayout::vertical()
      .child(logo_text)
      .child(DummyView)
      .child(info_text))
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
    .child(EditView::new()
      .filler(" ")
      .on_submit(submit_login_with_arg)
      .with_name("username")
      .fixed_width(15))
    .child(EditView::new()
      .secret()
      .filler(" ")
      .on_submit(submit_login_with_arg)
      .with_name("password")
      .fixed_width(15));

  let login_view = LinearLayout::horizontal()
    .child(labels)
    .child(entries);

  // let buttons = LinearLayout::horizontal()
  //   .child(Button::new("Back", login_menu))
  //   .child(Button::new("Submit", Cursive::quit))
  //   .fixed_width(20);

  s.add_layer(Dialog::around(login_view)
    .title("Login")
    .padding(Margins::lr(3, 3))
    .button("Back", login_menu)
    .button("Register", register)
    .button("Submit", submit_login));
}


fn register(s: &mut Cursive) {
  s.pop_layer();
  
  let labels = LinearLayout::vertical()
    .child(TextView::new("Username: "))
    .child(TextView::new("Password: "))
    .child(TextView::new("Confirm password: ")); 
  
  let entries = LinearLayout::vertical()
    .child(EditView::new()
      .filler(" ")
      .on_submit(submit_register_with_arg)
      .with_name("username")
      .fixed_width(15))
    .child(EditView::new()
      .secret()
      .filler(" ")
      .on_submit(submit_register_with_arg)
      .with_name("password")
      .fixed_width(15))
    .child(EditView::new()
      .secret()
      .filler(" ")
      .on_submit(submit_register_with_arg)
      .with_name("password_confirm")
      .fixed_width(15));

    let register_view = LinearLayout::horizontal()
      .child(labels)
      .child(entries);

  s.add_layer(Dialog::around(register_view)
    .title("Register")
    .padding(Margins::lr(3, 3))
    .button("Back", login_menu)
    .button("Login", login)
    .button("Submit", submit_register));
}

fn check_register(username: &str, password: &str, password_confirm: &str) -> u32 {
  if username == "" { 1 }
  else if password == password_confirm && password != "" { 0 } 
  else { 2 }
}

fn submit_register_with_arg(s: &mut Cursive, _: &str) {
  submit_register(s);
}

fn submit_register(s: &mut Cursive) {
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
  let password_confirm = 
    match s.call_on_name("password_confirm", |v: &mut EditView| { v.get_content().to_string() }) {
      Some(s) => s,
      None => String::from("")
    };


  let register_status = check_register(&username, &password, &password_confirm);
  if register_status > 0 {
    let register_error = match register_status {
      1 => "Invalid username or already taken.",
      2 => "Passwords do not match.",
      0|3_u32..=u32::MAX => "Unknown error.",
    };
    s.add_layer(Dialog::info(register_error));
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
    
  main_menu(s);
}


fn main_menu(s: &mut Cursive) {
  s.pop_layer();
  let terminal_input = EditView::new()
    .filler(" ")
    .on_submit(terminal_command)
    .with_name("input")
    .fixed_width(80);

  let terminal_prefix = TextView::new("$ ")
    .fixed_width(2);

  let terminal = LinearLayout::vertical()
    .child(TextView::new("-- Terminal --\n")
      .with_name("output")
      .scrollable()
      .fixed_height(20))
    .child(LinearLayout::horizontal()
      .child(terminal_prefix)
      .child(terminal_input));

  let chats = vec![("Chat 1", "Chat 1 description"),
    ("Chat 2", "Chat 2 description"),
    ("Chat 3", "Chat 3 description")];
  let select = SelectView::new()
    .with_all(std::iter::repeat(chats).take(1).flatten())
    //.on_submit(|s, item: &str| s.add_layer(Dialog::info(format!("Selected: {}", item))))
    .on_submit(|s, item: &str| chat(s, item))
    .with_name("chats");

  let horizontal_line = std::iter::repeat(String::from("|\n")).take(21).collect::<String>();

  let main_menu = LinearLayout::horizontal()
    .child(select)
    .child(DummyView)
    .child(TextView::new(horizontal_line))
    .child(terminal);

  s.add_layer(Dialog::around(main_menu)
    .title("Main menu")
    .button("Logout", login_menu)
    .button("Quit", Cursive::quit));

}

fn chat(s: &mut Cursive, chat_title: &str) {
  s.pop_layer();
  let chat_input = EditView::new()
    .filler(" ")
    // .style(ColorStyle::tertiary())
    .on_submit(chat_message)
    .with_name("input")
    .fixed_width(80);

  let chat_prefix = TextView::new("> ")
    .fixed_width(2);

  let chat = LinearLayout::vertical()
    .child(TextView::new(format!("-- {chat_title} --\n"))
      .with_name("output")
      .scrollable()
      .fixed_height(20))
    .child(LinearLayout::horizontal()
      .child(chat_prefix)
      .child(chat_input));

  let chat_menu = LinearLayout::horizontal()
    .child(chat);

  s.add_layer(Dialog::around(chat_menu)
    .title("Chat")
    .button("Back", main_menu)
    .button("Quit", Cursive::quit));
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
    "h"|"help" => { "Available commands: [h]elp, [q]uit, [l]ist, [cl]ear, join, create" },
    "q"|"quit" => { s.quit(); "Goodbye!" },
    "l"|"list" => { 
      match s.call_on_name("chats", 
        |v: &mut SelectView<&str>| { 
          v.iter()
            .map( |(s, _)| String::from(s) )
            .collect::<Vec<String>>()
      }) {
        Some(c) => &format!(
          "Available chats: {}", 
            c.iter()
              .map(|s| s.as_str())
              .collect::<Vec<&str>>()
              .join(", ")),
        None => "No chats available."
      }
    },
    "cl"|"clear" => { 
      let content = match s.call_on_name("output", |v: &mut TextView| { 
        v.get_content().to_owned().into_source() }) {
          // Only grab the first line of the content
          Some(c) => format!("{}\n", c.lines().next().unwrap().to_string()),
          None => String::from("")
      };
      s.call_on_name("output", |v: &mut TextView| { v.set_content(content); });
      ""
    },
    "join" => { "Please enter a chat name to join." },
    cmd if cmd.starts_with("join ") => {
      let param = &cmd[5..];
      if param == "" {
        "Invalid chat name."
      } else {
        let chat_name = match s.call_on_name("chats", 
          |v: &mut SelectView<&str>| { 
            match v.iter().find(move |(s, _)| *s == param) {
              Some((s, d)) => (String::from(s), String::from(*d)),
              None => ("".to_string(), "".to_string())
            }
        }) {
          Some((_, d)) => String::from(d),
          None => "".to_string()
        };
        
        if chat_name == "" {
          &format!("Chat '{}' not found.", param)
        } else {
          chat(s, chat_name.as_str());
          // &format!("Joining chat: {}", param)
          ""
        }
      }
    },
    "create" => { "Creating chat..." },
    _ => { "Unknown command. Type 'help' for a list of commands." }
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
