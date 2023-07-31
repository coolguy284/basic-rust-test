pub fn large_print_no_command() {
  println!("Coolguy284's basic rust experimentation program.");
  println!();
  println!("USAGE:");
  println!("    basic-rust-test <SUBCOMMAND>");
  println!();
  println!("SUBCOMMANDS (PRIMARY):");
  println!("    current_time");
  println!("    rng_simple");
  println!("    sleep");
  println!();
  println!("SUBCOMMANDS (DEBUGGING):");
  println!("    fixed_prec_parse_test");
  println!();
  println!("DISCUSSION:");
  println!("    Just a collection of random commands to do random things in rust.");
}

pub fn large_print_rng_simple() {
  println!("Coolguy284's basic rust experimentation program, random number module.");
  println!();
  println!("USAGE:");
  println!("    basic-rust-test rng_simple <OPTIONS>");
  println!();
  println!("OPTIONS:");
  println!("    --rng=<NAME>       The name of the RNG to use.");
  println!("        Valid RNGs:");
  println!("          Non random:    fourgenerator, countergenerator");
  println!("          PRNGs:         mt19937_32, mt19937_64");
  println!("          CSPRNGs:       cgcsprng1 (untested)");
  println!("    --seed-hex=<SEED>  The seed for the RNG, in hex.");
  println!("    --skip=<COUNT>     The number of initial outputs of the RNG to skip, in decimal.");
  println!("    --count=<COUNT>    The number of outputs of the RNG to display, in decimal.");
  println!();
  println!("DISCUSSION:");
  println!("    A command to print the output of a given RNG with a given seed. This should result in the same outputs every time on every platform, as long as a deterministic RNG is chosen.");
}

pub fn large_print_sleep() {
  println!("Coolguy284's basic rust experimentation program, sleep module.");
  println!();
  println!("USAGE:");
  println!("    basic-rust-test sleep [SECONDS] [OPTIONS]");
  println!();
  println!("OPTIONS:");
  println!("    [SECONDS]                     Time in seconds to sleep as an integer or a decimal.");
  println!("    --time-seconds=<SECONDS>      Time in seconds to sleep as an integer or a decimal.");
  println!("    --time-milliseconds=<MILLIS>  Time in milliseconds to sleep as an integer or a decimal.");
  println!("    --time-microseconds=<MICROS>  Time in microseconds to sleep as an integer or a decimal.");
  println!("    --time-nanoseconds=<NANOS>    Time in nanoseconds to sleep as an integer.");
  println!();
  println!("DISCUSSION:");
  println!("    A command to sleep a certain length of time. If a normal argument is given the first argument has precedence, if not the first one from the list above has precedence.");
}

pub fn large_print_fixed_prec_parse_test() {
  println!("Coolguy284's basic rust experimentation program, FixedPrec parse test module.");
  println!();
  println!("USAGE:");
  println!("    basic-rust-test fixed_prec_parse_test <OPTIONS>");
  println!();
  println!("OPTIONS:");
  println!("    --num-str=<VALUE>              The value to convert into a FixedPrec object and then back into a string.");
  println!("    --fractional-digits=<INTEGER>  The number of digits after the decimal point the FixedPrec object should have.");
  println!();
  println!("DISCUSSION:");
  println!("    A debug command to convert a number into a FixedPrec object and then back into a string, for testing.");
}
