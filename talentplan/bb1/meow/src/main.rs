#[macro_use]
extern crate clap;

fn main() {
    use clap::App;

    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();

    if let Some(mode) = m.value_of("mode") {
        match mode {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _      => unreachable!()
        }
    } else {
        println!("--mode <MODE> wasn't used...");
    }
}