# Learners Library - Leveraged Liabilities
A hackathon project made for CIS Hackathon 2024 (Apr 14 - Apr 20), centerred about using technology to improve the quality of education, that won the second place prize.

If you are interested, you can follow the [pitch slide deck here](https://docs.google.com/presentation/d/1XV8_BTMg2PNGryKLOX29FzrFqd5779yCPSfRvkDVnSk/edit#slide=id.g2cd065654bf_0_34).

![Rust Badge](https://img.shields.io/badge/Rust-000?logo=rust&logoColor=fff&style=flat) ![Vue.js Badge](https://img.shields.io/badge/Vue.js-4FC08D?logo=vuedotjs&logoColor=fff&style=flat) ![Unity Badge](https://img.shields.io/badge/Unity-FFF?logo=unity&logoColor=000&style=flat) ![C# Badge](https://img.shields.io/badge/C%23-512BD4?logo=csharp&logoColor=fff&style=flat) ![TypeScript Badge](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=fff&style=flat)

## Setup and Installation of Unity
1) Download unity hub and the LTS unity editor
2) Clone the github repo, (CLI, Github Desktop, GitKraken, the web interface)
3) Import the subdirectory, ARUnity/, as a project into unity hub
4) Test if everything works.

## Website frontend
The website is made with Vue, and currently lacks typescript strictness.
First install the packages,
```sh
npm i
```
To run the development testing
```
npm run dev
```
To build the website.
```
npm run build-only
```

## Backend
The backend makes use of rust, axum web, and a simple file directory, simply run
```sh
cargo build --release
```
and copy the binary in `/target/release/` to wherever you would like to execute and run the backend server at.

