# Digital Extinction

Digital Extinction is a 3D real-time strategy (RTS) game. It is set in the near
future where humans and AI fight over their existence.

It is [open source & free software](#license). Forever! It runs on Linux,
Windows and potentially other platforms.

The game is completely written in [Rust](https://www.rust-lang.org/) with
[Bevy](https://bevyengine.org/) used as the engine.

[![Gameplay video](https://img.youtube.com/vi/zUVyyVCCbus/0.jpg)](https://youtu.be/zUVyyVCCbus)

# Status

The game is still in [early phases](#roadmap) of its development. If you are
looking for a mature game, come back in a few months.

Feedback, bug reports and other [contributions](#contributing) are welcomed.

# How to Play?

Game controls & game play tutorial is at [TUTORIAL.md](/TUTORIAL.md).

## Using Build from `main` Branch

A ZIP file with a binary and all assets is produced by a GitHub Action which is
run on every push to `main` branch. You can browse the actions
[here](https://github.com/DigitalExtinction/Game/actions).

On a “usual setup” it is sufficient to extract the ZIP file and execute the
binary called `de` or `de.exe`.

## Building from Source

What you need:

* installed and configured Git
* [Git LFS](https://git-lfs.github.com/)
* [Rust](https://www.rust-lang.org/tools/install)

Clone, build & run recipe (will work in majority of the cases):

* `git clone git@github.com:DigitalExtinction/Game.git DigitalExtinction`
* `cd DigitalExtinction`
* make sure that Git LFS files in [assets/](assets/) are pulled
* `cargo run --release`

# Build Profiles

Link time optimizations (LTO) are enabled on release profile. Thus release
build takes a long time.

## Testing Profile

On top of the standard Cargo build profiles, there is a `testing` profile
optimized for manual testing of the game. In this profile, extra checks and
debug info is included in the build. LTO is disabled and optimization level is
fine tuned for fast compilation times while keeping performance reasonable.

# Where to Get Help?

* Open a question in [Q&A category of this repository's
  discussions](https://github.com/DigitalExtinction/Game/discussions/categories/q-a).

# Community

* [Discord server](https://discord.gg/vHMFuCWGSX)

* [Subreddit /r/DigitalExtinction/](https://www.reddit.com/r/DigitalExtinction/)

# Goals

Many of the goals are intentionally vague and aspirational. They serve as a
general direction for the future development. All changes and decisions shall
go hand in hand with the spirit of the following goals.

* Develop a forever free (as in free speech) and open source RTS game. A game
  without the restraints of commercial development – no marketing motivated
  features, no dopamine traps, no in-game purchases…

* Design an original game, not yet another clone of an existing (and often a
  very old) game. The game is a combination of tried and true concepts and
  mechanics from RTS genre with new and innovative ideas.

* Create a true **strategy** game, where dexterity or APS do not play primary
  role.

* Focus on exponential in-game technological and economical progress. Players
  who consistently outperform their competitors for extended times prevail.
  Short term boosts and performance fluctuations do have proportionately small
  impact on the game results.

* Produce a modern game, unchained from obsoleted constraints and utilizing
  current technology. Truly utilize the power of modern multi-core CPUs and
  other capable hardware with the help of the advances in software development,
  like fearless concurrency of Rust programming language, or ECS based Bevy
  engine.

  Seize this technological opportunity to create an RTS of grand scale, with
  hundreds of buildings and thousands of fully simulated units on the game map.

* Develop the game indefinitely and incrementally. To regularly ship a new
  version (rolling release) and to forever improve the game based on new
  experience and new ideas.

* Show that non-trivial games could be created in Rust and by extension Bevy
  engine.

# Contributing

Contributions to the project in any form are welcomed. Check out our
[Contributor's Guide](/CONTRIBUTING.md).

MSRV: the Minimum Supported Rust Version (MSRV) is “the latest stable release”
of Rust.

# Roadmap

1. [MVP](https://github.com/DigitalExtinction/Game/milestone/1)
1. Release (first) [version
   0.1](https://github.com/DigitalExtinction/Game/milestone/2)
1. Publicly announce the game:
   [#60](https://github.com/DigitalExtinction/Game/issues/60)
1. [Indefinitely improve the game](/CONTRIBUTING.md#development-process)

## MVP

This is an embarrassingly bare bones game. It is complete in the sense that you
can start the game, play against someone and win or loose.

At this stage, the game is too bare bones to be enjoyable. The UI/UX misses
some very important features (like seeing what units are selected, ability to
see health / energy of units, and so on).

The goal of this milestone is to lay down the foundations, setup basic
insfrastrucutre (GitHub Actions, issues labels, …) and achieve an important
psychological milestone.

## 0.1

This is the first published version of the game. In theory, this should be the
earliest possible version, minimizing development time and effort, which is
threshold enjoyable for an actual player.

As opposed to MVP, this version has all the basic UI to make the UX acceptable.
The game mechanics is somewhat expanded so it is no longer thoroughly "dummy".

# License

Digital Extinction is free and open source. All code in this repository is
licensed under GNU GPLv3 ([LICENSE](LICENSE) or
[https://www.gnu.org/licenses/gpl-3.0.en.html](https://www.gnu.org/licenses/gpl-3.0.en.html)).

Unless you explicitly state otherwise, any source code contribution
intentionally submitted for inclusion in the work by you, shall be licensed as
above, without any additional terms or conditions.

All other artwork in this repository, including 3D models, textures,
animations, UI bitmaps, sounds and music is licensed under
[Attribution-ShareAlike 4.0 International (CC BY-SA
4.0)](https://creativecommons.org/licenses/by-sa/4.0/legalcode)
