# Pylos

This repository contains an implementation of the Pylos board game. Pylos is a strategic abstract board game where two players compete to place their spheres at the top of a pyramid. The game, designed by David G. Royffe, requires tactical thinking and careful planning to outmaneuver your opponent.

The game is hosted on a DigitalOcean droplet at http://142.93.162.12:8000.

<p align="center">
  <img src="https://raw.githubusercontent.com/GKerfImf/Pylos/main/resources/screenshot_main.png" width="500"/>
  <img src="https://raw.githubusercontent.com/GKerfImf/Pylos/main/resources/screenshot_game.png" width="500"/>
</p>

## How to play:

- **Setup:** The game is played on a 4x4 grid. Each player has 15 spheres, one set is light-colored, and the other is dark-colored.
- **Objective:** The goal is to be the first to place your sphere at the apex of the pyramid.
- **Gameplay:**
  - Players take turns placing one sphere on the grid or on top of other spheres to build a pyramid.
  - If a player forms a square with their spheres, they can remove one or two of their spheres from the board and reuse them later.
  - Players can also move their spheres to higher levels if possible, rather than placing new ones.
- **Endgame:** The game ends when the final sphere is placed at the top of the pyramid. The player who places the last sphere wins the game.

## Installation

To spin up a local version of the game, follow these steps (tested on `macOS Sonoma 14.6.1`, `rustc 1.79.0`, and `npm 10.2.5`):

1. Clone the repo.
   ```
   git clone https://github.com/GKerfImf/Pylos
   cd Pylos
   ```
2. Install NPM packages and create the production build of the frontend.
   ```
   cd client
   npm install
   npm run build
   cd ..
   ```
3. Install cargo packages and start the server.
   ```
   cd server
   cargo build --release
   cargo run --release
   ```
4. Visit `http://localhost:8000/`

## Technologies Used

Frontend: [`Typescript`](https://www.typescriptlang.org/) with [`React`](https://react.dev/), [`Threejs`](https://threejs.org/), and [`Tailwind-css`](https://tailwindcss.com/)

Backend: [`Rust`](https://www.rust-lang.org/) with [`Tokio`](https://tokio.rs/) and [`Warp`](https://github.com/seanmonstar/warp).
