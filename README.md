# The Myst Programming Language

Myst is a programming language that will attempt to be simple to read and understand, reliable, and fun!

Its syntax will be similar to TypeScript, but its behavior will seek to avoid the *quirks* of JavaScript.

Currently, Myst is tokenized, parsed, and interpreted using Rust. In the future Myst will be compiled (or JITed) using a custom bytecode runtime.

## How to run

Compiling a `myst` executable and having a `.myst` source file is all that is needed to run Myst. The executable simply takes in a path to a `.myst` file and attempts to run it.

An example of running `myst/example.myst`:
```bash
cargo r myst/example.myst
```

Or, if you are running a complied executable directly:
```bash
./myst myst/example.myst
```

For debug information, use the `--debug` flag.

To run the test suite, use the `--tests` flag, and do not provide a source file destination.

Please note that if you are using cargo to run myst, you need `-- --debug` to pass in arguments to the executable, rather than to cargo.

---

### Licensing
This project is multi-licensed under both GPLv3, as well as the Non AI Usage License. Both licenses apply simultaneously.
