# jojo-otp-app

This is a desktop application that enables users to connect and configure [jojo-client](https://github.com/gggiulio77/jojo-client) using their credentials.

### Quick Links

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [License](#license)

## Getting Started

Upon its initial startup, [jojo-client](https://github.com/gggiulio77/jojo-client) operates as a WiFi access point, establishing its own network. This application streamlines the configuration process for users, facilitating the following:

- Utilizing the OS WiFi module via [jojo-wifi-manager](https://github.com/gggiulio77/jojo-wifi-manager) to locate the [jojo-client](https://github.com/gggiulio77/jojo-client) network and establish a connection.
- Sending HTTP requests to the [jojo-client](https://github.com/gggiulio77/jojo-client) server to scan networks and store credentials.

<br>

Developed using [Tauri](https://tauri.app/) and incorporating [Leptos](https://leptos.dev/) along with [DaisyUI](https://daisyui.com/) as the frontend framework.

For now the project only runs on Windows.

### Prerequisites

Before proceeding, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

If you're new to [Tauri](https://tauri.app/), I recommend reading the [getting started guide](https://tauri.app/v1/guides/getting-started/prerequisites).

Once you've met all the prerequisites for [Tauri](https://tauri.app/), install [Tauri-CLI](https://tauri.app/v1/api/cli/) by running 

`cargo install tauri-cli`

### Installation

`git clone https://github.com/gggiulio77/jojo-otp-app.git`

## Usage

To initiate a development environment, execute `cargo tauri dev`. This command compiles and runs the backend in a shell while opening a [WebView](https://tauri.app/v1/references/webview-versions/) window for the frontend.

For creating an installer, use `cargo tauri build`.

Installers will be stored using a release tag.

## Roadmap

- [ ] Enhance error handling
- [ ] Make it iOS and Android compatible (https://beta.tauri.app/blog/roadmap-to-tauri-2-0/)
- [ ] Make it Linux and MacOs compatible
- [ ] Improve documentation

## License