# Minecraft Tracker
This is a simple rust CLI that designed to track locations in minecraft. It extreamly simple and only allow the user to
add a locations coordinates and then later search all the coordinates that have been added. This is useful if you're
exploring the map and want to take not of an interesting location to come back and explore at a later time.

The program is extreamly simple. Once the program is
started you have 3 options [A] to add a new location. [S] to search all the exsisting locations. [Q] to quit the program.
All options are case sensitive. Adding a new location will ask you to add a location name, description and then it's coordinates
including the dimention you are in (Overworld, Nether, The End). Search text is complied into a regex so you any regex text
can be used to search locations. The search currently only looks through location titles.

# Getting Started

Using this CLI is very simple. First clone the repo to your computer. This tool was complied for Mac OSX.
If you are running a different operating system (Windows/Linux etc.) you'll need to recompile the project.
You can complie this project using cargo by following instructions on the [rust website](https://doc.rust-lang.org/cargo/index.html). Once you have the project downloaded and compiled you can run the tool from the terminal.

```./minecraft_tracker/target/release/minecraft_tracker locations.csv```

notice that the tracker tool requires 1 command line argument. locations.csv specifies the location where all
your added locations will be stored. You can create different files for different worlds and specify them here.

Once you start the tool you will be presented with 3 options
    1) [A]dd a place
    2) [S]earch a place
    3) [Q]uit
you can select any of these options by typing the first letter of the command (e.g. a, s, or q). These commands
are case insensitive.

## Adding a place

![Add A Place](https://github.com/bjatkin/minecraft_tracker/blob/master/img/mine_track_add.png)

When adding a place you will be asked to supply some basic information about the location. First you'll need to add a name.
Choose something descriptive (e.g. Desert Village) as this will be used later by the search function.
Next you'll be asked to give a description. 
You can leave this blank, or you can add some more information about the location you added 
(e.g. this village is near spawn and a desert temple).
Once you've entered a description you'll be prompted to specify the dimention your location is in.
    1) [O]verworld
    2) [N]ether
    3) [E]nd
Again, you make your choice by using the first letter of the command (e.g. o, n or e). These commands are case insensitive.
finally you'll have to enter the x, y and z coordinates. Be sure to press enter after entering each of the coordinates.

## Searching your places

![Search A Place](https://github.com/bjatkin/minecraft_tracker/blob/master/img/mine_track_search.png)

When searching your places you'll be asked to input a query. This is a regex that will attempt to match against 
all the place names. If a match is found the place will be displayed.

## Deleting a place

Currently deleting a place is not supported from within the CLI. If you have a place you want to delete simply open
the locations csv file (the file you specified in the command line arguments) and delete that row.

## Edition a place

Editing a place is also not currently supported. If you need to edit a place you'll need to edit it from within
the CSV file as well.

# Bugs, Feature Requests and Pull Requests

If you find a bug in this software please open an issue on the github repo. I'll try to fix it as quickly as possible.
If you'd like to help contribute to this project please contact me at opensource.atkin@gmail.com.

# TODO 
 * Edit old entries
 * Delete old entries
 * Search nearby specified coordinates
 * Query the description rather than the name
 * Query both the description and the name
 * Save a query to run in quickly
 * Input your coordinates to sort query results by distance