# officedays

**officedays** is a simple command-line tool that calculates how many days need to be worked in the office each quarter for hybrid workers.

It reads from a yearly TOML configuration file and takes into account bank holidays and booked leave, assuming they count as days worked in the office. If you include future planned office days, it also provides a projection to see whether you’re on track to meet your target.

## Features

- Reads a per-year TOML configuration file from the system config directory  
  (e.g. `~/.config/officedays/2025.toml` on Linux or `~/Library/Application Support/officedays/2025.toml` on macOS)
- Calculates quarterly required office days based on leave and bank holidays
- Optional projection based on planned future days
- Cross-platform (supports Linux, macOS, and Windows)
- Editable config via `-e` flag using `$EDITOR` or defaults to `nano`
- If no config exists, `-e` creates a new config file and exits

## Usage

```bash
officedays [-e | -h]
```

## Options

- `-e`
Edit the TOML file for the current year using $EDITOR, or fall back to nano if unset. If no config files exists, a new one is created.

- `-h`
Show the help message.

Running the tool without any arguments will display a summary of your current quarter’s office day requirement, leave, bank holidays, days worked, and projection.

## Configuration

The program expects a TOML file named YYYY.toml (e.g. 2025.toml) stored in your system’s config directory. This file contains:

- The number of days required in the office per quarter
- Bank holidays per quarter
- Leave booked per quarter
- Days worked or planned per quarter

An example configuration is available at [example.toml](example.toml).

## Installation

Install using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo install officedays
```

## Licence

[MIT](LICENSE)

## Author

Written by [Laurence Stock-Tully](https://github.com/lst1000)
