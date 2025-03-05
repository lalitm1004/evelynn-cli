# evelynn-cli
A cli tool designed for personal QoL improvements

## Current Features
- Manage all your SSH servers in one `config.toml` file
- Can fetch local IP from appwrite server (long store)

## Future Scope
- Add an LLM to talk to my obsidian vault

## Setup
1. Install the rust toolchain
2. Run
```
sudo apt install sshpass  
```
3. Clone this repository
```
git clone https://github.com/lalitm1004/evelynn-cli.git
cd evelynn-cli
```
4. Compile using `cargo build --release`
5. Move/Add built file to `PATH`
6. `evelynn-cli` will look for `config.toml` in `$HOME/.config/evelynn-cli/`

