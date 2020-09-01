# Minecraft Tracker
This is a simple rust cli that designed to track locations in minecraft. The program is extreamly simple. Once the program is
started you have 3 options [A] to add a new location. [S] to search all the exsisting locations. [Q] to quit the program.
All options are case sensitive. Adding a new location will ask you to add a location name, description and then it's coordinates
including the dimention you are in (Overworld, Nether, The End). Search text is complied into a regex so you any regex text
can be used to search locations. The search currently only looks through location titles.