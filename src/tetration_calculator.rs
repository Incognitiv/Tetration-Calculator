// tetration_calculator.rs

// Define a public module named 'tetration_calculator' used to organize and encapsulate related code
pub mod tetration_calculator {

    use num_traits::ToPrimitive;                    // Import the ToPrimitive trait for number conversion. This trait provides methods for converting numeric types to other numeric types
    use std::time::Instant;                         // Import the Instant struct from the standard library. Instant is used for measuring time, like calculating the duration between two time points
    use dialoguer::{Input, Select};                 // Import the Input and Select structs from the "dialoguer" crate. These are used for user interactions, specifically for receiving user input and displaying selectable options
    use num_format::{Locale, ToFormattedString};    // Import the Locale and ToFormattedString structs from the "num_format" crate. These are used for formatting numbers, such as adding commas to large numbers
    use num_bigint::BigUint;                        // Import the BigUint struct from the "num_bigint" crate. This type represents arbitrary-precision unsigned integers, often used in mathematical calculations that require very large numbers

    // Constant messages
    pub const WELCOME:                  &str = "Welcome to the Tetrator Calculator!";                                   // Welcome message
    pub const EXIT:                     &str = "Exit";                                                                  // Option to exit the program
    pub const BYE:                      &str = "Goodbye!";                                                              // Farewell message

    pub const OPTION_SELECT:            &str = "Select an option:";                                                     // Menu prompt
    pub const ENTER_BASE:               &str = "Please enter the base (a non-negative integer):";                       // Prompt for base input
    pub const ENTER_HEIGHT:             &str = "Please enter the height (a non-negative integer):";                     // Prompt for height input
    pub const START_COUNTING:           &str = "Perform Tetration";                                                     // Option to start tetration calculation

    pub const INVALID_BASE_INPUT:       &str = "Invalid base input:";                                                   // Error message for invalid base input
    pub const INVALID_HEIGHT_INPUT:     &str = "Invalid height input:";                                                 // Error message for invalid height input
    pub const INVALID_SELECTION:        &str = "Invalid selection. Please choose a valid option:";                      // Error message for invalid menu selection
    pub const INVALID_BASE_MESSAGE:     &str = "Invalid input for the base. Please enter a positive integer.";          // Error message for invalid base input format
    pub const INVALID_HEIGHT_MESSAGE:   &str = "Invalid input for the height. Please enter a non-negative integer.";    // Error message for invalid height input format
    pub const INVALID_BASE_AND_HEIGHT:  &str = "Invalid base and height inputs:";                                       // Error message for invalid base and height inputs
    pub const INVALID_INPUT_UNREADABLE: &str = "Failed to read input from the user.";                                   // Error message for unreadable input
    pub const ERROR_OCCURRED:           &str = "Error occurred during tetration calculation.";                          // Error message for a calculation error

    // Other constant values
    pub const ZERO:                     u128 = 0;                                                                       // Constant value for zero
    pub const ONE:                      u128 = 1;                                                                       // Constant value for one

    // Define a struct called TetrationCalculator, which represents a calculator for tetrations
    pub struct TetrationCalculator {
        base: u128,  // Stores the base value for the tetrator calculation
        height: u128,  // Stores the height (exponent) value for the tetrator calculation
    }

    // Define an implementation block for the TetrationCalculator struct
    impl TetrationCalculator {
        // Define a public function named "new" that constructs a new TetrationCalculator instance and returns it
        pub fn new() -> Self {
            // Initialize a new TetrationCalculator instance
            TetrationCalculator {
                base: ONE,  // Initializes the "base" field of the TetrationCalculator with a value represented by the constant ONE
                height: ZERO,  // Initializes the "height" field of the TetrationCalculator with a value represented by the constant ZERO
            }
        }
    
        // Set the base value of the calculator
        pub fn set_base(&mut self, value: u128) {
            self.base = value; // Set the base of the calculator to the provided value
        }

        // Get the current base value from the calculator
        pub fn get_base(&self) -> u128 {
            self.base // Get the current base value from the calculator
        }

        // Set the height value of the calculator
        pub fn set_height(&mut self, value: u128) {
            self.height = value; // Set the height of the calculator to the provided value
        }

        // Get the current height value from the calculator
        pub fn get_height(&self) -> u128 {
            self.height // Get the current height value from the calculator
        }
        
        // Calculate the tetration with the current base and height values
        pub fn calculate_tetration(&self) -> Option<BigUint> {
            let mut tetration_result = BigUint::from(1u8);  // Initialize a BigUint with a value of 1
            let mut height = self.height;  // Create a mutable variable "height" and initialize it with the value of "self.height"

            while height > 0 {  // Start a loop that continues as long as "height" is greater than 0
                let base_as_bigint = BigUint::from(self.base);  // Convert the "base" value from the calculator to a BigUint, a big integer type
            
                tetration_result = match tetration_result.to_u32() {  // Convert "tetration_result" to a u32, and use a match statement to handle the result
                            Some(tet_result_u32) => base_as_bigint.pow(tet_result_u32),  // Perform the tetration operation
                    None => return None,  // Return None if the conversion to u32 fails
                };
                height -= 1;  // Decrement the height
            }
            Some(tetration_result)  // Return the calculated tetration result
        }
    }

    // This function takes generic parameters and is used to validate and set input values
    // The "calculator" parameter is a mutable reference to a TetrationCalculator
    // "prompt" is a message that will be displayed to the user
    // "error_message" is a message to display in case of an error
    // "setter" is a function to set a value in the calculator
    fn validate_input<F>(calculator: &mut TetrationCalculator, prompt: &str, error_message: &str, setter: F, ) -> Result<(), String> where F: Fn(&mut TetrationCalculator, u128) {

        // Create a new input prompt for u128 values with the given "prompt" message, then interact with the user to obtain input
        let input = Input::<u128>::new().with_prompt(prompt).interact();

        // Matches the "input"
        match input {
            // If the input is Ok and equals 0, return an Err with the specified error message
            Ok(0) => Err(error_message.to_string()), 
            
            // If the "input" is Ok and not equal to 0, call the "setter" function with the "calculator" and "value" parameters to set a value in the calculator
            Ok(value) => {
                setter(calculator, value);
                
                // Return Ok with an empty unit value to indicate successful input validation
                Ok(()) 
            }
            
            // If there's an error reading the input (Err), return an Err with a predefined error message
            Err(_) => Err(INVALID_INPUT_UNREADABLE.to_string()), 
        }
    }

    // This function is responsible for displaying the results of the tetration calculation
    // It takes references to the calculator, the calculated result, and the elapsed time
    pub fn display_tetration_result(calculator: &TetrationCalculator, result: &BigUint, elapsed_time: &std::time::Duration) {

        // Convert the BigUint result into a string for display
        let result_str = result.to_string(); 

        // Calculate the number of digits in the result string
        let num_digits = result_str.len();

        // Format the number of digits using the en (English) locale
        let num_digits_formatted = num_digits.to_formatted_string(&Locale::en);

        // Display a message with the base and height values from the calculator
        println!("Tetrator({}, {}) =", calculator.get_base(), calculator.get_height());

        // Display the calculated result
        println!("Result: {}", result_str);

        // Display the number of digits in a formatted manner
        println!("Number of digits: {}", num_digits_formatted);

        // Display the elapsed time for the calculation
        println!("Time taken: {:?}", elapsed_time);
    }

    // This function coordinates the process of calculating and displaying tetration results
    // It takes a mutable reference to the "calculator" for setting base and height values
    pub fn calculate_tetration(calculator: &mut TetrationCalculator) {

        // Attempt to validate and set both the base and height values. The result is a tuple
        match (validate_input(calculator, ENTER_BASE, INVALID_BASE_MESSAGE, TetrationCalculator::set_base), validate_input(calculator, ENTER_HEIGHT, INVALID_HEIGHT_MESSAGE, TetrationCalculator::set_height)) {

            // If both base and height validation and setting are successful (Ok(())),
            (Ok(()), Ok(())) => {
                
                // Record the current time using the Instant module to measure elapsed time
                let start_time = Instant::now();
                
                // Attempt to calculate the tetration using the current calculator settings
                match calculator.calculate_tetration() {
                    
                    // If tetration calculation is successful and returns a result,
                    Some(result) => {
                    
                        // Calculate the elapsed time by subtracting "start_time" from the current time
                        let elapsed_time = start_time.elapsed();
                        
                        // Display the tetration result along with base, height, and time taken
                        display_tetration_result(calculator, &result, &elapsed_time);
                    }
                    // If tetration calculation fails, display an error message
                    None => println!("{}", ERROR_OCCURRED),
                }
            }
            // If there are errors in both the base and height inputs,
            (Err(base_err), Err(height_err)) => {
                
                // Display an error message indicating issues with both inputs
                println!("{}", INVALID_BASE_AND_HEIGHT);
                
                // Display the specific error related to the base input
                println!("Base Error: {}", base_err);
                
                // Display the specific error related to the height input
                println!("Height Error: {}", height_err);
            }
            // If there's an error in the base input:
            (Err(base_err), _) => {
                
                // Display an error message indicating issues with the base input
                println!("{}", INVALID_BASE_INPUT);
                
                // Display the specific error related to the base input
                println!("Base Error: {}", base_err);
            }
            // If there's an error in the height input,
            (_, Err(height_err)) => {
                
                // Display an error message indicating issues with the height input
                println!("{}", INVALID_HEIGHT_INPUT);
                
                // Display the specific error related to the height input
                println!("Height Error: {}", height_err);
            }
        }
    }

    // This is the main entry point of the program
    pub fn main() {
        
        // Display a welcome message to the user
        println!("{}", WELCOME);

        // Create a new instance of the TetrationCalculator to perform tetration calculations
        let mut calculator = TetrationCalculator::new();
        
        // Start an infinite loop to repeatedly prompt the user for actions
        loop {
            // Define an array of choices, including "START_COUNTING" and "EXIT"
            let choices = &[START_COUNTING, EXIT];
            
            // Use the "Select" builder to create a menu with the defined choices
            // Prompt the user to select an option, with a default choice of 0, and handle invalid input
            let selection = Select::new()
                .items(choices)
                .default(ZERO as usize)
                .with_prompt(OPTION_SELECT)
                .interact()
                .expect(INVALID_INPUT_UNREADABLE);
        
                // Use pattern matching to determine the user's selection
                match selection {
                
                // If the user selects option 0 (START_COUNTING), call the "calculate_tetration" function with the calculator
                0 => calculate_tetration(&mut calculator),
                
                // If the user selects option 1 (EXIT), display a goodbye message
                1 => {
                    println!("{}", BYE); // Displays a goodbye message
                    break; // Exit the loop to end the program
                }
                // If the user selects an option other than 0 or 1, display an error message indicating an invalid selection
                _ => println!("{}", INVALID_SELECTION),
            }
        }
    }
}
