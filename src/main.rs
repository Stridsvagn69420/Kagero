use kagero::printer::{Printer, Colors};

fn main() {
    let mut printer = Printer::default();
    printer.errorln("This is still WIP!", Colors::RedBright);
}
