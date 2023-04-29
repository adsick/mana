# Mana input manager

This project aims to research and develop an input library can handle keyboard layouts with complex keyboard combinations using the time information of keyboard events.

You may heard of QMK/VIA, Mana is different in that it is designed to power your application's input logic while allowing users to customize controls on per application basis.

## what problems will it solve?

* ergonomics - more actions are within reach thanks to home-row combos (FJ, JK, etc.), no need to move your fingers. It also enables you to use minimal keyboards (such without numpads, fn keys, and even arrows or numrows).
* do more with less - with mana you can bind much more key combinations using time relations between input events
* customizable controls - you can change the way you interact with your app by configuring it's mana profile.
* no-nonsense - mana allows you to eliminate key repeat where it is not necessary. 
* layout agnostic - have you used vim on non-qwerty layout? mana uses scan codes (physical key identifiers) to allow you to abstract over os-specific layout. You can define your own layout.

# State of the project
Currently mana in its bare-bones state, nothing works at the moment, but hopefully it will change soon.
