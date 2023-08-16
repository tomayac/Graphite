+++
title = "Interface"
template = "book.html"
page_template = "book.html"

[extra]
order = 2
+++

This chapter introduces the concepts and terminology for the user interface (UI) of the Graphite editor. You may skip to the next chapter if you're familiar with the general layout and terms used in industry-standard graphics editors.

## Title bar

The bar running across the top of the editor is called the **title bar**. In the (forthcoming) desktop release of Graphite, this acts as the draggable window frame.

![The title bar](https://files.keavon.com/-/SomberQualifiedSpringpeeper/capture.png)

### Menu bar

On the left, the [**menu bar**](./menu-bar.md) provides quick access to many editor, document, and artwork related controls. Its functions are covered in detail on the next page.

![The menu bar](https://files.keavon.com/-/BlackAdorableHectorsdolphin/capture.png)

In the (forthcoming) macOS desktop release, the menu bar is absent from the editor window; its functions are instead located in macOS menu bar.

### Document title

In the center, the **document title** displays the name of the active document. That name is given a `*` suffix if the file has unsaved changes. For example, *Painting.graphite** would be unsaved but *Painting.graphite* would have no changes since it was last saved.

![The document title](https://files.keavon.com/-/UrbanIllegalChamois/capture.png)

### Window buttons

On the right, the **window buttons** provide platform-specific controls for the application window. In the (forthcoming) macOS desktop release, this appears on the left side instead.

| | |
|-|-|
| **Web** | A button to enter fullscreen mode is displayed.<br /><br />The label "*Go fullscreen to access all hotkeys*" indicates that some shortcut keys like <kbd>Ctrl</kbd><kbd>N</kbd> (macOS: <kbd>⌘</kbd><kbd>N</kbd>) are reserved by the web browser and can only be used in fullscreen mode. (An alternative to fullscreen mode: include <kbd>Alt</kbd> in the shortcut combinations for browser-reserved hotkeys.)<br /><br />![Fullscreen button](https://files.keavon.com/-/UnkemptLustrousSpreadwing/capture.png) |
| **Windows<br />& Linux** | The standard window controls are displayed: minimize, maximize/restore down, and close.<br /><br />![Minimize/maximize/close window buttons](https://files.keavon.com/-/MammothEnormousRhino/capture.png) |
| **macOS** | The standard window controls are displayed: close, minimize, and fullscreen. These are located on the left of the title bar.<br /><br />![Close/minimize/fullscreen window buttons](https://files.keavon.com/-/AggravatingBuzzingElephantbeetle/capture.png) |

## Workspace

The **workspace** is the editor's main content area. It houses the **panels** in a grid. The **gutter** lines between neighboring panels may be dragged to resize them.

![The workspace](https://files.keavon.com/-/SkyblueDarkorchidLeech/capture.png)

### Panels

Panels are regions of the UI dedicated to a specific purpose. [**Document**](./document-panel.md), [**Properties**](./properties-panel.md), and [**Layers**](./layers-panel.md) are presently the three panel types. Each will be covered later in the chapter.

Each panel name is shown in its **panel tab bar**. Panel tabs provide a quick way to swap between multiple panels occupying the same area (currently only documents support this). Down the road, these tabs will be dockable so the default layout may be customized.

![The panel tab bar](https://files.keavon.com/-/DeadClutteredOvenbird/capture.png)

Beneath the panel tab bar, the **panel body** displays the content for its panel type.

## Status bar

The bar running across the bottom of the editor is called the **status bar**.

![Status bar](https://files.keavon.com/-/AdeptFrozenConch/capture.png)

### Input hints

The **input hints** are presently the only occupant of the status bar. They indicate what common keyboard and mouse inputs are valid in the current context. Hints change with each active tool as well as with the current interaction state. Keep a frequent eye on the hints to learn more features as you work.

Hints with a **`+`** mean that adding the indicated modifier key will change the base action. For example: in the following action, dragging with right-click held down will tilt the canvas; then additionally holding the <kbd>Ctrl</kbd> key will make the tilt snap to 15° angle increments.

![Example hint](https://files.keavon.com/-/MediumgoldenrodCompassionateCreature/capture.png)

The following chart describes each icon representing the mouse inputs you can perform so a hint's prescribed action occurs.

| | Clicks | Drags | Others |
|-|:-:|:-:|:-:|
| **Left mouse button** | Left click<br /><br />![Left click icon](https://files.keavon.com/-/DelightfulScrawnyAntipodesgreenparakeet/capture.png) | Left click drag<br /><br />![Left click drag icon](https://files.keavon.com/-/FatMarriedMara/capture.png) | Left double-click<br /><br />![Left double-click icon](https://files.keavon.com/-/SteelblueGratefulKid/capture.png) |
| **Right mouse button** | Right click<br /><br />![Right click icon](https://files.keavon.com/-/OutlyingBlondRhesusmonkey/capture.png) | Right click drag<br /><br />![Right click drag icon](https://files.keavon.com/-/DarkcyanExtrovertedAvians/capture.png) | Right double-click<br /><br />![Right double-click icon](https://files.keavon.com/-/SupportiveLivelyStilt/capture.png) |
| **Middle mouse button** | Middle click<br /><br />![Middle click icon](https://files.keavon.com/-/IcyPitifulThylacine/capture.png) | Middle click drag<br /><br />![Middle click drag icon](https://files.keavon.com/-/HalfPhysicalFlicker/capture.png) | Scroll up/down<br /><br />![Scroll up icon](https://files.keavon.com/-/MajesticGrownZebu/capture.png) ![Scroll down icon](https://files.keavon.com/-/SophisticatedStunningCurlew/capture.png) |
