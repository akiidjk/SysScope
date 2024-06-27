# SysScope ⛽ | Last docs update: 28/11/2023 (d/m/y)

[![Rust](https://github.com/Retr0100/VirgilBench/actions/workflows/rust.yml/badge.svg)](https://github.com/Retr0100/VirgilBench/actions/workflows/rust.yml)

## Introduction 📝

***SysScope*** is a simple software, written in Rust, designed to monitor and analyze the performance and resources used by specific processes running on your computer. This tool is indispensable for developers and system analysts

## Supported all platform
Rust is a compiled language like C or C++ and is possible to compile on all machine, [installing cargo and the compilar](https://www.rust-lang.org/tools/install) and running `cargo build` in the src directory.
In the release is avaible the executable for Windows and Linux

## Installation 📒

You can or download the executable from the last [release]() of repository or download the repository go in the ***src*** directory and run ```cargo build --release```

## How to use 📋

From line of command:

``` powershell
sysscope.exe [options]
```

## Option list & explation 📄

- `-h, --help`: Show an help message and exit.
- `-p, --pid`: The pid of process to monitoring ***(this value is obligatory)***
- `-V, --version`: Print the actual version of tool
- `-u, --update`: The time of update of tool ***(Every how many milliseconds the data is updated | deafult value is '200'ms)***
- `-d, --duration`: The duration of monitoring ***(default value is '0' and run the tool without time limit)***

## Demo 🔬

![demo_image](assets/Screenshot%202023-11-26%20112859.png)


### Contact me

For code related issues you can use github directly for other collaborations or alerts write to this email <projectvirgilai@gmail.com>

If you want to support a small developer take a [**special link**](https://www.paypal.me/Retr0jk)


<a href="https://www.paypal.com/paypalme/Retr0jk">
  <img width = 200 align="center" src="https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white" />
</a>
</div>

### Licence

- AGPL-3.0 licence
- [LICENSE FILE](https://github.com/Retr0100/VirgilAI/blob/master/LICENSE)
