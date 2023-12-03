# LoRa PHY Embedded Linux (Raspberry Pi)

This project is a Rust-based implementation of a LoRa physical layer (PHY) for embedded Linux devices. It provides a low-level interface for transmitting and receiving LoRa signals, allowing developers to build LoRa-based applications on Linux platforms.

## Getting Started

To get started with the LoRa PHY Embedded Linux project, follow these steps:

### Prerequisites

Before running the project, make sure you have the following software and dependencies installed:

- Rust 
- Linux (or WSL) and install the linker for Raspberry OS 64bit
    ```shell
    sudo apt-get install gcc-aarch64-linux-gnu
    ```
- LoRa hardware module (e.g., SX1276)
- Raspberry Pi 4
- Raspberry OS 64 Bit

### Installation

1. Clone the repository:

    ```shell
    git clone https://github.com/IhsenBouallegue/lora-phy-embedded-linux.git
    ```

2. Change into the project directory:

    ```shell
    cd lora-phy-embedded-linux
    ```

3. Build the project:

    ```shell
    cargo build
    ```
   Alternatie you can build and deloy the project to raspberry pi with ssh:

    ```shell
    ./deploy {TARGET_ADDRESS}
    ```
    This will build and deploy to node@TARGET_ADDRESS.local  
    You can change the username in the bash script.




