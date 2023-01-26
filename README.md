## Compile for Windows

- download and extract libmpv - 2021-11-07 from sourceforge x86_64
- compile a mpv.lib with Visual Studio Build Tools:
```
lib.exe /def:mpv.def /name:mpv-1.dll /out:mpv.lib /MACHINE:X64
```
- create a new folder in the project root called "64"
- move mpv.lib into 64
- set env variable MPV_SOURCE with your path:
```
set MPV_SOURCE=...<-- path here!
```
- drag mpv-1.dll into your debug/release folder
- and ur good to go