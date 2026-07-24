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
    *   Atomic Mass
    *   Electron Configuration
    *   Grouping info
    *   Physical Properties at STP
*   **Display the periodic table**
*   **Sort elements based on properties:**
    *   Alphabetical Order of name and symbol _(and inverse alphabetical order)_
    *   Increasing Order in Mass, Density, Boiling point, Melting point _(and decreasing)_
    *   etc

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

![YouTube](https://youtu.be/nsVz-hxfFCM)


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
