<h1 align=center> bush </h1>
Bush is a simple shell in Rust.

## Compatibilty
I think it just works on linux for now

## Build
Clone the repository:
```sh
git clone https://github.com/paulogarithm/bush
```
Enter it:
```sh
cd bush
```
Run with Cargo:
```sh
cargo run
```
To get the binary, do:
```sh
cargo build
# file target/debug/bush
```

## How to use
Just type shell commands, and press "CTRL + D" to exit
```
cargo run
[0] > ls -lF
total 24
-rw-r--r-- 1 pol pol  148 Nov 21 08:42 Cargo.lock
-rw-r--r-- 1 pol pol   58 Nov 21 08:42 Cargo.toml
-rw-r--r-- 1 pol pol 1069 Nov 21 08:42 LICENSE
-rw-r--r-- 1 pol pol  589 Nov 21 08:47 README.md
drwxr-xr-x 2 pol pol 4096 Nov 21 08:42 src/
drwxr-xr-x 3 pol pol 4096 Nov 21 08:42 target/
[0] > ls toto
ls: cannot access 'toto': No such file or directory
[2] > _
```