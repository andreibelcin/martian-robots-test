# Martian Robots

## The Problem

The surface of Mars can be modelled by a rectangular grid around which robots are able to move according to instructions provided from Earth. You are to write a program that determines each sequence of robot positions and reports the final position of the robot. A robot position consists of a grid coordinate (a pair of integers: x-coordinate followed by y-coordinate) and an orientation (N, S, E, W for north, south, east, and west). A robot instruction is a string of the letters "L", "R", and "F" which represent, respectively, the instructions:

* Left: the robot turns left 90 degrees and remains on the current grid point.
* Right: the robot turns right 90 degrees and remains on the current grid point.
* Forward: the robot moves forward one grid point in the direction of the current orientation and maintains the same orientation.

The direction North corresponds to the direction from grid point (x, y) to grid point (x, y+1).

There is also a possibility that additional command types may be required in the future and provision should be made for this. 

Since the grid is rectangular and bounded (...yes Mars is a strange planet), a robot that moves "off" an edge of the grid is lost forever. However, lost robots leave a robot "scent" that prohibits future robots from dropping off the world at the same grid point. The scent is left at the last grid position the robot occupied before disappearing over the edge. An instruction to move "off" the world from a grid point from which a robot has been previously lost is simply ignored by the current robot.

### The Input

The first line of input is the upper-right coordinates of the rectangular world, the lower-left coordinates are assumed to be 0, 0. The remaining input consists of a sequence of robot positions and instructions (two lines per robot). A position consists of two integers specifying the initial coordinates of the robot and an orientation (N, S, E, W), all separated by whitespace on one line. A robot instruction is a string of the letters "L", "R", and "F" on one line. Each robot is processed sequentially, i.e., finishes executing the robot instructions before the next robot begins execution.

The maximum value for any coordinate is 50.
All instruction strings will be less than 100 characters in length.

### The Output

For each robot position/instruction in the input, the output should indicate the final grid position and orientation of the robot. If a robot falls off the edge of the grid the word "LOST" should be printed after the position and orientation.

## How to run

### Without Docker

Install Rust with rustup, which also installs Cargo. On macOS or Linux, run the official installer and follow its prompts:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download and run `rustup-init.exe` from the [official Rust installation page](https://www.rust-lang.org/tools/install). Choose the MSVC toolchain if prompted; the installer will guide you if Visual Studio C++ Build Tools are needed.

Restart your terminal after installation, then verify that both tools are available:

```sh
rustc --version
cargo --version
```
Build using:

```sh
cargo build --release
```

Run using:

```sh
cargo run --release -- --input-path sample_input.txt
```

Test by running:

```sh
cargo test
```

### With Docker

Install Docker Desktop (macOS/Windows) or Docker Engine (Linux), and make sure the daemon is running. From the project root, build the image:

```sh
docker build -t barclays-challenge .
```

Mount an input file read-only and pass its container path to the program. In macOS/Linux shells:

```sh
docker run --rm --mount "type=bind,source=$PWD/sample_input.txt,target=/input.txt,readonly" barclays-challenge --input-path /input.txt
```

In Windows PowerShell:

```powershell
docker run --rm --mount "type=bind,source=$((Resolve-Path .\sample_input.txt).Path),target=/input.txt,readonly" barclays-challenge --input-path /input.txt
```

The sample should print:

```text
1 1 E
3 3 N LOST
2 3 S
```

For another input file, replace `sample_input.txt` with its path. If Docker reports that the daemon is unavailable, start Docker Desktop and retry. On macOS, if it reports that `docker-credential-desktop` cannot be found, add Docker Desktop's bundled tools to `PATH` for the current shell and retry:

```sh
export PATH="/Applications/Docker.app/Contents/Resources/bin:$PATH"
```