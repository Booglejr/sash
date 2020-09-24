use std::io;
use std::env;
use std::io::Write;

pub fn enter_shell() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();


    let cwd = env::current_dir().expect("Couldn\'t get current working directory");
    let info_string = format!("{} >", cwd.to_str().expect("Couldn\'t convert CWD pathbuf to &str"));
    let mut buf = String::new();

    loop {
        print!("{} ", info_string);

        if let Err(e) = stdout.flush() {
            eprintln!("Error flushing stdout:\n {}", e);
        }

        match stdin.read_line(&mut buf) {
            Ok(_) => handle_input(&buf),
            Err(e) => eprintln!("Read error occurred: \n{}", e)
        };

        println!("{}", buf);
    }
}

fn handle_input(input: &String) {
    print!("{}", input);
}