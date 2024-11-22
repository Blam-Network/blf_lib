# BLF Lib

This repository consists of a code library and command-line interface built for reading and writing Blam! Files, used by various Halo games.

# BLF Command-Line Interface

## Supported Titles and Versions
- Halo 3
  - 12070.08.09.05.2031.halo3_ship (Title Update 2)
- Halo 3: ODST
  - 13895.09.04.27.2201.atlas_release

## General Commands
### Convert Halo 3 MCC Map Variants
This command converts Halo 3 MCC map variants to Xbox 360.
The output files must be injected into an Xbox 360 console package in order to be loaded by the console, this can be done using [Horizon](https://www.wemod.com/horizon).
#### Arguments
1. MCC Maps Folder
   - This is usually at: `C:\Users\<user>\AppData\LocalLow\MCC\LocalFiles\<xuid>\Halo3\Map`
2. Converted Output Folder
#### Example Invocation
```console
$ blf_cli
  convert-h3mcc-map-variants
  C:\Users\john\AppData\LocalLow\MCC\LocalFiles\000901fc3fd9fe71\Halo3\Map
  "C:\Users\john\Desktop\Halo 3 Maps"
```

## Title Storage Commands
### Build
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
  title-storage
  build
  "~/Blam-Title-Storage/Halo 3/Release"
  ~/storage/title/tracked/12070
  "Halo 3"
  12070.08.09.05.2031.halo3_ship
```
---
### Build Title Storage Config
This command creates configuration files which can be used to manage Halo's Title Storage BLF files. Once you have folder of configuration files, you can build new Title Storage BLF files using the [Build Title Storage](#build-title-storage) command.
#### Arguments
1. BLF Input Folder
2. Title Storage Configuration Folder
3. Title Name
4. Title Version
#### Example Invocation
```console
$ blf_cli
  title-storage
  build-config
  ~/storage/title/tracked/12070
  "~/Blam-Title-Storage/Halo 3/Release"
  "Halo 3"
  12070.08.09.05.2031.halo3_ship
```
---
### Import RSA Signatures
This command imports RSA signatures from Halo `.map` files into the provided Title Storage Configuration folder. These RSA signatures are used for generating the `rsa_manifest.bin` file, and validating built map variant files.
This command is rarely used, the RSA signatures never change for a given Title and Version.
#### Arguments
1. Title Storage Configuration Folder
2. BLF Output Folder
3. Title Name
4. Title Version
#### Example Invocation
```console
$ blf_cli
  title-storage
  import-rsa-signatures
  "~/Blam-Title-Storage/Halo 3/Release/default_hoppers"
  "~/Halo 3/maps"
  "Halo 3"
  12070.08.09.05.2031.halo3_ship
```
---
### Import Variant
This command imports a Game or Map variant into the provided Title Storage Configuration folder. Behaviour may vary by selected Title and Version.
#### Support
- Halo 3 12070.08.09.05.2031.halo3_ship
  - Variants from Xbox 360 Console packages.
  - Variants from Halo The Master Chief Collection
    - Maps built with MCC exclusive objects or exceeding Halo 3's object limit will have objects removed.
  - Variants from pre-built Title Storage files (packed variants).
#### Arguments
1. Title Storage Configuration Folder
2. Variant Path
3. Title Name
4. Title Version
#### Example Invocation
```console
$ blf_cli
  title-storage
  import-variant
  "~/Blam-Title-Storage/Halo 3/Release/default_hoppers"
  ~/variants/cavern
  "Halo 3"
  12070.08.09.05.2031.halo3_ship
```
---
### Export Variant
This command exports a Game or Map variant from a Title Storage Configuration folder. Behaviour may vary by selected Title and Version.
#### Support
- Halo 3 12070.08.09.05.2031.halo3_ship
  - Exported variants can be inserted into an Xbox 360 Console file using Horizon with "Contents > Replace".
#### Arguments
1. Variant JSON file path.
2. Exported variant path.
3. Title Name
4. Title Version
#### Example Invocation
```console
$ blf_cli
  title-storage
  export-variant
  "~/Blam-Title-Storage/Halo 3/Release/default_hoppers/map_variants/atlas_eightygrit.json"
  ~/variants/atlas_eightygrit.bin
  "Halo 3"
  12070.08.09.05.2031.halo3_ship
```
