+++
title = "Menu bar"

[extra]
order = 1
+++

The **menu bar** is the series of menus running across the top left of the editor's [**title bar**](../#title-bar). It provides organized access to many actions which are described on this page.

![The menu bar](https://files.keavon.com/-/BlackAdorableHectorsdolphin/capture.png)

Clicking **File**, **Edit**, **Layer**, **Document**, **View**, and **Help** opens a dropdown menu with clickable actions. Pay attention to the keyboard shortcut listed on the right of each row in the dropdown menus. Learning to use them can help speed up your workflow.

The rest of this page is intended as a reference resource. Skip ahead to the next page if this is your first read-through of the manual.

## App button

![The app button](https://files.keavon.com/-/TinyAllTapaculo/capture.png)

The **app button** shows the [Graphite logo](/logo). Clicking it opens the Graphite website home page.

## File

![The File menu](https://files.keavon.com/-/SpeedyIdioticCockroach/capture.png)

The **File menu** lists actions related to file handling:

| | |
|-|-|
| New… | Opens the **New Document** dialog for creating a blank canvas in a new tab.<br /><br />![The 'New Document' dialog](https://files.keavon.com/-/VioletWillingNorthernseahorse/capture.png) |
| Open… | Opens the operating system file picker dialog for selecting a `.graphite` file from disk to be opened in a new tab. |
| Open Demo Artwork… | Opens the **Demo Artwork** dialog for loading a choice of premade sample artwork files provided for you to explore. Click the button below each image to open it.<br /><br />![The 'Demo Artwork' dialog](https://files.keavon.com/-/BrightScaredPolarbear/capture.png) |
| Close | Closes the active document. If it has unsaved changes (denoted by the `*` after the file name), you will be asked to save or discard the changes. |
| Close All | Closes all open documents. To avoid accidentally losing unsaved work, you will be asked to confirm that you want to proceed which will discard the unsaved changes in all open documents. |
| Save | Saves the active document by writing the `.graphite` file to disk. An operating system file download dialog may appear asking where to place it. That dialog will provide an opportunity to save over a previous version of the file if you wish, instead of saving another instance. |
| Import… | Opens the operating system file picker dialog for selecting an image file from disk to be placed as a new bitmap image layer into the active document. |
| Export… | Opens the **Export** dialog for saving the artwork as a *File Type* of PNG, JPG, or SVG. *Scale Factor* multiplies the content's document scale, so a value of 2 would export 300x400 content as 600x800 pixels. *Bounds* picks what area to render: *All Artwork* uses the bounding box of all layers, *Selection* uses the bounding box of the currently selected layers, and an *Artboard: \[Name\]* uses the bounds of that artboard. *Transparency* exports the PNG or SVG file with transparency instead of the artboard background color.<br /><br />![The 'Export' dialog](https://files.keavon.com/-/EmbellishedChillyAngelwingmussel/capture.png) |
| Preferences… | Opens the **Editor Preferences** dialog for configuring Graphite's settings.<br /><br />![The 'Editor Preferences' dialog](https://files.keavon.com/-/DeliriousRosyNutria/capture.png)

## Edit

![The Edit menu](https://files.keavon.com/-/StrangeDarksalmonUngulate/capture.png)

The **Edit menu** lists actions related to the editing workflow:

| | |
|-|-|
| Undo | Steps back in the history of changes in the active document. |
| Redo | Steps forward in the history of changes in the active document. |
| Cut | Copies the selected layer(s) to the clipboard. Also deletes those layers. |
| Copy | Copies the selected layer(s) to the clipboard. |
| Paste | Pastes the copied layer(s) from the clipboard into the document. It will end up beside a selected layer or inside a selected folder, or otherwise at the base of the folder structure. |

## Layer

![The Layer menu](https://files.keavon.com/-/WetStupendousNorwaylobster/capture.png)

The **Layer menu** lists actions related to the layers within a document:

| | |
|-|-|
| Select All | Selects all layers and folders in the document. |
| Deselect All | Deselects everything in the document. |
| Delete Selected | Removes all selected layers and folders. |
| Grab Selected | Begin grabbing the selected layer(s) to translate (move) them around with your cursor's movement. Confirm with a left click or <kbd>Enter</kbd> or cancel with a right click or <kbd>Esc</kbd>. <!-- TODO: link to more info in nav section --> |
| Rotate Selected | Begin rotating the selected layer(s) around their pivot point with your cursor's movement. <!-- TODO: link to more info in nav section --> |
| Scale Selected | Begin scaling the selected layer(s) around their pivot point with your cursor's movement. <!-- TODO: link to more info in nav section --> |
| Order ><br />Raise to Front | Reorders the selected layer(s) above all other layers within their same folder(s), so they appear in the layer stack and render above those other layers. |
| Order ><br />Raise | Reorders the selected layers(s) up by one in the layer stack, so any layer that was immediately above the selected layer(s) ends up immediately below. |
| Order ><br />Lower | Reorders the selected layers(s) down by one in the layer stack, so any layer that was immediately below the selected layer(s) ends up immediately above. |
| Order ><br />Lower to Back | Reorders the selected layer(s) below all other layers within their same folder(s), so they appear in the layer stack and render below those other layers. |

## Document

![The Document menu](https://files.keavon.com/-/IncomparableTealPullet/capture.png)

The **Document menu** lists actions related to the document and artwork:

| | |
|-|-|
| Clear Artboards | Removes all artboards from the document, thus enabling an infinite canvas. This action is **temporarily disabled** due to a code refactor, but the same can be achieved using the **Artboard tool**, selecting a canvas, hitting <kbd>Delete</kbd>, and repeating for every artboard. |

## View

![The View menu](https://files.keavon.com/-/NocturnalTurbulentJabiru/capture.png)

The **View menu** lists actions related to the view of the canvas and viewport:

| | |
|-|-|
| Zoom to Selected | Zooms and frames the viewport to the bounding box of the selected layer(s). |
| Zoom to Fit | Zooms and frames the viewport to fit all artboards, or all artwork if using infinite canvas. |
| Zoom to 100% | Zooms the viewport in or out to 100% scale, matching 1:1 the scale of the document and viewport. |
| Zoom to 200% | Zooms the viewport in or out to 200% scale, displaying the artwork at twice the actual size. |

## Help

![The Help menu](https://files.keavon.com/-/GhostwhiteWorthwhileLeopardseal/capture.png)

The **Help menu** lists actions related to information about Graphite:

| | |
|-|-|
| About Graphite… | Opens the **About Graphite** dialog for displaying release and license information. |
| User Manual | Opens this [user manual](https://graphite.rs/learn/). |
| Report a Bug | Opens a page to file a [new GitHub issue](https://github.com/GraphiteEditor/Graphite/issues/new). |
| Visit on GitHub | Opens the [Graphite GitHub repository](https://github.com/GraphiteEditor/Graphite). |
| *Debug section* | Developer-only actions. |
