# caber

Embed binary data into JavaScript and TypeScript projects.

## Install

```shell
cargo install caber --features cli
```

## How-to

```shell
> caber --help
caber 0.1.0
Gerald Nash (aunyks)
Embed binary data into JavaScript and TypeScript projects.

USAGE:
    caber [OPTIONS] <INPUT_FILE>

ARGS:
    <INPUT_FILE>    The binary to be embedded

OPTIONS:
    -h, --help                         Print help information
    -l, --output-lang <OUTPUT_LANG>    The output language of the binary (javascript / ecmascript,
                                       typescript, etc)
    -m, --export-mode <EXPORT_MODE>    The export mode of the typed array (default, object, or none)
    -o, --output-file <OUTPUT_FILE>    The output file containing the embedded file
    -V, --version                      Print version information
```

## Examples

Embed a `file.bin` file and export the typed array with `export default ...`. The result is written to stdout.

```shell
> caber ./file.bin
const binary = new Uint8Array([104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

export { binary };
```

Embed a `file.bin` file into a `file.ts` TypeScript file and export the typed array with `export default ...`.

```shell
> caber ./file.bin --output-lang typescript --export-mode default --output-file file.ts
```

Embed a `file.bin` file as JavaScript and export the typed array with `export { ... }`. The result is written to stdout.

```shell
> caber ./file.bin --output-lang javascript --export-mode object
const binary = new Uint8Array([104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

export { binary };
```

Embed a `file.bin` file as JavaScript and export the typed array with `export default ...`. The result is written to stdout.

```shell
> caber ./file.bin --output-lang javascript --export-mode default
const binary = new Uint8Array([104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

export default binary;
```

## License

Fair Source License, Version 0.9  
Copyright © 2022 Gerald Nash
