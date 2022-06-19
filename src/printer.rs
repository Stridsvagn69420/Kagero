use std::io::{Write, Stdout, stdout, Stderr, stderr, Stdin, stdin};


/// Printer to print to stdout and stderr and fead from stdin.
/// 
/// This is a simple wrapper around [Stdout], [Stderr] and [Stdin],
/// but featues all the tools you need for printing to the terminal and reading user input.
/// 
/// # Examples
/// Printing with color to stderr:
/// ```
/// use kagero::printer::{Printer, Colors};
/// let mut printer = Printer::default();
/// printer.errorln("This is a BIG error!", Colors::RedBright)
/// ```
/// 
/// Reading user input from the same file:
/// ```
/// use kagero::printer::{Printer, Colors};
/// let mut printer = Printer::default();
/// let input = printer.prompt("Enter your name: ", Colors::GreenBright);
/// ```
pub struct Printer {
    pub stdout: Stdout,
    pub stderr: Stderr,
    pub stdin: Stdin,
}

impl Printer {
    /// Create a new printer with default stdout, stderr and stdin
    /// 
    /// Printer with [stdout], [stderr] and [stdin] from the standard library.
    pub fn default() -> Printer {
        Printer {
            stdout: stdout(),
            stderr: stderr(),
            stdin: stdin(),
        }
    }

    /// Create a new printer with custom stdout, stderr and stdin
    /// 
    /// Printer with a custom [Stdout], [Stderr] and [Stdin].
    pub fn new(output: Stdout, error: Stderr, input: Stdin) -> Printer {
        Printer {
            stdout: output,
            stderr: error,
            stdin: input,
        }
    }

    /// Prints to stdout with color
    /// 
    /// # Arguments
    /// * `msg` - The message to print as a [str]
    /// * `color` - The color to print the message in as a [Color](Colors)
    pub fn print(&mut self, msg: &str, color: Colors) {
        let _ = self.stdout.write_all(color.as_ref());
        let _ = self.stdout.write_all(msg.as_bytes());
        let _ = self.stdout.write_all(RESET);
        let _ = self.stdout.flush();
    }

    /// Prints to stdout with color and newline
    /// 
    /// # Arguments
    /// * `msg` - The message to print as a [str]
    /// * `color` - The color to print the message in as a [Color](Colors)
    pub fn println(&mut self, msg: &str, color: Colors) {
        self.print(format!("{}\n", msg).as_str(), color)
    }

    /// Prints to stdout
    /// 
    /// # Arguments
    /// * `msg` - The message to print as a [str]
    pub fn write(&mut self, msg: &str) {
        let _ = self.stdout.write_all(msg.as_bytes());
        let _ = self.stdout.write_all(RESET);
        let _ = self.stdout.flush();
    }

    /// Prints to stdout with newline
    /// 
    /// # Arguments
    /// * `msg` - The message to print as a [str]
    pub fn writeln(&mut self, msg: &str) {
        self.write(format!("{}\n", msg).as_str())
    }

    /// Print to stderr with color
    /// 
    /// # Arguments
    /// * `msg` - Message to print as a [str]
    /// * `color` - Color to print with from the [Colors] enum
    pub fn error(&mut self, msg: &str, color: Colors) {
        let _ = self.stderr.write_all(color.as_ref());
        let _ = self.stderr.write_all(msg.as_bytes());
        let _ = self.stderr.write_all(RESET);
        let _ = self.stderr.flush();
    }

    /// Print to stderr with color and newline
    /// 
    /// # Arguments
    /// * `msg` - Message to print as a [str]
    /// * `color` - Color to print with from the [Colors] enum
    pub fn errorln(&mut self, msg: &str, color: Colors) {
        self.error(format!("{}\n", msg).as_str(), color);
    }

    /// Print to stderr
    /// 
    /// # Arguments
    /// * `msg` - Message to print as a [str]
    pub fn err(&mut self, msg: &str) {
        let _ = self.stderr.write_all(msg.as_bytes());
        let _ = self.stderr.write_all(RESET);
        let _ = self.stderr.flush();
    }

    /// Print to stderr and newline
    /// 
    /// # Arguments
    /// * `msg` - Message to print as a [str]
    pub fn errln(&mut self, msg: &str) {
        self.err(format!("{}\n", msg).as_str());
    }

    /// Reads from stdin
    /// 
    /// This function ignores any errors with non UTF-8 bytes since this shouldn't happen in a console.
    /// If you use a custom stdin, it's up to you to make sure it's UTF-8.
    /// You can also directly get [Printer::stdin](Stdin) and use `read_line` on it, if you have to check for non-UTF-8 bytes.
    pub fn readln(&mut self) -> String {
        let mut input = String::new();
        let _ = self.stdin.read_line(&mut input);
        input
    }

    /// Promts the user for input on the same line
    /// 
    /// # Arguments
    /// * `msg` - Message to print as a [str]
    pub fn ask(&mut self, msg: &str) -> String {
        self.prompt(msg, Colors::None)
    }

    /// Promts the user for input on a new line
    ///
    /// # Arguments
    /// * `msg` - Message to print as a [str]
    pub fn askln(&mut self, msg: &str) -> String{
        self.promptln(msg, Colors::None)
    }

    /// Prompts the user for input on the same line with a color
    /// 
    /// # Arguments
    /// * `msg` - Message to print as a [str]
    /// * `color` - Color to print with from the [Colors] enum
    pub fn prompt(&mut self, msg: &str, color: Colors) -> String {
        self.print(format!("{}: ", msg).as_str(), color);
        self.readln().trim().to_string()
    }

    /// Prompts the user for input on a new line with color
    /// 
    /// # Arguments
    /// * `msg` - Message to print as a [str]
    /// * `color` - Color to print with from the [Colors] enum
    pub fn promptln(&mut self, msg: &str, color: Colors) -> String {
        self.println(format!("{}:", msg).as_str(), color);
        self.readln().trim().to_string()
    }
}

const RESET: &'static [u8] = b"\x1b[0m";

/// Terminal Colors
/// 
/// Enum of ASCII escape codes to represent terminal colors.
/// 
/// # Examples
/// 
/// Printing a simple message in green
/// 
/// ```
/// use kagero::printer::{Printer, Colors};
/// let mut printer = Printer::default();
/// printer.println("Hello, world!", Colors::Green);
/// ```
/// 
/// Printing a simple message in bright red
/// 
/// ```
/// use kagero::printer::{Printer, Colors};
/// let mut printer = Printer::default();
/// printer.println("Hello, world!", Colors::RedBright);
/// ```
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

impl AsRef<[u8]> for Colors {
    fn as_ref(&self) -> &'static [u8] {
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

impl AsRef<str> for Colors {
    fn as_ref(&self) -> &'static str {
        match self {
            Colors::None => "",
            // Default
            Colors::Black => "\x1b[30m",
            Colors::Red => "\x1b[31m",
            Colors::Green => "\x1b[32m",
            Colors::Yellow => "\x1b[33m",
            Colors::Blue => "\x1b[34m",
            Colors::Magenta => "\x1b[35m",
            Colors::Cyan => "\x1b[36m",
            Colors::White => "\x1b[37m",
            // Bright
            Colors::BlackBright => "\x1b[90m",
            Colors::RedBright => "\x1b[91m",
            Colors::GreenBright => "\x1b[92m",
            Colors::YellowBright => "\x1b[93m",
            Colors::BlueBright => "\x1b[94m",
            Colors::MagentaBright => "\x1b[95m",
            Colors::CyanBright => "\x1b[96m",
            Colors::WhiteBright => "\x1b[97m"
        }
    }
}