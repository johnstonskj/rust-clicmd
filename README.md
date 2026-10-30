# crate clicmd

This crate provides some composable components for command-line applications.

<!-- markdownlint-disable MD013 -->
[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![crates.io](https://img.shields.io/crates/v/rust-clicmd.svg)](https://crates.io/crates/rust-clicmd)
[![docs.rs](https://docs.rs/rust-clicmd/badge.svg)](https://docs.rs/rust-clicmd)
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

TBD

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