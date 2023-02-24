# Catsploit

## Automated Testing
Some points on automated testing within Catsploit:

- Tests are written for logical functionality. For example in `catsploit_lib/src/core/exploit/remote_tcp.rs`, tests are written for the both `connect` and `custom_connect` but not for `opts`. A test could be written for `opts` that iterates through the values looking for `RHOST` etc., but this makes the changing the `opts` function more involved for not much benefit
- Tests are not written for functions that only print to STDOUT with no side effects. Example being `print_exploit` in `catsploit/src/cli/info.rs`
- It's fine to modify a functions parameters and code block solely to make it more testable. For example `show_exploits` in `catsploit/src/cli/cmd/show.rs` takes a boolean indicating if the function is running in a test or not, to prevent it from printing the full exploit table to STDOUT during tests. There may be possible ways to block the STDOUT printing in the tests using closures etc., that wouldn't need to modify the `show_exploits` function signature. The added complexity and development time for that isn't worth it to avoid a simple parameter change.

### Running Tests
To run tests for both the catsploit library and the CLI application:

```
cargo test --manifest-path=catsploit_lib/Cargo.toml && cargo test --manifest-path=catsploit/Cargo.toml
```
