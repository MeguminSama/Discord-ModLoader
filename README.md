# Discord ModHook

An in-memory mod loader for Discord.

# Note about 64-bit vs 32-bit Discord.

Discord recently released x64 builds of all their clients.
For this program to work, it must be compiled for the same architecture that your Discord runs on.

If you go into account settings and scroll to the bottom of the sidebar, you should see `Host x.x.xxx x64` or `Host x.x.xxx x86`.

Make sure to get the right build for this architecture.

# Usage

Download the latest version from the Releases page, and extract it somewhere.
Make sure this directory doesn't change, as any shortcuts you make need to point to the executable.

If you right-click and drag `modhook.exe`, when you release the mouse button there will be an option to create a shortcut.
Alternatively, you can always make a shortcut manually.

## Commandline Arguments

As an example of what I use...

```
"C:\Users\megu\Workspace\Discord\ModHook\ModHook-RS\target\release\modhook.exe" -d "C:\Users\megu\AppData\Local\Discord" -m "c:\users\megu\workspace\discord\vencord\dist\patcher.js"
```

And here's the documentation of all the flags...

```
C:\path\to\modhook.exe
 -d "c:\users\megu\appdata\roaming\discord" # This is the path to the folder Discord is installed to.
 -m "c:\users\megu\downloads\vencord\dist\patcher.js" # This is the path to your mod's entrypoint file.
 # Below are optional, and probably not needed.
 -t "c:\users\megu\downloads\vencord\dist\patcher.js" # This is the query used to see if the mod is loaded. This defaults to the value of `-m`
 -c "MyAlternativeProfile" # A unique name that lets you have multiple instances of discord with unique profiles
 -a "c:\users\megu\downloads\ModHook\app.asar" # The path to the custom app.asar loader.
 -h # This just shows a help command in case you get lost.
```

# Setup

You'll need rust...

Run `cargo build` to build the project and the `app.asar` file.

If you're developing this project, you'll need to place the built app.asar in the root of this project.
You'll find it in the target/{release} folder.

# Meow

meow
