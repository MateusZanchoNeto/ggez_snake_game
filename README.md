# 🐍 Snake Game

A classic Snake game built from scratch in Rust using the awesome ggez game engine. This project is a demonstration of game loop management, state handling, and 2D graphics rendering in a Rust environment.

<img width="450" height="450" alt="image" src="https://github.com/user-attachments/assets/f7e5b11e-5072-4e37-8e7a-9467bc2283ab" />

---

## 🌟 Features

- **Classic Gameplay:** Control the snake to eat food and grow longer.
- **Game Over Detection:** The game ends if the snake collides with itself.
- **Responsive Controls:** Smooth and immediate snake movement with keyboard input.

---

## 💻 Tech Stack

- **[Rust](https://www.rust-lang.org/)**: The core programming language, chosen for its performance and safety.
- **[ggez](https://ggez.rs/)**: A lightweight and simple game framework for making 2D games in Rust.
- **[rand](https://crates.io/crates/rand)**: A Rust library for random number generation, used for placing food.

---

## 🚀 Getting Started

Follow these instructions to get a copy of the project up and running on your local machine.

### Prerequisites

You need to have **Rust** and **Cargo** installed on your system. If you don't have them, you can install them via [rustup](https://rustup.rs/).

```sh
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
````

You will also need to install the dependencies required by `ggez` for your specific operating system. Please follow the official ggez guide [here](https://www.google.com/search?q=https://ggez.rs/docs/guides/getting_started/%23setting-up).

### Installation

1.  **Clone the repository:**
    ```sh
    git clone [https://github.com/MateusZanchoNeto/ggez_snake_game.git](https://github.com/MateusZanchoNeto/ggez_snake_game.git)
    ```
2.  **Navigate to the project directory:**
    ```sh
    cd ggez_snake_game
    ```
3.  **Build and run the project:**
    Cargo will automatically handle all the dependencies.
    ```sh
    cargo run --release
    ```
    The `--release` flag is recommended for a smoother gameplay experience.

-----

## 🎮 How to Play

  - **Start the Game:** Launch the executable by running `cargo run`.
  - **Controls:**
      - Use the **Arrow Keys** (`↑`, `↓`, `←`, `→`) or **W, A, S, D** to control the direction of the snake.
  - **Objective:**
      - Navigate the snake to eat the red food blocks that appear on the screen.
      - Each piece of food you eat increases your score and the length of the snake.
  - **Game Over:**
      - Don't let the snake run into its own body\!
      - The game will end, and you can simply close the window and run the program again to restart.

-----

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
