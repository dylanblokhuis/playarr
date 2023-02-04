## Dependencies:
- rust >1.67
- mpv 0.35.0

## Compile for Windows

- download and extract libmpv - 2022-11-13 from sourceforge x86_64
- compile a mpv.lib with Visual Studio Build Tools:
```
lib.exe /def:mpv.def /name:mpv-2.dll /out:mpv.lib /MACHINE:X64
```
- create a new folder in the project root called "64"
- move mpv.lib into 64
- set env variable MPV_SOURCE with your project root as path:
```
set MPV_SOURCE=...<-- path here!
```
- drag mpv-2.dll into your debug/release folder
- and ur good to go