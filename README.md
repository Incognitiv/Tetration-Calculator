# Tetration-Calculator
A Rust-based tool for performing tetration calculations, allowing you to compute the result of tetration operations with ease and benchmark it a little.

Runs properly on 

[![Linux](https://github.com/Incognitiv/Tetration-Calculator/actions/workflows/rust_linux.yml/badge.svg)](https://github.com/Incognitiv/Tetration-Calculator/actions/workflows/rust_linux.yml)

[![Windows](https://github.com/Incognitiv/Tetration-Calculator/actions/workflows/rust_windows.yml/badge.svg)](https://github.com/Incognitiv/Tetration-Calculator/actions/workflows/rust_windows.yml)


## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Dependencies](#dependencies)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Tetration is a mathematical operation that iteratively applies exponentiation. This tool lets you specify the base and height values for the tetration operation and tries to provide the result (as long as the number of digits is lower than a million) along with other information.

## Getting Started

To get started with the Tetrator Calculator, you'll need Rust installed on your system. If you don't have Rust, you can install it from [the official Rust website](https://www.rust-lang.org/tools/install).


## Dependencies
  ```bash
  num-bigint = "0.4.4"
  num-traits = "0.2.17"
  num-format = "0.4.4"
  dialoguer = "0.11.0"
  structopt = "0.3.26"
  ```

  Links to each of the dependencies on **Docs.rs**:
  
  - [num-bigint](https://docs.rs/num-bigint/latest/num_bigint/),   
  - [num-traits](https://docs.rs/num-traits/latest/num_traits/), 
  - [num-format](https://docs.rs/num-format/latest/num_format/),
  - [dialoguer](https://docs.rs/dialoguer/latest/dialoguer/),
  - [structopt](https://docs.rs/structopt/latest/structopt/)

  **Crates.io**:
  
  - [num-bigint](https://crates.io/crates/num_bigint),
  - [num-traits](https://crates.io/crates/num_traits),
  - [num-format](https://crates.io/crates/num_format),
  - [dialoguer](https://crates.io/crates/dialoguer),
  - [structopt](https://crates.io/crates/structopt)
  

## Installation

Make sure you have Cargo (Rust's package manager) properly configured, and it will take care of downloading and building the dependencies for you.
1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/Incognitiv/Tetration-Calculator.git
   ```

2. Change your working directory to the project folder:
   
   ```bash
   cd Tetration-Calculator
   ```

4. Add all the necessary dependencies:
   
   ```rust
   cargo add num-bigint
   ```
   ```rust
   cargo add num-traits
   ```
   ```rust
   cargo add num-format
   ```
   ```rust
   cargo add dialoguer
   ```
   ```rust
   cargo add structopt
   ```

5. Build and run the program using Cargo:
   
   ```rust
   cargo run --release
   ```
   
## Usage

Just follow the on-screen prompts to enter the base and height values for your tetration calculation. The calculator will then display the result, the number of digits in the result, and the time taken for the calculation.

Example:
  ```bash
  Welcome to the Tetrator Calculator!

  Select an option:
  1. Perform Tetration
  2. Exit

  Please enter the base (a non-negative integer): 3
  Please enter the height (a non-negative integer): 2

  Tetrator(3, 2) =
  Result: 27
  Number of digits: 2
  Time taken: 15ms
  ```

Commandline Example:
  ```bash
  ./tetration_calculator<.exe> --base <value_1> --height <value_2>
  <value_1> can be up to 100
  <value_2> should rarely exceed 2, maybe 3, depends very much on your hardware
  ```

## Contributing

If you want to contribute to this project, feel free to fork the repository and submit pull requests. You can also open issues to report bugs or suggest improvements.

## License

This project is licensed under the BSD 3-Clause License (Incognitiv, 2023).

**License.md:**

```plaintext
BSD 3-Clause License

Copyright (c) 2023, Incognitiv

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```
