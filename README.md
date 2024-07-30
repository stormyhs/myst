# The Myst Programming Language

Myst is a programming language that will attempt to be simple to read and understand, reliable, and fun!

Its syntax will be similar to TypeScript, but its behavior will seek to avoid the *quirks* of JavaScript.

Currently, Myst is tokenized, parsed, and interpreted using Rust. In the future Myst will be compiled (or JITed) using a custom bytecode runtime.


### How to run
In this stage of development, it is suggested you use Rust's `cargo` to run Myst.

An example of running `myst/example.myst`:
```bash
cargo r myst/example.myst
```

### Notes
Myst also takes some CLI arguments. You can use `--debug` for debugging information as the program runs, or `--repl` to evaluate code live, as the user feeds input source code.

To do this through `cargo`, you would use this command:
```bash
cargo r myst/example.myst -- --debug
```

Or, if you are running a complied executable directly:
```bash
./myst myst/example.myst --debug
```
