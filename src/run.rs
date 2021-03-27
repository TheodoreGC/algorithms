use crate::algorithm::{Algorithm, Mode};
use crate::verify::test;
use indicatif::ProgressBar;

// Invoke the rust compiler on the path of the given algorithm,
// and run the ensuing binary.
// The verbose argument helps determine whether or not to show
// the output from the test harnesses (if the mode of the algorithm is test)
pub fn run(algorithm: &Algorithm, verbose: bool) -> Result<(), ()> {
    match algorithm.mode {
        Mode::Test => test(algorithm, verbose)?,
        Mode::Compile => compile_and_run(algorithm)?,
        Mode::Clippy => compile_and_run(algorithm)?,
    }
    Ok(())
}

// Invoke the rust compiler on the path of the given algorithm
// and run the ensuing binary.
// This is strictly for non-test binaries, so output is displayed
fn compile_and_run(algorithm: &Algorithm) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", algorithm).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation_result = algorithm.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compilation of {} failed!, Compiler error message:\n",
                algorithm
            );
            println!("{}", output.stderr);
            return Err(());
        }
    };

    progress_bar.set_message(format!("Running {}...", algorithm).as_str());
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            success!("Successfully ran {}", algorithm);
            Ok(())
        }
        Err(output) => {
            println!("{}", output.stdout);
            println!("{}", output.stderr);

            warn!("Ran {} with errors", algorithm);
            Err(())
        }
    }
}
