use crate::app::{Application, Event};

mod app;

mod services;
#[tokio::main]
async fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    println!("welcome to this card draw simulator, type register or login to continue...");
    let mut app = app::App::new();
    while let Ok(_) = stdin.read_line(&mut input) {
        let command = input.trim();
        match app.poll_events(command).await {
            Ok(event) => match event {
                Event::Exit(msg) => {
                    println!("{}", msg);
                    break;
                }
                Event::LogIn(res) => {
                    match res {
                        Ok(_) => println!("logIn succeeded"),

                        Err(err) => eprintln!("{}", err),
                    }
                    input = String::new();
                    continue;
                }
                Event::Register(res) => {
                    match res {
                        Ok(_) => println!("register succeeded"),
                        Err(err) => eprintln!("{}", err),
                    }
                    input = String::new();
                    continue;
                }
            },
            Err(err) => {
                eprintln!("{}", err);
                input = String::new();
                continue;
            }
        }
    }
}
