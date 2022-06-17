use std::{io::{Write, Stdout, stdout, Stderr, stderr/*, Stdin, stdin*/}};

pub struct Printer {
    stdout: Stdout,
    stderr: Stderr,
    //stdin: Stdin,
}
impl Printer {
    pub fn default() -> Printer {
        Printer {
            stdout: stdout(),
            stderr: stderr(),
            //stdin: stdin(),
        }
    }
    pub fn new(output: Stdout, error: Stderr/*, input: Stdin*/) -> Printer {
        Printer {
            stdout: output,
            stderr: error,
            //stdin: input,
        }
    }
    // --- Stdout ---
    pub fn print(&mut self, msg: &str, color: Colors) {
        let _ = self.stdout.write_all(color.as_bytes());
        let _ = self.stdout.write_all(msg.as_bytes());
        let _ = self.stdout.write_all(RESET);
        let _ = self.stdout.flush();
    }
    pub fn println(&mut self, msg: &str, color: Colors) {
        self.print(format!("{}\n", msg).as_str(), color)
    }
    pub fn write(&mut self, msg: &str) {
        let _ = self.stdout.write(msg.as_bytes());
        let _ = self.stdout.write_all(RESET);
        let _ = self.stdout.flush();
    }
    pub fn writeln(&mut self, msg: &str) {
        self.write(format!("{}\n", msg).as_str())
    }

    // --- Stderr ---
    pub fn error(&mut self, msg: &str, color: Colors) {
        let _ = self.stderr.write(color.as_bytes());
        let _ = self.stderr.write(msg.as_bytes());
        let _ = self.stderr.write_all(RESET);
        let _ = self.stderr.flush();
    }
    pub fn errorln(&mut self, msg: &str, color: Colors) {
        self.error(format!("{}\n", msg).as_str(), color);
    }
    pub fn err(&mut self, msg: &str) {
        let _ = self.stderr.write(msg.as_bytes());
        let _ = self.stderr.write_all(RESET);
        let _ = self.stderr.flush();
    }
    pub fn errln(&mut self, msg: &str) {
        self.err(format!("{}\n", msg).as_str());
    }
}

const RESET: &'static [u8] = b"\x1b[0m";
pub enum Colors {
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BlackBright,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright
}

impl Colors {
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            Colors::None => b"",
            // Default
            Colors::Black => b"\x1b[30m",
            Colors::Red => b"\x1b[31m",
            Colors::Green => b"\x1b[32m",
            Colors::Yellow => b"\x1b[33m",
            Colors::Blue => b"\x1b[34m",
            Colors::Magenta => b"\x1b[35m",
            Colors::Cyan => b"\x1b[36m",
            Colors::White => b"\x1b[37m",
            // Bright
            Colors::BlackBright => b"\x1b[90m",
            Colors::RedBright => b"\x1b[91m",
            Colors::GreenBright => b"\x1b[92m",
            Colors::YellowBright => b"\x1b[93m",
            Colors::BlueBright => b"\x1b[94m",
            Colors::MagentaBright => b"\x1b[95m",
            Colors::CyanBright => b"\x1b[96m",
            Colors::WhiteBright => b"\x1b[97m"
        }
    }
}