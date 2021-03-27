use crate::algorithm::{CompiledAlgorithm, Algorithm, Mode, State};
use console::style;
use indicatif::ProgressBar;

// Verify that the provided container of Algorithm objects
// can be compiled and run without any failures.
// Any such failures will be reported to the end user.
// If the Algorithm being verified is a test, the verbose boolean
// determines whether or not the test harness outputs are displayed.
pub fn verify<'a>(
    start_at: impl IntoIterator<Item = &'a Algorithm>,
    verbose: bool,
) -> Result<(), &'a Algorithm> {
    for algorithm in start_at {
        let compile_result = match algorithm.mode {
            Mode::Test => compile_and_test(&algorithm, RunMode::Interactive, verbose),
            Mode::Compile => compile_and_run_interactively(&algorithm),
            Mode::Clippy => compile_only(&algorithm),
        };
        if !compile_result.unwrap_or(false) {
            return Err(algorithm);
        }
    }
    Ok(())
}

enum RunMode {
    Interactive,
    NonInteractive,
}

// Compile and run the resulting test harness of the given Algorithm
pub fn test(algorithm: &Algorithm, verbose: bool) -> Result<(), ()> {
    compile_and_test(algorithm, RunMode::NonInteractive, verbose)?;
    Ok(())
}

// Invoke the rust compiler without running the resulting binary
fn compile_only(algorithm: &Algorithm) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", algorithm).as_str());
    progress_bar.enable_steady_tick(100);

    let _ = compile(&algorithm, &progress_bar)?;
    progress_bar.finish_and_clear();

    success!("Successfully compiled {}!", algorithm);
    Ok(prompt_for_completion(&algorithm, None))
}

// Compile the given Algorithm and run the resulting binary in an interactive mode
fn compile_and_run_interactively(algorithm: &Algorithm) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", algorithm).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation = compile(&algorithm, &progress_bar)?;

    progress_bar.set_message(format!("Running {}...", algorithm).as_str());
    let result = compilation.run();
    progress_bar.finish_and_clear();

    let output = match result {
        Ok(output) => output,
        Err(output) => {
            warn!("Ran {} with errors", algorithm);
            println!("{}", output.stdout);
            println!("{}", output.stderr);
            return Err(());
        }
    };

    success!("Successfully ran {}!", algorithm);

    Ok(prompt_for_completion(&algorithm, Some(output.stdout)))
}

// Compile the given Algorithm as a test harness and display
// the output if verbose is set to true
fn compile_and_test(algorithm: &Algorithm, run_mode: RunMode, verbose: bool) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Testing {}...", algorithm).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation = compile(algorithm, &progress_bar)?;
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            if verbose {
                println!("{}", output.stdout);
            }
            success!("Successfully tested {}", &algorithm);
            if let RunMode::Interactive = run_mode {
                Ok(prompt_for_completion(&algorithm, None))
            } else {
                Ok(true)
            }
        }
        Err(output) => {
            warn!(
                "Testing of {} failed! Please try again. Here's the output:",
                algorithm
            );
            println!("{}", output.stdout);
            Err(())
        }
    }
}

// Compile the given Algorithm and return an object with information
// about the state of the compilation
fn compile<'a, 'b>(
    algorithm: &'a Algorithm,
    progress_bar: &'b ProgressBar,
) -> Result<CompiledAlgorithm<'a>, ()> {
    let compilation_result = algorithm.compile();

    match compilation_result {
        Ok(compilation) => Ok(compilation),
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compiling of {} failed! Please try again. Here's the output:",
                algorithm
            );
            println!("{}", output.stderr);
            Err(())
        }
    }
}

fn prompt_for_completion(algorithm: &Algorithm, prompt_output: Option<String>) -> bool {
    let context = match algorithm.state() {
        State::Done => return true,
        State::Pending(context) => context,
    };

    let success_msg = match algorithm.mode {
        Mode::Compile => "The code is compiling!",
        Mode::Test => "The code is compiling, and the tests pass!",
        Mode::Clippy => "The code is compiling, and 📎 Clippy 📎 is happy!",
    };

    println!();
    println!("🎉 🎉  {} 🎉 🎉", success_msg);
    println!();

    if let Some(output) = prompt_output {
        println!("Output:");
        println!("{}", separator());
        println!("{}", output);
        println!("{}", separator());
        println!();
    }

    println!("You can keep working on this algorithm,");
    println!(
        "or jump into the next one by removing the {} comment:",
        style("`I AM NOT DONE`").bold()
    );
    println!();
    for context_line in context {
        let formatted_line = if context_line.important {
            format!("{}", style(context_line.line).bold())
        } else {
            context_line.line.to_string()
        };

        println!(
            "{:>2} {}  {}",
            style(context_line.number).blue().bold(),
            style("|").blue(),
            formatted_line
        );
    }

    false
}

fn separator() -> console::StyledObject<&'static str> {
    style("====================").bold()
}
