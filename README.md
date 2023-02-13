# Playarr
[<img src="https://img.shields.io/github/v/release/dylanblokhuis/playarr" /> <br /><br />](https://github.com/dylanblokhuis/playarr/releases)
Watch all your media from [Radarr](https://github.com/Radarr/Radarr) and [Sonarr](https://github.com/Sonarr/Sonarr) 

## Server (docker)
Pull the docker image (``amd64`` and ``arm64`` are supported) and configure the volumes just like ``Radarr`` and ``Sonarr`` so they match. 

Below is how an entry in your ``docker-compose.yml`` could look like:
```yaml
version: "3.4"

services:
  playarr:
      container_name: playarr
      image: ghcr.io/dylanblokhuis/playarr-server:master
      restart: unless-stopped
      ports:
        - 8000:8000
      environment:
        - SONARR_ADDRESS=http://<host>:8989
        - SONARR_API_KEY=
        - RADARR_ADDRESS=http://<host>:7878
        - RADARR_API_KEY=
      volumes:
        - <path_to_tv>:/tv
        - <path_to_movies>:/movies
  ```

  ## Client
  The binaries published in [releases](https://github.com/dylanblokhuis/playarr/releases) are without the "mpv (>0.35.0)" dependency. So you need to install it yourself:

  - <b>Windows:</b> mpv-2.dll for windows https://sourceforge.net/projects/mpv-player-windows/files/libmpv/ and place it near the executable or in ``PATH``
  - <b>MacOS</b> Just run ``homebrew install mpv``
  - <b>Ubuntu</b> 
  ```
  sudo add-apt-repository ppa:savoury1/ffmpeg4
  sudo add-apt-repository ppa:savoury1/ffmpeg5
  sudo add-apt-repository ppa:savoury1/mpv
  sudo apt-get update
  sudo apt-get install libmpv2 mpv
  ```
  - <b>Arch</b>
  ```
  pacman -S mpv
  ```

  
