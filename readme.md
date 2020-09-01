# Minecraft Tracker
This is a simple rust CLI that is designed to track locations in minecraft. It extremely simple and only allow the user to
add a location's coordinates and then later search all the coordinates that have been added. This is useful if you're
exploring the map and want to take note of an interesting location to come back and explore at a later time.

# Getting Started

Using this CLI tool is easy. First clone this repo to your computer. Minecraft Tracker was originally compiled for Mac OSX.
If you are running a different operating system (Windows/Linux etc.) you'll need to recompile the project.
You can compile this project using cargo by following instructions on the [rust website](https://doc.rust-lang.org/cargo/index.html). Once you have the project downloaded and compiled you can run the tool from the terminal.

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
Once you've entered a description you'll be prompted to specify the dimension your location is in.
    1) [O]verworld
    2) [N]ether
    3) [E]nd
Again, you make your choice by using the first letter of the command (e.g. o, n or e). These commands are case insensitive.
finally you'll have to enter the x, y and z coordinates. Coordinates should be entered as whole numbers rather than decimals.
Be sure to press enter after entering each of the coordinates.

## Searching your places

![Search A Place](https://github.com/bjatkin/minecraft_tracker/blob/master/img/mine_track_search.png)

When searching your places you'll be asked to input a query. This is a regex that will attempt to match against 
all the place names. If a match is found the place will be displayed. In addition to the coordinates you originally
set for the location, all overworld locations will display their coordinates in the nether and vice versa.

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
 * Compile for multiple platforms to make it easier to use for non-technical users
 * Edit old entries
 * Delete old entries
 * Search nearby specified coordinates
 * Query the description rather than the name
 * Query both the description and the name
 * Save a query to run in quickly
 * Input your coordinates to sort query results by distance
