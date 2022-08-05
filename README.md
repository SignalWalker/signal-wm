# Signal WM

A dynamic Wayland compositor, designed for efficiency and low latency. Built with Rust and WGPU.

## Features

Released features have an adjacent checkmark.

* Hardware-accelerated composition
  * Multi-monitor support
    * Separate refresh rates
  * Multi-GPU support
* Fine-grained, modal input settings
  * Context-based hotkeys
  * Device configuration

## Design

### Definitions

* **Surface**: The buffer into which applications render themselves.
* **Panel**: An abstract unit managed by the compositor. A panel may have an arbitrary number of child panels, organized as a tree.
  * A panel may either be **floating** or **tiling**. A floating panel may not be the child of any other panel.
  * **Window**: A special kind of panel, onto which surfaces are projected, and which can hold input focus.
  * **Tag**: A key-value pair that can be used to provide hints to the layout or compositor. A panel may have an arbitrary number of tags.
* **Portal**: A view into a set of panels, with an associated resolution, pixel density, and pixel depth.
  * The root panels of a portal may include an arbitrary number of floating panels, and up to one tiling panel.
* **Monitor**: A buffer into which portals (and the panels they view) are rendered, with an associated resolution, pixel density, pixel depth, and refresh rate.
* **Layout**: A method for determining the relative position and dimensions of a set of panels. A panel may have an associated layout, which applies to all of its children.

### Problems


