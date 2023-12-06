# Discord ModHook

An in-memory mod loader for Discord.

# Note about 64-bit & 32-bit Discord.

This modloader currently only supports 64-bit Discord. If you aren't currently using the 64-bit version of Discord, make sure you download the 64-bit version from the Discord website.

You can check your architecture in Discord settings. Scroll to the bottom of the sidebar and look at "Host".
If it's 64-bit, it'll say x64.

# Usage

Download the latest version from the [Releases page](https://github.com/MeguminSama/ModHook/releases/latest), and extract it somewhere.
Make sure this directory doesn't change, as any shortcuts you make need to point to the executable.

If you right-click and drag `modhook.exe`, when you release the mouse button there will be an option to create a shortcut.
Alternatively, you can always make a shortcut manually.

If you are using a shortcut, the "Start In" directory must be the folder the ModHook executable is in.

## Example Usage: Vencord

Change the `-d` flag if you want to use a different Discord branch.

```
"C:\path\to\modhook.exe"
	-d "C:\Users\megu\AppData\Local\Discord"
	-m "c:\path\to\vencord\dist\patcher.js"
```

## Example Usage: Moonlight

```
"C:\path\to\modhook.exe"
	-d "C:\Users\megu\AppData\Local\Discord"
	-m "c:\path\to\moonlight\dist\injector.js"
```

## All configuration flags

Names prefixed with a \* are required.

| Flag        | Name             | Purpose                                                  | Example                                  |
| ----------- | ---------------- | -------------------------------------------------------- | ---------------------------------------- |
| -d          | \*Directory      | The path to the Discord AppData folder                   | -d "c:\Users\megu\AppData\Local\Discord" |
| -m          | \*Mod Entrypoint | The path to the Mod Entrypoint                           | -m "c:\path\to\vencord\dist\patcher.js"  |
| --moonlight | Moonlight        | Support for the Moonlight Mod                            | --moonlight                              |
| -f          | ASAR Filename    | The name of the _modded_ ASAR                            | -f "\_app.asar"                          |
| -t          | Toggle Query     | Toggle asar redirection when this query is hit           | -t "\injector.js"                        |
| -a          | Custom ASAR      | Use a custom ASAR instead of the modhook-provided one    | -a c:\path\to\my_app.asar                |
| -c          | Custom UserDir   | Use a custom user directory. Helpful for multi-instances | -c "my-custom-userdir"                   |
| -h          | Help             | Shows the Help command                                   | -h                                       |

### -d (\*Directory)

This is the path to your Discord AppData folder.

Usage: `-d <path>`

Common values are:

- `C:\Users\<username>\AppData\Local\Discord`
- `C:\Users\<username>\AppData\Local\DiscordPTB`
- `C:\Users\<username>\AppData\Local\DiscordCanary`
- `C:\Users\<username>\AppData\Local\DiscordDevelopment`

### -m (\*Mod Entrypoint)

This is the path to the entrypoint of the Discord mod.

Usage: `-m <path>`

Common values are:

- `C:\path\to\vencord\dist\patcher.js`
- `C:\path\to\moonlight\dist\injector.js`

### --moonlight (Moonlight)

Use this flag if you're loading Moonlight.

Moonlight's entrypoint works slightly differently to most, so this is needed to load it.

Usage: `--moonlight`

### -f (ASAR Filename)

The name of the modded ASAR.

Many mods rename the original ASAR to `_app.asar` or `app.orig.asar` or similar.
If your mod does not use one of these, you need to provide it with this flag.

Usage: `-f <asar name>`

### -t (Toggle Query)

The query to use to toggle ASAR redirection.

ModHook redirects all calls to `app.asar/index.js` to a custom asar that load the mod.

Once the mod is loaded, we want to disable this redirection so that Discord can load.

This is only needed if for some reason, the query should be something different to the mod entrypoint (-m).

Usage: `-t c:\my\custom\query`

### -a (Custom ASAR)

Custom ASAR to laod, instead of the default ModHook one.

If you want to use your own ASAR instead of the ModHook one, use this flag.

Usage: `-a c:\path\to\custom.asar`

### -c (Custom User Directory)

Allows you to have a custom User Directory for your instance.

This means that instead of being in `%AppData%\Discord`, it'll be in `%AppData%\DiscordModHook\AppData\<custom user dir>`.

This allows two things:

- Each custom directory will have it's own profile, so you can have multiple accounts on the same branch.
  - This means that you will have to log in for each custom User Directory you use.
- Each directory stores all the electron cache - this makes it safe to run multiple instances of the same branch.

# Setup

You'll need rust...

Run `cargo build` to build the project and the `app.asar` file.

If you're developing this project, you'll need to place the built app.asar in the root of this project.
You'll find it in the target/{release} folder.

# Meow

meow
