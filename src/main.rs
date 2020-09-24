mod shell;

fn main() -> Result<(), ()> {
    shell::enter_shell();
    Ok(())
}
