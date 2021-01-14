# MechI Final Project
Final project Mechatronics of JST team at Reykjavik University.

JST composed by Matteo Guerrini, Giacomo Menchi and William Paciaroni.


The main concept of this project is the home security: we are trying to solve the problem of thieves and intruders entering your house when you are not inside by implementing a layer of security which alerts you if a presence is detected when the system is active.

We plan on using a Raspberry Pi which controls a camera, a PIR sensor and a microphone.
•  The PIR sensor detects movements inside the house and turns on the camera when one is registered,which then recognizes the presence of an intruder (a human figure).
•  The microphone is used to check for loud noises (i.e.  those which are above a certain threshold) whichwould further confirm the presence of an intruder.
•  A notification is sent to the user (via email or telegram bot) whenever the camera detects a person or themicrophone hears a loud noise.
•  The system can be turned on and off with a physical button or wireless via Adafruit.io dashboard (APIcalls), and its status is reflected on a LED.

The preliminary components list would be:
•  Raspberry Pi Zero
•  Raspberry Pi Camera module
•  PIR sensor
•  Microphone
•  Switch
•  One led RGB or two LEDs red and green
•  Jumpers, Resistors and Breadboard
•  Box (3d printed) 