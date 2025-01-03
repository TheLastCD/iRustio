# Outline
## What is iRustio?
it's a little experiment to build a dynamic terminal based iRadio player using MPV and Rust. Inspired by radion and termusic

## Why the name?
i'm terrible at coming up with them so please have a better idea 
## Can I Contribute
Yes!!! my rust knowledge and syntax is not the best so any suggestions in the form of a PR are much appreciated

# Backends
iRustio supports the following backends:
  - MPV (default)
  - GStreamer
Change backend by modifying the Config.toml file created intitial startup

# Install
Mac OS:
TODO


# Project Milestones

## General Milestones
-[x] Produce a working iRadio stream using the RadioBrowser API

-[x] Enable name querying for available streams

-[x] Add support for browsing available tags

-[?] Store/ cache streams so we don't need to call them everytime a query is made/ app is restarted 
  - implemented the following:
    - Storing of presets
    - Recently played stations
  - Current API speeds doesn't really warrent more forms of caching but maybe needed later down the line


-[] Add support for user favourites

-[] Add support for user added stations

-[] Make a clean terminal based UI

## Stretch Goals/ Sidetracks
-[] Create a module for termusic to add native radio streams

-[x] Add support for different audio backends (see above)

-[] Speed up API calls where possible


# Inspiration and Sources

radion and pyradion:
* used for base code outline and use of MPV and initial idea
* https://gitlab.com/christosangel/pyradion
* https://gitlab.com/christosangel/radion

RadioBrowser:
* Rust API for radio urls
* https://crates.io/crates/radiobrowser
* https://www.radio-browser.info/

Termusic:
* Future create module that can be incuded with Termusic
* Initial Idea 
* https://github.com/tramhao/termusic
