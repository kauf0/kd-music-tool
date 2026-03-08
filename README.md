# kd-music-tool

A GUI utility for adding custom music to [Keep Driving](https://store.steampowered.com/app/2509010/Keep_Driving/).

Drop your audio files, set metadata, hit GO and listen to your farourite music ingame!

## What it does

- Inject tracks directly into the game's audio system with use of UndertaleModTool
- Tracks appear on the radio immediately (no new game required)
- You can add whatever tracks you like (if they're mp3, ogg, wav, flac, aac or m4a)!
- You can also remove individual tracks or reset all custom music back to vanilla at any time

## Requirements

- [ffmpeg](https://ffmpeg.org/download.html) installed and available in PATH
- [Keep Driving](https://store.steampowered.com/app/2509010/Keep_Driving/) game

## Download

Download the latest version from [Releases](../../releases):

- **Windows:** `kd-music-tool-windows.zip`
- **Linux:** `kd-music-tool-linux.zip`

Extract the zip wherever you like and run the `kd-music-tool.exe`. *This tool uses UndertaleModTool under the hood, it is bundled inside already.*

## Usage

1. Launch kd-music-tool
2. Click **BROWSE** or the path field to select your Keep Driving game folder
3. Drag audio files into the **QUEUE** panel, or click **+ ADD**
4. Click a track to edit its metadata: artist, album, title, track number
5. Click **GO** to add all queued tracks to your game
6. Launch Keep Driving and enjoy!

To remove a track, click **x** next to it in the **IN GAME** panel.  
To restore the game to its original state, click **RESET** in the IN GAME panel.

## Notes

- kd-music-tool creates a backup of `data.win`, `audiogroup3.dat`, and `music.kdr` on first run. RESET restores from this backup
- Do **not** delete `.bak` files from your game folder if you want to be able to RESET in case something breaks
- Only tested on Keep Driving v1.3.1.1d

## Support

If you find this useful, feel free to send tips <3

<a href="https://dalink.to/kauf0"><img src="https://img.shields.io/badge/Dalink-458a4d.svg?logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iMzgiIGhlaWdodD0iNDAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgZmlsbD0ibm9uZSI+CgogPGc+CiAgPHRpdGxlPkxheWVyIDE8L3RpdGxlPgogIDxwYXRoIGlkPSJzdmdfMSIgZmlsbD0iI2Y3ZjdmNyIgZD0ibTI1LjMxNzQsMGMwLjQ5NzEsMC4wMDAwNyAwLjk0NSwwLjIwMDYzIDEuMjkzLDAuNjAxNTZsNy4wNTk1LDguMjcwNTFjMC4yNDg2LDAuMzUwODcgMC4zOTgzLDAuODAyMTMgMC4yOTg5LDEuMjAzMTNsLTEuMDQ0LDEyLjI4MDNjLTAuMDQ5NywwLjQwMDggLTAuMTk5LDAuNzUyIC0wLjQ5NzEsMS4wNTI3bC04LjcwMjEsOC41MjE1Yy0wLjI5ODMsMC4zNTA4IC0wLjY5NjEsMC41MDEgLTEuMTQzNiwwLjUwMWwtOS4yNDgsMGwtOC4xNTQzMSw3LjU2OTNsMC42NDY0OCwtNy41NjkzbC00LjEyNjk1LDBjLTAuNDk2ODgsLTAuMDAwMSAtMC44OTQzMSwtMC4yMDAyIC0xLjI0MjE5LC0wLjU1MDhjLTAuMzQ4MDUsLTAuMzUwOSAtMC40OTc5NiwtMC44NTI2IC0wLjQ0ODI0LC0xLjMwMzdsMi41MzYxMywtMjkuMDcyMjljMC4wOTk0NCwtMC44NTIxMiAwLjc5NTQ2LC0xLjUwMzkgMS42OTA0MywtMS41MDM5MWwyMS4wODIwNSwwem0tMTYuNTQ4ODUsMTUuNjgwN2MtMS4xNDExNCwxLjE0MTEgLTEuNzY5NDIsMi42NTk1IC0xLjc2OTUzLDQuMjc0NGMwLDEuNjE1MSAwLjYyODI5LDMuMTM0MSAxLjc2OTUzLDQuMjc1NGMxLjE0MTE2LDEuMTQxMSAyLjY1OTU1LDEuNzY5NCA0LjI3NDQ1LDEuNzY5NWMxLjYxNTEsMCAzLjEzMjksLTAuNjI4MyA0LjI3NTQsLTEuNzY5NWwyLjYxNjIsLTIuNjE2MmwtMS42Nzk3LC0xLjY3OTdsLTIuNjE2MiwyLjYxNDJjLTEuMzg3LDEuMzg1NyAtMy44MDIyLDEuMzg2OCAtNS4xOTA1LDBjLTAuNjkzNDksLTAuNjkyMyAtMS4wNzQxOCwtMS42MTQgLTEuMDc0MTgsLTIuNTkzN2MwLjAwMDExLC0wLjk4MDggMC4zODA3OSwtMS45MDEzIDEuMDc0MTgsLTIuNTk0N2wyLjYxNjMsLTIuNjE2M2wtMS42Nzk3LC0xLjY3OTZsLTIuNjE2MjUsMi42MTYyem0zLjMyODE1LDMuNTQybDEuNjc5NywxLjY3ODdsNy4xMjUsLTcuMTI1bC0xLjY3ODcsLTEuNjc4N2wtNy4xMjYsNy4xMjV6bTcuODU4NCwtMTIuMjIzNjhjLTEuNjE0OSwwLjAwMDA2IC0zLjEzMjEsMC42Mjg0NSAtNC4yNzQ0LDEuNzY5NTNsLTIuNjE2MiwyLjYxNjI1bDEuNjc4NywxLjY3OTdsMi42MTYyLC0yLjYxNDNjMC42OTM1LC0wLjY5NDY4IDEuNjE0OCwtMS4wNzYxMiAyLjU5NTcsLTEuMDc2MThjMC45Nzk3LDAgMS45MDIxLDAuMzgwNjkgMi41OTU3LDEuMDc0MThjMC42OTM0LDAuNjkzNiAxLjA3NDIsMS42MTYxIDEuMDc0MiwyLjU5NTdjLTAuMDAwMSwwLjk4MDcgLTAuMzgwOSwxLjkwMTQgLTEuMDc0MiwyLjU5NDhsLTIuNjE2MiwyLjYxNjJsMS42Nzg3LDEuNjc5N2wyLjYxNzIsLTIuNjE2MmMxLjE0MSwtMS4xNDEyIDEuNzY5NCwtMi42NTk2IDEuNzY5NSwtNC4yNzQ1YzAsLTEuNjE1IC0wLjYyODQsLTMuMTM0MTEgLTEuNzY5NSwtNC4yNzUzNWMtMS4xNDEzLC0xLjE0MTI0IC0yLjY2MDMsLTEuNzY5NTMgLTQuMjc1NCwtMS43Njk1M3oiLz4KICA8cGF0aCBpZD0ic3ZnXzIiIGZpbGw9IiNGQzgxMDgiIGQ9Im00Ny4wMjM3LDI2bC00LjcwNDYsMGwyLjE3MzMsLTEzLjA5MDlsNC42MTUxLDBjMS4zMTY3LDAgMi40MDk4LDAuMjY4NSAzLjI3OTEsMC44MDU0YzAuODczNiwwLjUzNjkgMS40ODkzLDEuMzA2MSAxLjg0NzMsMi4zMDc1YzAuMzU3OSwwLjk5NzIgMC40MjE5LDIuMTg4MiAwLjE5MTcsMy41NzMyYy0wLjIyMTUsMS4zMzggLTAuNjYyNiwyLjQ4NjUgLTEuMzIzMSwzLjQ0NTNjLTAuNjYwNSwwLjk1NDUgLTEuNTA2NCwxLjY4NzUgLTIuNTM3NiwyLjE5ODhjLTEuMDMxMywwLjUwNzEgLTIuMjExNywwLjc2MDcgLTMuNTQxMiwwLjc2MDd6bS0xLjU0MDUsLTIuMzcxNGwxLjcwMDMsMGMwLjgyMjQsMCAxLjU0MjYsLTAuMTUzNSAyLjE2MDUsLTAuNDYwM2MwLjYyMjEsLTAuMzA2OCAxLjEzMTQsLTAuNzc5OCAxLjUyNzcsLTEuNDE5YzAuNDAwNSwtMC42MzkyIDAuNjgzOSwtMS40NTc0IDAuODUwMSwtMi40NTQ2YzAuMTU3NywtMC45NDYgMC4xNDcsLTEuNzEzIC0wLjAzMTksLTIuMzAxMWMtMC4xNzQ4LC0wLjU5MjMgLTAuNTEzNSwtMS4wMjQ5IC0xLjAxNjQsLTEuMjk3NmMtMC41MDI4LC0wLjI3NyAtMS4xNjMzLC0wLjQxNTUgLTEuOTgxNSwtMC40MTU1bC0xLjgyMTcsMGwtMS4zODcxLDguMzQ4MXptMTEuNDQ5NywyLjM3MTRsLTIuOTY1OSwwbDYuNjkyNSwtMTMuMDkwOWwzLjU2NjgsMGwyLjMzOTQsMTMuMDkwOWwtMi45NjU5LDBsLTEuNTkxNiwtMTAuMDk5NGwtMC4xMDIzLDBsLTQuOTczLDEwLjA5OTR6bTAuNjc3NiwtNS4xNDU2bDcuMDA1NywwbC0wLjM3MDgsMi4xNjA1bC03LjAwNTcsMGwwLjM3MDgsLTIuMTYwNXoiLz4KICA8cGF0aCBpZD0ic3ZnXzMiIGZpbGw9IndoaXRlIiBkPSJtNjguMTQxNCwyNmwyLjE3MzMsLTEzLjA5MDlsMi43Njc3LDBsLTEuNzg5NywxMC44MDg5bDUuNjEyMiwwbC0wLjM4MzUsMi4yODJsLTguMzgsMHptMTUuMTUzOSwtMTMuMDkwOWwtMi4xNzMzLDEzLjA5MDlsLTIuNzY3NywwbDIuMTczMywtMTMuMDkwOWwyLjc2NzcsMHptMTMuMjI2OCwwbC0yLjE3MzMsMTMuMDkwOWwtMi4zOTA3LDBsLTQuMzE0NiwtOC4yMzkzbC0wLjEwMjMsMGwtMS4zNzQzLDguMjM5M2wtMi43Njc3LDBsMi4xNzMzLC0xMy4wOTA5bDIuNDI5LDBsNC4yOTU0LDguMjMyOWwwLjEwODcsMGwxLjM2MTUsLTguMjMyOWwyLjc1NSwwem0wLjExMzQsMTMuMDkwOWwyLjE3MzMsLTEzLjA5MDlsMi43NjgyLDBsLTAuOTUzLDUuNzcybDAuMTczLDBsNS42NjMsLTUuNzcybDMuMzE4LDBsLTUuODExLDUuODYxNWwzLjY5NSw3LjIyOTRsLTMuMzExLDBsLTIuNjcyLC01LjM4MjFsLTEuNjQ5LDEuNjc0N2wtMC42MjY3LDMuNzA3NGwtMi43Njc4LDB6Ii8+CiA8L2c+Cjwvc3ZnPg==&style=for-the-badge" width="200"/></a>

I would also appreciate if you like, fav and comment on Steam!

## Building from source

Requires [Node.js](https://nodejs.org) 22+, [Rust](https://rustup.rs), and [ffmpeg](https://ffmpeg.org/download.html).

```bash
git clone https://github.com/kauf0/kd-music-tool.git
cd kd-music-tool
npm install
```

Place UndertaleModTool CLI binaries in `src-tauri/UTMT/`, then:

```bash
npm run tauri dev
```

## Third-party

This tool bundles [UndertaleModTool](https://github.com/UnderminersTeam/UndertaleModTool), licensed under [GPL v3](https://www.gnu.org/licenses/gpl-3.0.html).

## License

GPL v3