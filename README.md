# PoC multiview

This project is a Proof of Concept to validate the multi viewport capacity of
candelabre libs, mostly for candelabre-widgets. The idea is to be able to
easily divide the window in multiple viewports, and specify some of them to
handle specific graphics, like 3D, or 2D with custom graphics, or what you can
dream of. Sky is the limit!

Small steps to validate the PoC:

* ~~make multiple windows (3 will be good)~~
  * ~~initialize candelabre window manager~~
  * ~~handle event loop (basic inputs, like quit, resize and redraw)~~
  * ~~show the 3 windows~~
* ~~create a shape in each window~~
  * ~~initialize nvg~~
  * ~~create a shape~~
* separate in multiple viewports
  * ~~change gl code~~
  * ~~load multiple nvg contexts~~
  * recreate the shape in all viewports
  * set different color to each nvg context
  * send a different shape in each viewport
* add keyboard input to change something about the shape (like color)
  * handle keyboard input
  * handle mouse input
  * randomize color
  * send new color to the right shape
