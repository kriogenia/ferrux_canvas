# FerruX Canvas
The FerruX Canvas is a tool developed while creating the FerruXengine
to abstract and ease the use of the *Pixels* buffer used by the engine.
As the tool proved useful for other possible future projects and a couple
of mutual thought that they could use it too, it was decided to extract 
it as its own library.

## Roadmap

The current development attemps to cover the following features:

* `reset()`, function to clear the pixel buffer.
* `clear()`, function to clear the pixel buffer and screen.
* `autoclear` config, so it can be determined if the current canvas should
be cleared after each frame.
* Configuration to set the background color.