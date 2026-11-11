# crate clicmd

This crate provides some composable components for command-line applications.

<!-- markdownlint-disable MD013 -->
[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![crates.io](https://img.shields.io/crates/v/clicmd.svg)](https://crates.io/crates/clicmd)
[![docs.rs](https://docs.rs/clicmd/badge.svg)](https://docs.rs/clicmd)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-clicmd.svg)](<https://github.com/johnstonskj/rust-clicmd/stargazers>)
<!-- markdownlint-enable MD013 -->

Rather than focus on the Cli structures as primary abstractions, this library
emphasizes composable *command* components. So actions in the application are
represented as commands which are then composed into initialization,
command-dispatch, and cleanup, rather than being tightly coupled to the CLI
structure.

A command implements the `Command` trait, as follows, which is a general-purpose
type, but by convention for CLI commands is usually used with output as
`std::process::ExitCode` and with the error as `crate::error::CliError`. This
allows for commands to be easily chained, nested and otherwise composed.

```rust
pub trait Command {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: Error;

    /// Executes the command, consuming self.
    fn execute(self) -> Result<Self::Output, Self::Error>;
}
```

You will notice that some features such as `color` and `verbosity` are optional
and yet the associated components are still usable and composable within the
application. This allows the default behavior of the application to remain
consistent regardless of which optional features, and by implication exposed
command-line options, are enabled.

## Examples

## Example Single Command CLI

```rust
use clicmd::{
    Command,
    clap::Parser,
    cli_main,
    commands::{Verbosity, color::Color},
    error::CliError,
};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(flatten)]
    verbosity: Verbosity,

    #[command(flatten)]
    color: Color,

    message: Option<String>,
}

impl Command for Cli {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        self.verbosity.execute()?;
        self.color.execute();
         println!(
            "Message: {:?}",
            self.message.unwrap_or_else(|| "Hello world".into())
        );
        Ok(ExitCode::SUCCESS)
    }
}

fn main() -> ExitCode {
    cli_main::<Cli>()
}
```

## Example Multiple Command CLI

```rust
use clicmd::{
    Command,
    clap::{Subcommand},
    cli_main,
    commands::CliWithSubCommands,
    error::CliError,
};
use std::process::ExitCode;

#[derive(Debug, Subcommand)]
pub enum GreetingCommands {
    /// Print a message
    Hello {
        /// The message to print
        message: String,
    },
    /// Print a message
    Goodbye {
        /// The message to print
        message: String,
    },
}

type Cli = CliWithSubCommands<GreetingCommands>;

impl Command for GreetingCommands {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
       match self {
            GreetingCommands::Hello { message } => {
                println!("Hello: {}", message);
            }
            GreetingCommands::Goodbye { message } => {
                println!("Goodbye: {}", message);
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

fn main() -> ExitCode {
    cli_main::<Cli>()
}
```

## Licenses

### Apache-2.0

<!-- markdownlint-disable MD013 -->
> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-Apache).
<!-- markdownlint-enable MD013 -->

### MIT

<!-- markdownlint-disable MD013 -->
> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-MIT).
<!-- markdownlint-enable MD013 -->