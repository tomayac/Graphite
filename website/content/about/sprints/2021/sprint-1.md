+++
title = "Sprint 1"

[extra]
order = 1
+++

*February 13 – March 13, 2021*

![](https://youtu.be/gqCxt8XL92o?t=393)

{{ youtube(id="gqCxt8XL92o", t="393", image="https://static.graphite.rs/content/index/commander-basstronaut-youtube.avif", title="") }}

Since February's Rust Gamedev Meetup which [introduced the Graphite vision](https://www.youtube.com/watch?v=Ea4Wt_FgEEw&t=563s) has attracted tremendous interest, community advice has shifted the development strategy to focus on an MVP alpha release with haste:

- The past year's in-development [custom GUI](https://github.com/GraphiteEditor/Graphite/tree/c72f8ba2dbe0819790c24e9bfd8efee6da1bb67e/gui) has been [shelved](https://github.com/GraphiteEditor/Graphite/commit/e21bca41c6ba53358fcf275c663453640737f82d) in lieu of an interim web GUI. Graphite intends to natively support Windows, Mac, Linux, and Web. This change unblocks core application development but means Graphite is Web-only until the Rust GUI ecosystem matures. Good progress this month has been made building the web GUI with Vue.
- Graphite's MVP will now support only vector editing. This defers the large complexity of the graph render engine required for node-based raster editing. It should be less difficult to first focus on building a vector editor that improves upon the UX of Illustrator and Inkscape.
