---
title "ZeroBridge 0.1.0 Release!"
date "2024-04-03 15:00"
---

I wrote a Discord bridge for Minecraft 1.4.7! I might go in depth about this in another blog post, but for now, here's a link: [ZeroBridge](https://git.gay/j0/ZeroBridge)

## Okay here's a bit of talk
It runs in NilLoader, and is mainly targeted for the pack Rewind Upsilon. It was really fun to make! NilLoader is very barebones, and most of the code is directly injected into Minecraft via MiniTransformer. For some reason, I find working with ASM (Java Bytecode Manipulator) to be quite nice.

I originally started work on this project maybe a year ago? I digged it out recently from my backups because I wanted to play Upsilon but the people I want to play with *really* want a Discord bridge. I basically rewrote the entire thing to play nice when inside Rewind Upsilon. I also added a config (with Jankson), webhooks (with Visage rendering), a bunch of other stuff that makes it feel like a real thing.

Anyway, enjoy!

<img src="https://f004.backblazeb2.com/file/j0-lol-website/0b-dc.png" alt="ZeroBridge being used on Discord">

<img src="https://f004.backblazeb2.com/file/j0-lol-website/0b-mc.png" alt="ZeroBridge being used on Minecraft">
