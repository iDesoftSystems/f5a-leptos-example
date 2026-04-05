# F5a Leptos

This repository contains the sample code developed during the **Front End Development with Rust** course.

## Video Lessons

The course video tutorials are available on YouTube:

- Italiano: [Guarda la Playlist su YouTube](https://www.youtube.com/playlist?list=PLSLcKcqBWfjLfIB8_jFMWCwVOQ08ZNucc)

## Curriculum Overview

- Module 1: Introduction to Leptos and the Client-Side Stack
- Module 2: Setting up the Development Environment with Trunk & Tailwind CSS
- Module 3: Leptos Fundamentals: Components and View Macros
- Module 4: Fine-Grained Reactivity and Signals
- Module 5: Routing and Advanced State Management
- Module 6: Form Handling and Server Interaction (Data Fetching)

## Getting started from scratch

1. Install [Rust](https://www.rust-lang.org/learn/get-started)
2. Install [Trunk](https://github.com/trunk-rs/trunk)

    `cargo install trunk`

3. Add WebAssembly support

    `rustup target add wasm32-unknown-unknown`

4. Install [Just](https://github.com/casey/just)

   `cargo install just`

5. Launch the development server

   `just start`

6. Open the app, navigate to <http://localhost:8080>.

## Tech Stack

- **Leptos**: A cutting-edge, high-performance Rust framework.
- **Tailwind CSS**: For modern, utility-first styling.
- **Trunk**: To manage the WASM build pipeline.
- **Just**: For workflow efficiency.
