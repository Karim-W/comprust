# comprust

Image Compressor CLI tool.

## Usage

```sh
comprust <SOURCE> <DESTINATION> <QUALITY> <SIZE>
```

| Argument    | Description                                                                     |
| ----------- | ------------------------------------------------------------------------------- |
| SOURCE      | Path to the image to compress **OR** a directory containing images to compress. |
| DESTINATION | Path to the directory where the compressed images will be saved.                |
| QUALITY     | <FLOAT> Quality of the compressed images. Must be between 0.0 and 100.0         |
| SIZE        | <FLOAT>Size of the compressed images. Must be between 0.0 and 1.0               |

## Example

```sh
comprust ./images ./compressed 80.0 0.5
```

## Installation

```sh
cargo install https://github.com/karim-w/comprust
```

## License

BSD-3-Clause

## Author

Karim W

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
