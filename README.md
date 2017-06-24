# Project Zoomy
A trial to DRM KMS technology in direct rendering. One of
future path to my own Desktop Environment.

## Installation
For information on distribution specific, refer to `./distro`
subfolder. But, in general to build the project, run `cargo` as usual.
```
$ cargo build
```

Then run the executable,
```
$ ./target/debug/zoomy
```

> **As a Notice**<br/>
> If you run XServer, it will block `zoomy` to gain access as
> DRM master. Thus, the app will error with `EACCESS`
> –permission denied. Instead, run in unused tty.

## License and Attribution
Licensed under `General Public License version 3.0`.

Hardly learned and based on,
1. DRM/KMS tutorial https://github.com/dvdhrm/docs
2. Dive in the Linux kernel code