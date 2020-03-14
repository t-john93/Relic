# Collaborators:
Trevor Johnson trevor29@pdx.edu
Nathaniel Sherrett sherrett@pdx.edu
Eric Harper harper28@pdx.edu

# Name:
Relic

# Description:
This is a simple side-scroller game implemented with the ggez framework. The player constantly runs through the map, and navigates to the star at the end, the Relic. The player changes direction upon reaching a wall. The player has the ability to jump up on the wall (using spacebar), slide down, and jump off in order to reach higher platforms.

# Usage:
Navigate to the relic directory in the terminal, once in the root directory of the project, execute the command ```cargo run```. This will download and compile all of the necessary dependencies and then run the game in a separate window.

# Testing:
Manual testing has been conducted throughout the game, and no automated testing was done. Using a single thread event loop style game engine limited type of testing we could implement. We added a few simple assertions as the game ran to verify that values didn’t drift overtime. However, even with this testing style we were limited. The values are updated once every nanosecond and are subject to a large amount of variability.

# The Rundown:
What worked:
Getting simple left, right, and jump movements working for the player was rather quick and effortless. Drawing a map also seemed to be relatively straightforward.

What didn't work:
Once additional physics were added, e.g., wall-sliding, and collisions, things became pretty tricky. Also resolving type errors upon separating code into individualized files proved to be troublesome.

Satisfaction/Improvements:
We are relatively satisfied with the game that we ended up with, given how hard we had to work to get here. We learned a lot about event handling and how difficult it is to program physics and collisions. Towards the end, we found a crate that includes collision capabilities, so knowing that, we probably would have implemented those elements differently. Unfortunately adding this crate would have forced a refactor of nearly the entire game, and we did not have time to rewrite everything we would have needed to. As far as improvements, much can be said. It was a risky choice to choose a game engine when none of us had ever worked with one before, and it shows in our final product. We spent much of our time implementing logic that in theory should have worked, only to find the functionality virtually ignored by the engine while running. This forced us to constantly retrace our steps and is the reason for much of the hardcoding and “magic numbers” seen in the repo. If we could have a second go of the project, we would have decided to use a more mature engine such as piston, or simply use vanilla rust. We hope you see the correct use of crates, modules and submodules, as well as the use of closures and other functional aspects. We tried to show our knowledge gained from the course despite our limitations as game devs.
