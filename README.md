# Dot-Image-CLI

Print image as dot in terminal

![Screenshot](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/dot-image/screenshot.png)

[![asciicast](https://asciinema.org/a/G8Cq4y0Ob76UDEokcZso5a7dN.svg)](https://asciinema.org/a/G8Cq4y0Ob76UDEokcZso5a7dN)

![Color](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/dot-image/color.jpg)

## Installation

```bash
$ cargo install --git https://github.com/Nguyen-Hoang-Nam/dot-image-cli
```

## Usage

By default, image will be converted to grayscale then binary colors and displayed.

```bash
$ dot-image -i path/to/image -w 100 -h 100
```

If you want the opposite color then use `-I` flag, stand for invert.

The experiment feature color dot image

```bash
$ dot-image -i path/to/image -w 100 -h 100 -c
```

## Support

- Image (png, jpg)
- Gif

## TODO

- [x] Support color
- [ ] Use Otsu's method to threshold
- [x] Auto scale image
- [x] Write file
- [ ] Support video

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
