# Catsploit

## Automated Testing
General approach to automated testing within Catsploit is to write individual tests for all functions/methods that have some logical functionality that is easily testable.

For example in `catsploit_lib/src/core/exploit/remote_tcp.rs`, tests are written for the both `connect` and `custom_connect` but not for `opts`. A test could be written for `opts` that iterates through the values looking for `RHOST` etc., but this makes the changing the `opts` function more involved for not much benefit.

### Running Tests
TODO
