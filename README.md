# LinuxRPC
LinuxRPC is a simple and customizable Discord RPC client made in Rust for Arch Linux (compatability for other distros coming soon).

It will cycle through images and messages you set every 10 seconds, as well as show what you are listening to on most media players.

## Contents
[Installing](#installing-coming-soon)

[Manual Build](#manually-building)

[Configuration](#configuration)
- [Variables](#variables)

[Setting up Application](#setting-up-the-application)
- [Getting Client ID](#getting-the-client-id)
- [Adding Pictures](#adding-pictures)
- [Choosing Media Player](#choosing-your-media-player)

[Running LinuxRPC](#running-linuxrpc)

## Installing (Coming Soon)

## Manually Building

### Arch Linux & Arch-based Distros
```bash
git clone https://github.com/Sinmysize/LinuxRPC.git
cd LinuxRPC
makepkg -si
```

After building, it is highly recommended you run `systemctl --user enable linuxrpc.service` as to ensure the program automatically runs on boot and through the CLI.

## Configuration

You can find the configuration file in `~/.config/LinuxRPC/config.rpc`.

Copy this config template into your config file and fill out your client ID

```
[clientId]
<YOUR CLIENT ID>

[icons]

[messages]

[default_icon]

[default_small_icon]

[default_icon_text]

[default_small_text]

[player]

```

### Variables

- **[clientId]**: The client ID you get from your application.

- **[icons]**: The big picture you see in the rich presence.
- **[messages]**: Any message you would like to display (Try keeping it under 20 characters). Each message should be separated by a new line.
- **[default_icon]**: The picture that will default as your icon if none is set.
- **[default_small_icon]**: The small picture in the bottom right corner of your icon. This will be the default picture that is set.
- **[default_icon_text]**: The text that appears when you hover over your icon.
- **[default_small_text]**: The text that appears when you hover over your small icon.
- **[player]**: The player you wish to be displayed (Ex: spotify, your browser, etc.)

## Setting Up the Application

In order to add pictures to your RPC client, you will first have to create an application on Discord and acquire the Client ID.

### Getting the Client ID
In order to get your Client ID, go to https://discord.com/developers/applications/ (Be sure to login if you have not done so).

Next, click `New Application` give it a name (This will be the name that shows up as your status). You will then be redirected to the application's page.

![alt text](screenshots/image-2.png)

Then, go to `OAuth2` on the side and under **Client Information**, you will find the Client ID

![alt text](screenshots/image.png)
![alt text](screenshots/image-1.png)

(The Client ID for this example is invalid)

Lastly, in your config file, paste your Client ID under **[clientId]** and run `linuxrpc run` in your terminal to test if it works. 

Example:
```
[clientId]
1127450784842387537 
```
If you get the message `Connected!`, then you have successfully created the application.

### Adding Pictures

In order to add pictures to your RPC Client, go to your application's page and go to the `Rich Presence` tab on the side.

![alt text](screenshots/image-3.png)

Next, under **Rich Presence Assets** click **Add Image(s)** and select any image you would want to appear in your RPC client.

![alt text](screenshots/image-4.png)

Then, give the image(s) a name (you will need these names later).

Lastly, in your config (or do `linuxrpc config`, `Edit Config` > `Add to config` > `icons` or `default_icon` or `default_small_icon`), add the name of each image you added.

Example (in config.rpc):
```
...

[icons]
icon1
icon2

[default_icon]
icon3

[default_small_icon]
icon4

...

```

Run `linuxrpc run` to test if it works. If you do everything correctly, it should display the image(s) you added.

![alt text](screenshots/image-5.png)

### Choosing Your Media Player

You can display what song you are listening to by setting what media player in the config.

First, choose which player you want to use. Any media player that playerctl can detect can work.

The name is case sensitive to the config so ensure you find the proper name of the player. To find out the name of the media player, do `playerctl -l` and choose the name of the player. In some cases for browsers, it may contain other things besides the name. An example of this can be found by Firefox where it will show `firefox.instance_...`.
This is not a problem as you can simply but `firefox` and it will still work. Feel free to create an issue if a media player does not work.

> **CAUTION** IF YOU SET THE PLAYER TO YOUR PLAYER, **IT WILL DISPLAY <u>ANY</u> VIDEO/SONGS YOU ARE LISTENING TO ON YOUR BROWSER! YOU HAVE BEEN WARNED!**

Next, in you config add the name of the player.

Example: 
```
...

[player]
spotify

...
```

Finally, run `linuxrpc run` to test if it works. If everything is done correctly, it should display what you are listening to on the RPC client.

![alt text](screenshots/image-6.png)

## Running LinuxRPC
When all your configuration is done and you want to let it run in the background, you can do two things:

### Command
Run `linuxrpc start` and the RPC client should run immediately.

### Systemd (Autostart)
Run `systemctl --user start linuxrpc.service --now` and the RPC client should run immediately.

### Reboot
Once you reboot your system, open Discord and see if it immediately starts running. (This should work if you ran `systemctl --user enable linuxrpc.service` prior)