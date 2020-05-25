# `sfexec` - Self Extracting Executable

`sfexec` is a simple tool to create archive-like native binaries that extract contents from their static memory to temporary files when ran.
It is useful for packaging a bunch of binaries into a single one.
Temporary files are cleaned up after the post-extraction script hook.

## Example

``` shell
$ echo "Hello world!" > file.txt
$ sfexec-create -e 'cat %location/file.txt' - file.txt
Writing to /home/avril/work/sfexec/file.h...
 + test.txt OK
Adding lengths...
Adding names...
 - "test.txt" OK
Compiling binary...
Complete.
$ ./sfexec
Extracting 1 files to "/tmp/eda0bd22-9565-7e3c-e1d0-f7cdff96770e"...
 <- test.txt (13)
exec: cat /tmp/eda0bd22-9565-7e3c-e1d0-f7cdff96770e/test.txt
Hello world!
```

## Usage

It comes with 2 scripts, `sfexec-create` and `sfexec-create-compress`. Both take the same arguments:
| Argument      | Description                                         |
|---------------|-----------------------------------------------------|
| `-s`          | Silent mode. Do not output anything when extracting |
| `-e <string>` | Post-extraction hook. See below for details.        |
| `-`           | Stop reading argument flags                         |

`sfexec-create-compress` compresses the binary with `gzip`, and decompresses when executed.

### Post-extraction hook

The post extraction hook is passed to `/bin/sh`, with some input changes:
| Argument    | Usage                                                                                                 |
|-------------|-------------------------------------------------------------------------------------------------------|
| `%location` | The directory root that the files are extracted to                                                    |
| `%argc`     | The number of command line arguments passed to `sfexec`                                               |
| `%argv`     | A list of all args passed to `sfexec`                                                                 |
| `%arg[n]`   | The `n`th argument passed to `sfexec`, if `n` is outside the range of arguments, nothing is replaced. |

## Building
To build the `sfexec` binary, g++ is used, along with [sha256_literal] for verifying the post-extraction hook.
Included in the repo is a pre-built generator binary, signed with [my GPG key] at `generator-v<version>.gpg` with a checksum in `generator-v<version>.sha256`. Alternatively you can build it yourself like so:

[sha256_literal]: https://github.com/aguinet/sha256_literal
[my gpg key]: https://flanchan.moe/flanchan.asc

### Building the generator
To build the generator yourself, Rust and Cargo are needed.
``` shell
$ make clean && make generator
```
Will remove the pre-built generator binaries, build the generator, and symlink accordingly.

## License
GPL'd with love <3

