// Module for handling the command line arguments facility
extern crate docopt;

// Defining a USAGE string
pub const USAGE: &'static str = "
Leftlock is a simpple session lock for X11.

Usage: leftlock [options]

Options:
    -h, --help          Show this message.
    -p, --password      Change password.
    -k, --keypad        Lock only keypad.
";
