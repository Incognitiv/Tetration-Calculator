use structopt::StructOpt;                                               // Import the StructOpt trait, which is used for command-line argument parsing
use tetration_calculator::tetration_calculator::TetrationCalculator;    // Import the TetrationCalculator struct from the 'tetration_calculator' module

// Declare the 'tetration_calculator' module, which is expected to contain tetration calculation functionality
mod tetration_calculator;                                               

#[derive(Debug, StructOpt)]

// Define a structure 'Opt' to hold command-line arguments and options using StructOpt attributes
struct Opt {
    // Define command-line options for the 'base' and 'height' values.
    #[structopt(short, long)]

    // Base value
    base: Option<u128>, 

    #[structopt(short, long)]

    // Height value
    height: Option<u128>,
}

// The main function is the entry point of the program.
fn main() {

    // Parse command-line arguments into the "Opt" structure
    let opt = Opt::from_args();

    // Check if both base and height options are provided in the command-line arguments
    if opt.base.is_some() && opt.height.is_some() {

        // Create a new instance of the TetrationCalculator
        let mut calculator = TetrationCalculator::new();

        // Set the base and height values in the calculator
        calculator.set_base(opt.base.unwrap());
        calculator.set_height(opt.height.unwrap());

        // Record the current time for measuring elapsed time
        let start_time = std::time::Instant::now();

        // Calculate the tetration result using the provided base and height values
        let result = calculator.calculate_tetration();

        // Calculate the elapsed time
        let elapsed_time = start_time.elapsed();

        match result {
                // If the tetration calculation is successful and produces a result
                Some(result) => {

                // Display the tetration result along with base, height, and time taken
                tetration_calculator::tetration_calculator::display_tetration_result(&calculator, &result, &elapsed_time);
            }
            // If the tetration calculation fails
            None => {
                // Display an error message
                println!("{}", tetration_calculator::tetration_calculator::ERROR_OCCURRED);
            }
        }
    // If the required base and height options are not provided in the command-line arguments
    } else {
        // Invoke the "main" function of the "tetration_calculator" module to handle user interactions
        tetration_calculator::tetration_calculator::main();
    }
}
