Attempts at learning how to interact with WinAPI with Rust, with objective of having a basic wrapper api built at the end.
Basic rules are, having to build everything ourselves... until we have an implementation that works.

Current goals:
- Improve public api:
    - Public API shouldn't deal with WinAPI types.
    - Maybe set up a system to pass closures that define window behaviour?

Main things learnt so far:
- C interop
- Custom Error types
- Creating an API
- Safe API for unsafe functions