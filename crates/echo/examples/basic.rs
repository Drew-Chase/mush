use echo::cli::EchoConfig;

fn main() {
    let config = EchoConfig {
        no_newline: false,
        interpret_escapes: false,
        args: vec!["hello".to_string(), "world".to_string()],
    };

    let joined = config.args.join(" ");
    if config.no_newline {
        print!("{joined}");
    } else {
        println!("{joined}");
    }
}
