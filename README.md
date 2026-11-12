# Smooth resizing on Windows 11

This project implements **perfectly smooth resizing** on Windows 11, not by using any band-aid fixes, but by copycatting the internal UWP architecture reverse-engineered by many developers. This is written in pure Rust, outputs a single executable, and takes around 550 lines of code.

People don't usually care if applications resize smoothly on Windows. My brother said he doesn't care what a window does while it's resizing, but I do. If anyone sees this repository, please feel free to test this project yourself and tell me how it goes :3

## How did this happen?

**[@apkipa](https://github.com/apkipa)** originally posted a reverse-engineered [explanation](https://github.com/bigfatbrowncat/noflicker_directx_window/issues/1#issuecomment-2611975203) of what UWP, the famous flickerless framework, does when resizing. I projected this idea to a working window in Rust with the help of a [gist](https://gist.github.com/apkipa/20cae438aef2a8633f99e10e0b90b11e) he provided. It worked on my computer, but piecing everything together seemed to miss a lot, as it didn't work on other computers.

Then recently, **[Conor Dockry (@cdock1029)](https://github.com/cdock1029)** posted [this video](https://github.com/microsoft/microsoft-ui-xaml/issues/5148#issuecomment-6139475689) in a longstanding WinUI issue, demonstrating perfect resizing with his detailed reverse-engineered method. He then made a [PR for Epic Games' RAD Debugger](https://github.com/EpicGames/raddebugger/pull/1033), implementing his smooth resizing method. It pointed out my exact development path, with the same bugs!

Much of this repository's code is directly ported from Dockry's PR, including the complicated message handling. If you want to support this, please also support him! I didn't reverse engineer anything useful for this repository :)
