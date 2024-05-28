# Rexout 
### Like [hexdump](https://www.gnu.org/software/grub/manual/grub/html_node/hexdump.html), but with Rust :crab: and stdout.

## Usage :wrench:
```bash
    rexout <file path> <options flags>
```

The program only contains two options flags
- --big  ->  show the words in big-endian style, default is little-endian 
- --no-count -> disable the byte count on the left

## Building

The program don't have any third party dependency, show just run:

```bash
    cargo build --release
```

## License
The project is under the [MIT](./LICENSE) License.
