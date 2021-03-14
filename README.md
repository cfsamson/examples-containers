# Containers

Following along Liz Rice's presentation from GOTO 2018 (https://www.youtube.com/watch?v=8fi7uSYlOdc)
in Rust.

The presentation is done with GO and the defalut "Command" interface, along with
the "syscall" package in GO makes this example quite a bit easier to follow.

It turns out to be a bit more difficult to follow in Rust but I figured it would
be a good example to show some basic FFI and walk through some edge cases often
encountered crossing FFI boundaries in Rust (Trait objects / wide pointers).

Even if the complexity is a bit higher in the start it becomes less complex
since we have a way to execute code in the new process before we execute the
new process image typically invoced using the "Command" interface in both
Rust and GO.
