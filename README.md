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


## Create a new file system

`sudo` is needed in WSL since you can't run as root.

```text
chr=/home/dev/test/container-root
mkdir -p $chr
mkdir -p $chr/{bin,lib,lib64,proc}
cd $chr
sudo cp -v /bin/{bash,touch,ls,rm, ps, mount, sleep} $chr/bin
list="$(ldd /bin/bash | egrep -o '/lib.*\.[0-9]*')"
echo $list # optional
for i in $list; do sudo cp -v --parents "$i" "/home/dev/test/container-root"; done
list="$(ldd /bin/touch | egrep -o '/lib.*\.[0-9]*')"
for i in $list; do sudo cp -v --parents "$i" "/home/dev/test/container-root"; done
list="$(ldd /bin/ls | egrep -o '/lib.*\.[0-9]*')"
for i in $list; do sudo cp -v --parents "$i" "/home/dev/test/container-root"; done
list="$(ldd /bin/ls | egrep -o '/lib.*\.[0-9]*')"
for i in $list; do sudo cp -v --parents "$i" "/home/dev/test/container-root"; done
list="$(ldd /bin/ps | egrep -o '/lib.*\.[0-9]*')"
for i in $list; do sudo cp -v --parents "$i" "/home/dev/test/container-root"; done
list="$(ldd /bin/mount | egrep -o '/lib.*\.[0-9]*')"
for i in $list; do sudo cp -v --parents "$i" "/home/dev/test/container-root"; done
list="$(ldd /bin/sleep | egrep -o '/lib.*\.[0-9]*')"
for i in $list; do sudo cp -v --parents "$i" "/home/dev/test/container-root"; done
```


## Running in WSL

Since WSL doesn't allow you to run as root, you need to do `cargo build`, and
then `sudo ./target/debug/containers run /bin/bash` to run the example.