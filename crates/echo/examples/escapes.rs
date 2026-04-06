use echo::escape::process_escapes;

fn main() {
    let examples = [
        r"Hello\tWorld",
        r"Line1\nLine2\nLine3",
        r"Alert: \a",
        r"Hex A: \x41",
        r"Octal A: \0101",
        r"Stop here\cignored",
        r"Backslash: \\",
    ];

    for input in &examples {
        let (output, stopped) = process_escapes(input);
        println!("Input:   {input}");
        println!("Output:  {output:?}");
        if stopped {
            println!("  (output stopped by \\c)");
        }
        println!();
    }
}
