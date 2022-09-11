# Meadowlark UX Design Document

The UX design of Meadowlark is to create a friendly, but still heavily utilizable platform for musicians to create music. As a result, the design aspects of the UX are created to increase customizability while still keeping a clean space for visualizing the entire composure of the song. As a result of making a easy to use platform for all users, the UX is designed to be both MacOS and Windows friendly. Many features that are included in other music editing software are included in Meadowlark to give a seamless creation process for users.

# Objective

The main objective of the UX is to be functional and simple, making for easy customizability. This starts with the various options that a user may have when trying to create a song. They can edit the file types and also customize their view of the window. The channel rack enables users to look at all the instruments being used in a single track, also giving them the ability to modify how the sounds are integrated. The patterns section is where users are able to view the pertaining patterns of notes used from each of the instruments in the channel rack. The main objective of the Meadowlark UX is to enable users with differing skillsets to still manage and make sounds to their liking.

# Goals

One of the largest goals for Meadowlark is to get the various components working together. The browser panel being resizable is something that needs to completed for the UX to be more customizable. Along with this, a variety or other items need to be built to the browser panel including a side-bar, keymap, scroll option when there are too many datapoints, search, tooltips, refresh, preview sample playback, clipping item labels, and the general browser requirements. These goals are currently being worked on, ensuring that the functionality is tested thoroughly. 

As this project is still in the early phases, many features have yet to be worked on and thought through for designing and eventually creating. The browser is definitely one of the most crucial components to work on as the user experience with having a simple and effective way to find anything in a complex software like this is needed. The user's experience should be a flawless interaction with the UI and the best way to ensure that is to enable them to customize their view and be able to find any information or data they need in producing sounds.  

# Non-Goals
(*TODO*)

# UX Design

Note that this is just a mockup and the final design is subject to change. Here is the most up-to-date mockup for Meadowlark:

![UI Mockup](assets/design/gui-mockup-version3.png)

## Top Bar

### Section 1
![Top bar section 1](assets/design/top_bar/top-bar-1.png)

* We start with the traditional "File/Edit/View/Help" dropdown menus. I feel these are pretty self-explanatory.

(From left to right)
* Undo/Redo buttons
* Save button
* The tempo in bpm at the current playhead. The user can drag on this widget like a slider to change this value, or they may double-click this box to enter a new tempo with the keyboard.
* When the user clicks this multiple times, it will set the bpm value to the left according to the rate at which they clicked it.
* Groove Menu - When the user clicks this, a dialog box with groove settings will appear (settings like swing and volume accents). *(This dialog box has not been designed yet.)*
* The time signature at the current playhead. The user can drag on this widget like a slider to change this value, or they may double-click this box to enter a new value with the keyboard.

### Section 2
![Top bar section 2](assets/design/top_bar/top-bar-2.png)

(From left to right)
* Transport Display - The user can click on this to toggle between two different modes:
    * Musical time - This will display the time at the current playhead in (measures/beats/16th beats).
    * Real time - This will display the time at the current playhead in (hours/minuts/seconds/milliseconds).
* Loop Toggle Button - The user can click on this to toggle looping in the transport.
* Play button - The user clicks this to play/pause the transport.
* Stop button - The user clicks this to pause the transport and return to the beginning of the most recently seeked position.
* Record button - *(TODO: decide on recording workflow)*
* Record settings - *(TODO: decide on recording workflow)*

*TODO: Rest of the design doc.*
