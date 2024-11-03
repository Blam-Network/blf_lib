# BLF Lib

This repository consists of a code library and command-line interface built for reading and writing Blam! Files, used by various Halo games.

## Supported Games
- Halo 3
    - 12070.08.09.05.2031.halo3_ship (Title Update 2)


## Commands
### Build Title Storage
This command builds Title Storage files which are used by Halo for online features such as Matchmaking Playlists and Message of the Day messages.
Title Storage files are built from a provided folder of configuration files, [Blam Network's title storage configuration](https://github.com/Blam-Network/Blam-Title-Storage) can be used as an example, or if you have pre-built title storage files available, you can generate config from these files using the [Build Title Storage Config](#build-title-storage-config) command.
#### Arguments
1. Title Storage Configuration Folder
2. BLF Output Folder
3. Title Name
4. Title Version
#### Example Invocation
```console
$ blf_cli
  build-title-storage
  "~/Blam-Title-Storage/Halo 3/Release"
  ~/storage/title/tracked/12070
  "Halo 3"
  12070.08.09.05.2031.halo3_ship
```