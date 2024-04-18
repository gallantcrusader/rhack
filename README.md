# rhack

Patching utility!

## Usage:

### Help
```
$ rhack                  
Patch/Rom hacking utility.

Usage: rhack <COMMAND>

Commands:
  ips     Uses IPS patching
  xdelta  Uses XDELTA patching
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Patching
```
$ rhack xdelta
Uses XDELTA patching

Usage: rhack xdelta [OPTIONS]

Options:
  -i, --input-file <FILE>   File to patch
  -p, --patch-file <FILE>   Patch file to use
  -o, --output-file <FILE>  Output file
  -h, --help                Print help

$ rhack ips   
Uses IPS patching

Usage: rhack ips [OPTIONS]

Options:
  -i, --input-file <FILE>   File to patch
  -p, --patch-file <FILE>   Patch file to use
  -o, --output-file <FILE>  Output file
  -h, --help                Print help
```
