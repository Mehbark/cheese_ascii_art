fn main() {
    let stuff = "--------v^^^^^^^^||||||||<<0<<1<<2<<3<<4<<5<<";
    let test = "<<0<<1<<2<<3<<4<<5<<";

    print!("\x1b[2J\x1b[H");

    print!(
        "{}",
        stuff
            .chars()
            .map(|c| {
                match c {
                    '-' => "^-.\x1b7\x1b[1G|\x1b8\x1b[8B\x1b[3D^-.\x1b8\x1b[1B",
                    'v' => "\x1b[7B",
                    '^' => "|\x1b[1A\x1b[1D",
                    '0' => "\x1b7\x1b[9Bo\x1b8",
                    '1' => "\x1b7\x1b[2Bo\x1b8",
                    '2' => "\x1b7\x1b[5Bo\x1b8",
                    '3' => "\x1b7\x1b[10B\x1b[11Co\x1b8",
                    '4' => "\x1b7\x1b[7Bo\x1b8",
                    '5' => "\x1b7\x1b[3Bo\x1b8",
                    '|' => "\\\x1b[2D\x1b[1A",
                    '<' => "-\x1b[2D",
                    _ => "",
                }
            })
            .collect::<String>()
    );

    print!("\x1b[20B");
    println!("{}", test);
}
