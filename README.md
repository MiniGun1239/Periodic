> **License Notice:**
> This program is free software: you can redistribute it and/or modify
> it under the terms of the GNU Lesser General Public License as published by
> the Free Software Foundation, either version 3 of the License, or
> (at your option) any later version.

# Periodic

[![Time Spent](https://hackatime.hackclub.com/api/v1/badge/U0AJWQ44PGQ/MiniGun1239/Periodic)](https://hackatime.hackclub.com/my/projects/Periodic)
[![Periodic](https://img.shields.io/badge/GitHub-Periodic-green?style=plastic)](https://www.github.com/MiniGun1239/Periodic)

> Fun with chemistry and statistics

Fun chem app ("Fun" in very, very deep quotes, and very subjective)

---

## Features

*   **Lookup:** Gives some basic information about elements like:
    *   Name
        * ```periodic <atomic_number> -n``` outputs name of element
    *   Symbol
        * ```periodic <atomic_number> -s``` outputs symbol of element
    *   Atomic Mass
        * ```periodic <atomic_number> -m``` outputs mass of element
    *   Electron Configuration
        * ```periodic <atomic_number> -e [Options]```
        * Options:
          * Bohr Config
          * Quantum Config
          * Semantic Config
          * Valency
          * First Ionization Energy
          * Electron Affinity
    *   Grouping info
        * ```periodic <atomic_number> -g [Options]```
        * Options:
          * Category
          * Group
          * Period
          * Block
    *   Physical Properties at STP
        * ```periodic <atomic_number> -p [Options]```
        * Options:
          * Boiling Point
          * Melting Point
          * Density
          * Phase
*   **Display the periodic table**
*   **Sort elements based on properties:**
    *   Alphabetical Order of name and symbol _(and inverse alphabetical order)_
    *   Increasing Order in Mass, Density, Boiling point, Melting point _(and decreasing)_

## Stack

*   **Frontend:**   Terminal
*   **Backend:**    Rust
*   **Database:**   json
*   **Styling:**    Vibez

---

## Getting Started

Follow these simple steps to get a local copy up and running.

### Prerequisites

None (Unless you want to build it from source)

### How to get this for yourselves:

**Download it**

#### Linux
Get the latest release from [GitHub](https://github.com/MiniGun1239/Periodic/releases)

>**Note:** The name of the executable will be "periodic-*", where * is the version number, 
> remember to type the full name when executing like ```./periodic-* -V``` , or rename it from "periodic-*" to "periodic"

> If you downloaded, most likely it is in the downloads directory, 
> so either move it to the home directory (/home/user/) or run ```cd ~/Downloads``` before
> doing ```./periodic```

Or download from command line, like this:

```shell
curl -L https://github.com/MiniGun1239/Periodic/releases/download/Release/periodic-1.0.0-x86_64-Linux -o periodic
chmod +x periodic
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./periodic -V
```

#### Windows

Not Supported

### Video demonstration:  

* [Demonstration](https://youtu.be/nsVz-hxfFCM)
* [How to Download](https://youtu.be/MZtM6yARDFw)

## Building from Source

1. **Pre-requisites:**  
   Need to install Rust

2. **Clone the repository:**
   ```shell
   git clone https://github.com/MiniGun1239/Periodic.git
   cd Periodic
   ```
   
3. **Build**:  
   Idk how... i just used the IDE, someone help,

   Probably just run:

   ```shell
   cargo build --release
   ```

4. **Done!:**  
   Now test the binary with: 
   ```shell
   ./periodic -V
   ```

## Contributors
*   **[![Me✨✨](https://img.shields.io/badge/GitHub-MiniGun1239-orange?style=plastic)](https://www.github.com/MiniGun1239)**
*   **TBA (no one else yet 🥹)**

> Coded and tested in Arch Linux, should work in any distro.


### [Examples](https://github.com/MiniGun1239/Periodic/tree/main/Examples)

#### Name

```shell
$ periodic -n 1
  Outputting for Element 1:
---
  Name: Hydrogen
---
```
```shell
$ periodic -n 6
  Outputting for Element 6:
---
  Name: Carbon
---
```
```shell
$ periodic -n 92
  Outputting for Element 92:
---
  Name: Uranium
---
```

#### Mass

```shell
$ periodic -m 1
  Outputting for Element 1:
---
  Average Mass: 1.008u
---
```
```shell
$ periodic -m 6
  Outputting for Element 6:
---
  Average Mass: 12.011u
---
```
```shell
$ periodic -m 92
  Outputting for Element 92:
---
  Average Mass: 238.028913u
---
```

#### Symbol

```shell
$ periodic -s 1
  Outputting for Element 1:
---
  Symbol: H
---
```
```shell
$ periodic -s 119
  Outputting for Element 119:
---
  Symbol: Uue
---
```
```shell
$ periodic -s 54
  Outputting for Element 54:
---
  Symbol: Xe
---
```

#### Electron

```shell
$ periodic -e help
List of Available Options:
Usage: periodic <ATOMIC_NUMBER> --electron [OPTIONS]
 ---
  h , help                 Prints help information

  bc, bohr-config          Prints the Bohr Configuration
  qc, quantum-config       Prints the Quantum Configuration
  sc, semantic-config      Prints the Semantic Configuration

  v , valence              Prints the Valence Information
  ie, ionization-energy    Prints the ionization Energy
  ea, electron-affinity    Prints the Electron Affinity

  a , all                  Prints all of the above
```
```shell
$ periodic 1 -e bc
  Outputting for Element 1:
---
  Bohr configuration:
  1
---
```
```shell
$ periodic 87 -e all
  Outputting for Element 87:
---
  Quantum Configuration:
  1s2 2s2 2p6 3s2 3p6 4s2 3d10 4p6 5s2 4d10 5p6 6s2 4f14 5d10 6p6 7s1
---
  Bohr configuration:
  2, 8, 18, 32, 18, 8, 1
---
  Semantic configuration:
  [Rn] 7s1
---
  First Ionization Energy:
  380 Joules
---
  Electron Affinity:
  46.89 Joules
---
  Valence Information:
  Number of electrons in last shell filled: 1
  Electrons in the last (7th) shell:  1
---
```

#### Physcial Properties

```shell
$ periodic -p help
List of Available Options:
Usage: periodic <ATOMIC_NUMBER> --physical [OPTIONS]
 ---
  h, help             Show this help message

  b, boiling-point    Prints boiling point of element
  m, melting-point    Prints melting point of element
  d, density          Prints density of element at STP
  p, phase            Prints phase of element at STP
```
```shell
$ periodic -p d 54
  Outputting for Element 54:
---
  Density at STP: 5.894 g/cm³
---
```
```shell
$ periodic -p all 86
  Outputting for Element 86:
---
  Boiling point: 211.5 Kelvin
---
  Melting point: 202 Kelvin
---
  Density at STP: 9.73 g/cm³
---
  Phase at STP: Gas
---
```

#### Grouping

```shell
$ periodic -g help
List of Available Options:
Usage: periodic <ATOMIC_NUMBER> --grouping [OPTIONS]
 ---
  h, help         Prints help information

  c, category     Prints category of element

  g, group        Prints group of element
  p, period       Prints period of element

  b, block        Prints block of element
```
```shell
$ periodic -g g 8
  Outputting for Element 8:
---
  Group: 16
---
```
```shell
  Outputting for Element 75:
---
  Category: transition metal
---
  Group: 7
---
  Period: 6
---
  Block: d
---
```

#### Table

```shell
$ periodic table
---
    H                                                           He
  [s-block]                                      [--- p-block ---]
    Li Be                                        B  C  N  O  F  Ne
    Na Mg       [--------- d-block ---------]    Al Si P  S  Cl Ar
    K  Ca       Sc Ti V  Cr Mn Fe Co Ni Cu Zn    Ga Ge As Se Br Kr
    Rb Sr       Y  Zr Nb Mo Tc Ru Rh Pd Ag Cd    In Sn Sb Te I  Xe
    Cs Ba       *  Hf Ta W  Re Os Ir Pt Au Hg    Tl Pb Bi Po At Rn
    Fr Ra      \#  Rf Db Sg Bh Hs Mt Ds Rg Cn    Nh Fl Mc Lv Ts Og

    Uue    ] --- hypothetical

                    [--------------- f-block ---------------]
      *Lanthanides: Ce Pr Nd Pm Sm Eu Gd Tb Dy Ho Er Tm Yb Lu
     \#Actinides:   Th Pa U  Np Pu Am Cm Bk Cf Es Fm Md No Lr
---
```

#### Logo

```shell
$ periodic --logo
 ____           _           _ _
|  _ \ ___ _ __(_) ___   __| (_) ___
| |_) / _ \ '__| |/ _ \ / _` | |/ __|
|  __/  __/ |  | | (_) | (_| | | (__
|_|   \___|_|  |_|\___/ \__,_|_|\___|
```
