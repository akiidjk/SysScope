# VirgilBench ⛽ | Last docs update: 26/11/2023 (d/m/y)

## Introduction 📝

***VirgilBench*** is a simple software, written in Rust, designed to monitor and analyze the performance and resources used by specific processes running on your computer. This tool is indispensable for developers and system analysts

## Installation 📒

You can or download the executable from the last [release]() of repository or download the repository go in the ***src*** directory and run ```cargo build --release```

## How to use 📋

From line of command:

``` powershell
virgilbench.exe [options]
```

## Option list & explation 📄

- `-h, --help`: Show an help message and exit.
- `-p, --pid`: The pid of process to monitoring ***(this value is obligatory)***
- `-V, --version`: Print the actual version of tool
- `-u, --update`: The time of update of tool ***(Every how many milliseconds the data is updated | deafult value is '200'ms)***
- `-d, --duration`: The duration of monitoring ***(default value is '0' and run the tool without time limit)***

## Demo 🔬

![demo_image](assets/Screenshot%202023-11-26%20112859.png)
