[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=plastic&logo=rust&logoColor=white)](https://www.youtube.com/watch?v=cE0wfjsybIQ&t=73s)

<div align="center">

# ReSplatted  🦑 🦑

**ReSplatted** is a Project created in rust. This project aim to be a fully customizable stress test tool. For now, you can stress tool to a 26.1.2 Minecraft server

</div>

## Project Architecture

The project is structured in differents crates to have a better modularity:

* **`resplatted`**: The main binary. Inside is the CLI and the virtual Minecraft client
* **`resplatted-protocol`**: The internal network processing library. It encapsulates the serialization/deserialization of Minecraft packets, I/O stream extensions, and the strict definition of protocol states.

## Prerequisites

* [Rust and Cargo](https://rustup.rs/) (stable version).
* *On Linux/WSL environments (e.g., Ubuntu): ensure you have the basic build tools installed (`sudo apt install build-essential`).*

## Installation OR Build

You can either get the project in the release section or clone the repository and build the project yourself.

```bash
git clone https://github.com/hthug06/ReSplatted.git
cd ReSplatted
cargo build --release -p resplatted
```

## Usage
Run the CLI tool with the target Minecraft server's address and port:

```bash
resplatted --address <ip_or_hostname> [OPTIONS]
```

### Available Options
| Option               |  Short   | Default      |                                                                                            Description                                                                                            |
|----------------------|:--------:|--------------|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
| ```--address```      |    -     | **Required** |                                                                                     The server IP or hostname                                                                                     |
| ```--port```         | ```-p``` | ```25565```  |                                                                                      The target server port                                                                                       |
| ```--status```       | ```-s``` | ```false```  |                                                        See infos about the targetted server like in the server list of a minecraft client                                                         |
| ```--bot_number```   | ```-b``` | ```1```      |                                                                               The number of bot sent to the server                                                                                |
| ```--wait```         | ```-w``` | ```1```      |                                                         The base waiting time (in ms). Used differently depending on the `waiting-mode.`                                                          |
| ```--waiting_mode``` | ```-```  | ```Linear``` | Waiting mode between each bot connection. `Linear`: delays each bot by (index * wait). `Static`: every bot waits exactly 'wait' ms. `Random`: every bot waits a random time between 0 and 'wait'. |


# Example
Retrieve complete information from a public server:

```bash
resplatted --address mc.hypixel.net --status
```
Output:
```
📌 Version : Requires MC 1.8 / 1.21 (Protocol version : 775)
👥 Players : 17024 / 200000
📝 MOTD    :                  Hypixel Network [1.8/26.1]
                            SB LOTUS ATOLL - BW DREAMFEAST
🖼️  Favicon : Saved in ./temp/mc.hypixel.net_favicon.png
```


# Performance & Benchmarks

ReSplatted is built with Rust and the `tokio` asynchronous runtime, designed to be extremely lightweight and blazingly fast. It uses asynchronous I/O and shared memory states (`Arc`) to spawn thousands of concurrent connections without overwhelming the host machine.

## The 1,000 Bots Benchmark
Tested on a local environment sending 1,000 concurrent bot connections to a local server. (Needed to use a rust server software like [SteelMC](https://steel-foundation.github.io/SteelDocs/) or [PumpkinMC](https://pumpkinmc.org/), java server like [PaperMC](https://papermc.io/) can't handle 1000 or can, but I don't have the money for a big server.)

**Test Environment:**

- **CPU:** AMD Ryzen 5 7520U (4 Cores / 8 Logical Processors @ 2.80 GHz)

- **Host RAM:** 16.0 GB

- **OS:** Linux Ubuntu (WSL2) - Kernel 6.6.114.1 (WSL Available RAM: 7.4 GB available)

- **Command:** ./resplatted --address localhost -n 1000 --waiting-mode linear -w 50

**Resource Footprint:**

- **Memory (RAM):** ~15.5 MB (Resident Set Size)

- **CPU Usage:** ~50% of a single core (out of 8 cores)

- **Cost per Bot:** ~15 KB of RAM per active connection

- **Note:** To achieve these results, ensure you compile and run the tool in release mode using cargo run --release. Debug mode will consume significantly more CPU and Memory.

## Other information
* This project is a way for me to learn rust and the Minecraft protocol.
* ReSplatted is a direct continuation of my previous project [Splatted](https://github.com/hthug06/Splatted). Go check it if you are interested
* I used IA for this project, not for coding but to think about the architecture. I want to avoid using IA for coding because I want to learn and understand the code I write.
* The project is still in early development, so expect some bugs and missing features. I will try to add more features in the future, but for now, I want to focus on the core functionality of fetching server information.
* If you have any suggestions or want to contribute, feel free to open an issue or a pull request (made suggestion about feature pls I'm so dumb idk what to make). I will be happy to review it and merge it if it's good.
* I don't really know the real objective for now, create a stress test tool or an entire Minecraft client in rust? I was also thinking of a chunk viewer (I'm too indecisive lol)

# I HOPE YOU LIKE THIS PROJECT
*Made with passion, because I was bored and also because I like Minecraft :)*