# Smooth resizing on Windows 11

This project implements **smooth resizing** on Windows 11, not by using any band-aid fixes, but by copycatting the internal UWP architecture reverse-engineered by [@apkipa](https://github.com/apkipa) in an [explanation](https://github.com/bigfatbrowncat/noflicker_directx_window/issues/1#issuecomment-2611975203) and [gist](https://github.com/bigfatbrowncat/noflicker_directx_window/issues/1#issuecomment-2611975203). This is written in pure Rust, outputs a single executable, and takes up around 650 lines of code.

People don't usually care if applications resize smoothly on Windows. My brother said he doesn't care what a window does while it's resizing, but I do. If anyone sees this repository, please feel free to test this project yourself and tell me how it goes :3

*This proof-of-concept is intended for later versions of Windows 11. Building the project in debug mode without the DirectX debug layer installed will result in an error. This executable may register as a Windows app. I have seen this flicker on other computers, but not on the latest version. This repository was originally modeled after [noflicker_directx_window](https://github.com/bigfatbrowncat/noflicker_directx_window) by [@bigfatbrowncat](https://github.com/bigfatbrowncat).*
