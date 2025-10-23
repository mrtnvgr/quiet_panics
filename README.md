# quiet-panics

Hides debugging information from the user, leaving only the panic message.

Turns this:

```console
thread 'main' panicked at src/main.rs:22:9:
test.patch.ini is not a file
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Into this:
```console
test.patch.ini is not a file
```

## Usage:

```rust
use quiet_panics::set_panic_hook;

fn main() {
    set_panic_hook();

    // ...
}
```
