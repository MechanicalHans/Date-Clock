use date_clock::*;
use std::io::{self, prelude::*};

fn main() -> io::Result<()> {
    writeln!(io::stdout(), "{}", DateClock::today())
}
