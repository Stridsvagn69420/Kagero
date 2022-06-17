use kagero::test;
use kagero::printer::{Printer, Colors};

fn main() {
    test();
    let mut printer = Printer::default();
    printer.println("You're gay!", Colors::Green);
    printer.errorln("Wait, that's illegal!", Colors::Magenta);
}
