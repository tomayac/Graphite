+++
title = "Features and limitations"

[extra]
order = 1
+++

Please keep in mind that Graphite is alpha software, meaning it is actively changing and improving. Remember to save you work frequently because crashes are not unheard of.

## Current capabilities

Vector editing is the core competency of the Graphite editor at this stage in its development. That means you can expect to create **vector**, or shape-based, illustrations and graphic designs. Basic geometry like circles and rectangles can be drawn and modified into more complex shapes, or created from scratch with the Pen tool.

Raster editing is a growing capability that will develop into the central focus in time. **Raster** imagery is composed of pixels which are grids of color that can represent anything, like paintings and photographs. The current feature set lets you import images, manipulate them using the node-based compositor, and apply nondestructive adjustment effects.

Furthermore, the raster-based Imaginate feature enables you to synthesize artwork using generative AI based on text descriptions. With it, you can also nondestructively modify your vector art and imported images. You can inpaint (or outpaint) the content in a specific masked part of an image or use it to touch up quick-and-dirty compositions.


## Status and limitations

Please make yourself aware of these factors to better understand and work around the rough edges in today's Graphite editor.

### Unstable document format

Artwork you save as a `.graphite` document file will eventually fail to open in future versions of the Graphite editor because of code changes. Since the implementations are in flux for many systems, file format stability isn't possible yet during this alpha stage of development. A redesigned file format with a `.gdd` (Graphite Design Document) extension will replace `.graphite` files and it will be built with backwards-compatability in mind.

Sometimes an error will appear when opening an outdated document. Other times, it may open but result in a crash or broken functionality when editing.

To open an outdated file, [look here](https://github.com/GraphiteEditor/Graphite/deployments/activity_log?environment=graphite-editor+%28Production%29) for the previous version of the Graphite editor that was published before the date you saved the document. Click "View deployment" to open it.

### Legacy layers

Because of an ongoing code migration, layers are currently split into two concepts: **legacy layers** and **node-based layers**. The editor's tools create and interact with legacy layers, which live in the Layers panel. Each legacy layer constitutes an isolated node graph storing its artwork content, such as vector shapes or raster images. Consequently, nodes cannot interact with one another across the separate node graphs of each legacy layer. As a workaround, nodes can be copy/pasted into the same graph.

Node-based layers are designed to work in the **document graph**, which is the single node graph belonging to the document instead of a legacy layer. Node-based layers don't work properly in legacy layer node graphs (rendering errors will arise). For now, tools can't interact with artwork in the document graph.

Once the code for tools is migrated to edit the document graph's node-based layers, legacy layers (and the legacy folders which contain them) will be retired. The Layers panel will then be updated to display all the node-based layers in the document graph.
