mod args;

use args::ReminderArgs;
use clap::Parser;

fn main() {
    ReminderArgs::parse();

}
