# Smooth resizing on Windows 11

This project implements **absolutely perfect smooth resizing** on Windows 11, not by using any band-aid fixes, but by copycatting the internal UWP architecture reverse-engineered by [@apkipa](https://github.com/apkipa) in an [explanation](https://github.com/bigfatbrowncat/noflicker_directx_window/issues/1#issuecomment-2611975203) and [gist](https://github.com/bigfatbrowncat/noflicker_directx_window/issues/1#issuecomment-2611975203). This is written in pure Rust, outputs a single executable, and takes up less than 600 lines of code.

People don't usually care if applications resize smoothly on Windows. My brother said he doesn't care what a window does while it's resized, but I do. I was shocked the resizing actually worked when I combined all these ideas! If anyone sees this repository, please feel free to test this project yourself and tell me how it goes :3
