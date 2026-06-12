## sys2dis - System to Discord
<img width="300" alt="ok" src="https://github.com/user-attachments/assets/92aa435e-5d0f-496c-bfd5-65dfe5c4679c" />

\
Have you ever thought "it would be cool if my friends on Discord could see my system resources for no apparent reason..."? \
Well, `sys2dis` is the solution!

## What is sys2dis?
`sys2dis` is a CLI for displaying your system resource usage via Discord RPC. With it, you can now show the world that... well... you're using a PC.

Features include:
- CPU usage.
- CPU temperature.
- RAM and swap usage.
- Configuration for changing App ID, etc. (`~/.config/sys2dis.toml`).

## Installation
1. Get the latest executable in [Release](https://github.com/kepalakubik/sys2dis/releases/latest).
2. Run `./sys2dis` for the first time to generate the configuration file and then press `Ctrl + C`.
3. Change the `app_id` value in the `~/.config/sys2dis.toml` file with the Application ID you created in the [Discord Developer Portal](https://discord.com/developers/applications).
4. Run `./sys2dis` again and you are good to go!

## Autostart
`sys2dis` can be run automatically via Systemd. Here's how:
1. Move the `sys2dis` executable into the `~/.local/bin` directory.
2. Download [sys2dis.service](https://github.com/kepalakubik/sys2dis/blob/main/systemd/sys2dis.service) and move it to `~/.config/systemd/user`.
3. Then run the following commands in the terminal:
   ```
   sudo systemctl daemon-reload
   systemctl --user enable --now sys2dis.service
   ```
4. Make sure the service is running properly by running `systemctl --user status sys2dis.service`.

## Credits
- Claude
- Gemini
- ChatGPT
- Deepseek

(yep, 90% of the code is written by AI cuz i actually can't do Rust)
