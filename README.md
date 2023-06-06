# bilibili-extractor

This will extract downloaded videos inside Bilibili. This is the successor of [bilibili-tools](https://github.com/nanashi-1/bilibili-tools).

## Installation
Firstly, make sure that ffmpeg is installed. To check whether you have it run:

    which ffmpeg    # This should return the location of ffmpeg.

In case you don't have it installed:

### Arch

    pacman -S ffmpeg

### Ubuntu/Debian

    apt install ffmpeg

### Fedora

    dnf install https://download1.rpmfusion.org/free/fedora rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm
    dnf install ffmpeg

After installing `ffmpeg`, you can install the CLI tool. Find install intructions here, [bilibili-extractor-cli](https://github.com/nanashi-1/bilibili-extractor-cli#Installation).

## License

This project is licensed under the MIT License.