<p align="center">
    <img width="512" alt="bilibili-extractor logo" src="logo.png">
</p>

# bilibili-extractor

[![Tests](https://github.com/nanashi-1/bilibili-extractor/actions/workflows/test.yml/badge.svg)](https://github.com/nanashi-1/bilibili-extractor/actions/workflows/test.yml)

This tool extracts and compiles downloaded content from bilibili. This is the successor of [bilibili-tools](https://github.com/nanashi-1/bilibili-tools).



## The Problem

Downloaded content from bilibili doesn't get saved as one file. It saves the audio, video, and subtitles separately. And they aren't saved with human-readable names. They are saved with their IDs as their name. Although the subtitle has been in SSA format recently, there are still older uploads in the proprietary JSON subtitle format by bilibili.

<details>
<summary>Example of the structure</summary>

```
download/
└── s_34641
    ├── 342865
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── 35813381d9f4a5f623e052af678072aca867054e.json
    │   └── entry.json
    ├── 346992
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── 7678f3b1b74e728ccf8a301ac36bc2a440e2a983.json
    │   └── entry.json
    ├── 350165
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── fae1bea747dfdbba0bc9734cbf36b66ee44ef602.json
    │   └── entry.json
    ├── 355184
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── 95422040af142e903cf3501f1efbbd4d650f0788.json
    │   └── entry.json
    ├── 358304
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── 38bb9688fba53f2b25b860cbf80fccdd4bfefb9e.json
    │   └── entry.json
    ├── 359293
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── 09d37641f164d657bf139b7ac6b6721fb8c22342.json
    │   └── entry.json
    ├── 359295
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── b880308196740c89f6a591d51171f3586799a69a.json
    │   └── entry.json
    ├── 359296
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── 87829ca0dda97d9356ccd07cc9f8b642f85ec608.json
    │   └── entry.json
    ├── 359297
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── 4885e942e5c0731bff07b78ab56d696e08030b4b.json
    │   └── entry.json
    ├── 359298
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── 45ccbcee9c73355dc658603a8a673cf8b4726f34.json
    │   └── entry.json
    ├── 359300
    │   ├── 112
    │   │   ├── audio.m4s
    │   │   ├── index.json
    │   │   └── video.m4s
    │   ├── en
    │   │   └── bb6e5a10c0c917c9ed23bfb4cff66cd048708696.json
    │   └── entry.json
    └── 359301
        ├── 112
        │   ├── audio.m4s
        │   ├── index.json
        │   └── video.m4s
        ├── en
        │   └── f9171d78e6aa73b2079d15ceb9b5e4f9139cd157.json
        └── entry.json
```
</details>

## The Solution

Firstly, the subtitle problem, a program that can translate it to other formats compatible with FFmpeg, is employed. The files are then compiled into one `.mkv` file using FFmpeg. Folders are then created to store the `.mkv` files about to be extracted. The folders are named based on the entry title. Then the `.mkv` files are renamed and moved to their specified folder.

## Dependencies

Firstly, make sure that `ffmpeg` is installed. To check whether you have it run:

    which ffmpeg    # This should return the location of ffmpeg.

In case you don't have it installed:

### Arch

    pacman -S ffmpeg

### Ubuntu/Debian

    apt install ffmpeg

### Fedora

    dnf install https://download1.rpmfusion.org/free/fedora rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm
    dnf install ffmpeg


## Installation

    cargo install bilibili-extractor-cli

## License

This project is licensed under the MIT License.
