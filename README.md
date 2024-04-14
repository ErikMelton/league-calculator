# League Simulator

## About

This project is a simulation of a game scenario written in Rust. It includes modules for creating champions, items, 
builds, scenarios, enhancements, and running simulations. The project is still under development and contributions are
welcome.

## Installation and Running

To install and run this project, you need to have Rust and Cargo installed on your system. Follow these steps:

1. Clone the repository: `git clone https://github.com/ErikMelton/your-repo-name.git`
2. Navigate to the project directory: `cd your-repo-name`
3. Build the project: `cargo build`
4. Run the project: `cargo run`

## Building Documentation

To build the documentation for this project, use the following command:

```bash
cargo doc --open
```

This will generate the documentation and open it in your web browser.

## TODO
 
- Implement the simulation module.
  - 30 TPS
  - Implement damage over time tick event
- Consider how to run simulations.
- Implement abilities
- Implement the ability rotation and hit chance for both the player and enemy in the scenario module.
- Consider auto cancels in the scenario module.
- Implement runes in the main module.
- Implement items
- Write a system to automate the creation and updating of champions, runes, abilities, and items.