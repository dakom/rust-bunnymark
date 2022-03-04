[![Build Status](https://github.com/dakom/rust-bunnymark/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/rust-bunnymark/actions)

# What is it?

A Rust/WASM/WebGL bunny bouncing test, built on [awsm](http://github.com/dakom/awsm/), for comparing to [PIXI](https://www.goodboydigital.com/pixijs/bunnymark/)

The main branch and live demo uses explicit geometry without instancing, the `instancing` branch uses instancing

# How to start?

## [CLICK HERE](https://dakom.github.io/rust-bunnymark)

By default it adds 100 bunnies per tick, but this can be overridden by appending `#NUM` to the url (where NUM is the number of bunnies to add per tick)

# How does it compare?

Of course Rust/WASM wins... gotta keep in mind though PIXI is an amazing and robust 2D framework. Awsm is only an opinionated but thin wrapper over webgl. So on the Rust side here there's no scene graph, no user-specified filters, no easy API for grouping textures, etc.

A more accurate comparison to test would be a raw WebGL wrapper in JS, with all the bunny adding/updating in JS as well.

However, what really surprised me is how wide the margin is.

On my current machine, PIXI slows down _drastically_ while adding bunnies, already feeling it badly at ~30,000. Letting go and having the bunnies settle, it gets _much_ better, and took around 130,000 bunnies to get to a FPS of 50

Impressive... but... Rust barely slowed down at all while adding bunnies, and it took around 500,000 bunnies before I got to that same slowdown of 50fps. Half a million!
 
(note - PIXI does a neat aesthetic thing of having different textures when you release/re-press but the numbers are exactly the same when just keeping pressed the whole time and using one texture)
