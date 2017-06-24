# NixOS Distribution Scripts
Scripts to enable installation of this project in Nix and NixOS environment.

## Build and Install
To build the project and put it into the `/nix/store`, run the following.
```
# nix-build shell.nix -iA self
```

## Development Shell
To further develop the package, run `nix-shell`.
```
# nix-shell shell.nix
```
and we then can use the isolation shell to build and debug.

To build the project, run `cargo` as usual.
```
$ cargo build
```

Then run the executable,
```
$ ./target/debug/zoomy
```