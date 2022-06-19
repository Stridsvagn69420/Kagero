use kagero::printer::{Printer, Colors};

fn main() {
    let mut printer = Printer::default();
    printer.println("You're gay!", Colors::Green);
    printer.errorln("Wait, that's illegal!", Colors::Magenta);
    let username = printer.prompt("Tell me, what's your name, mate?", Colors::CyanBright);
    let input = printer.promptln(format!("Oh, hello there {}! Type in something...", username).as_str(), Colors::Yellow);
    printer.writeln(input.as_str());
}
