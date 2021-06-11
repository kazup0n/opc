opc
=====================================================

## What's this ??

opc is a tiny wrapper of 1Password CLI (op) .
opc manages cache of token genarated by `op signin`.

## Details

- Token cache saves in `~/.config/opc/cache.toml`
- Cache expires after 30 min regardless of use.

see also [1Password CLI doc Get started with the command-line tool
](https://support.1password.com/command-line-getting-started/#get-started-with-the-command-line-tool)

## Usage

```
USAGE:
    opc [FLAGS] <ACCOUNT>

FLAGS:
    -h, --help       Prints help information
        --refresh    force refresh token cache
    -V, --version    Prints version information

ARGS:
    <ACCOUNT>    Name of account (see output of `op signin list`)
```
