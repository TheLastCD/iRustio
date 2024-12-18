# Outline
## What is iRustio?
it's a little experiment to build a dynamic terminal based iRadio player using MPV and Rust. Inspired by radion and termusic

## Why the name?
i'm terrible at coming up with them so please have a better idea 
## Can I Contribute
Yes!!! my rust knowledge and syntax is not the best so any suggestions in the form of a PR are much appreciated

# Install

TODO


# Project Milestones

## General Milestones
-[x] Produce a working radion stream using the RadioBrowser API

-[x] Enable name querying for available streams

-[] Store/ cache streams so we don't need to call them everytime a query is made/ app is restarted

-[] Add support for browsing available tags

-[] Add support for user favourites

-[] Add support for user added stations

-[] Make a clean terminal based UI

## Stretch Goals/ Sidetracks
-[] Create a module for termusic to add native radio streams

-[] Add support for different audio backends (if necassery)

-[] Speed up API calls where possible


# Inspiation and Sources

radion and pyradion:
* used for base code outline and use of MPV and initial idea
* https://gitlab.com/christosangel/pyradion
* https://gitlab.com/christosangel/radion

RadioBrowser:
* Rust API for radio urls
* https://crates.io/crates/radiobrowser
* https://www.radio-browser.info/

Termusic:
*Future create module that can be incuded with Termusic
*https://github.com/tramhao/termusic
