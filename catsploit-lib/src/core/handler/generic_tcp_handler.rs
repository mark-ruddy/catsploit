// TODO: I think what I want here is a generic TCP listener that can be launched just before a payload is sent over
// The server itself probably needs to run in its own thread, so it doesn't block the rest of the library executing
// Single client should be accepted, don't see any reason to want multiple clients for a revshell
// Need to be able to attach the servers I/O to the terminal too, that logic maybe can be implemented in handler.rs
