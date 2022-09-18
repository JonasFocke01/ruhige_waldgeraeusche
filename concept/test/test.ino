/*
   Original sketch taken from https://github.com/PaulStoffregen/DmxSimple
   Modified by Gadget Reboot to use with a demo for two
   American DJ Micro Wash RGBW fixtures

** This program allows you to set DMX channels over the serial port.
**
** After uploading to Arduino, switch to Serial Monitor and set the baud rate
** to 9600. You can then set DMX channels using these commands:
**
** <number>c : Select DMX channel
** <number>v : Set DMX channel to new value
**
** These can be combined. For example:
** 100c355v : Set channel 100 to value 255.
**
** For more details, and compatible Processing sketch,
** visit http://code.google.com/p/tinkerit/wiki/SerialToDmx
**
** Help and support: http://groups.google.com/group/dmxsimple       */

#include <DmxSimple.h>

#define CHANNEL_OFFSET 29

void setup() {
  Serial.begin(9600);

  /* DMX devices typically need to receive a complete set of channels
  ** even if you only need to adjust the first channel. You can
  ** easily change the number of channels sent here. If you don't
  ** do this, DmxSimple will set the maximum channel number to the
  ** highest channel you DmxSimple.write() to. */
  //DmxSimple.maxChannel(10);
  DmxSimple.usePin(3);   // digital output for DMX serial data

  Serial.println("DMX Manual Control");
  Serial.println();
  Serial.println("Syntax:");
  Serial.println(" 123c : use DMX channel 123 (range: 1-512)");
  Serial.println(" 45v  : set current channel to value 45 (range: 0-255)");
  
  // moving heads
  //DmxSimple.write(7 + CHANNEL_OFFSET, 8);
  //DmxSimple.write(8 + CHANNEL_OFFSET, 255);

  // laser
  DmxSimple.write( CHANNEL_OFFSET + 1, 255 );
  DmxSimple.write( CHANNEL_OFFSET + 12, 128 );
  DmxSimple.write( CHANNEL_OFFSET + 13, 15 );
  DmxSimple.write( CHANNEL_OFFSET + 2, 103 );
  DmxSimple.write( CHANNEL_OFFSET + 3, 200 );
}

int value = 0;
int channel;
unsigned long timestamp;

void loop() {
  int c;

  

  //laser
  DmxSimple.write( CHANNEL_OFFSET + 2, random(0, 100) );
  DmxSimple.write( CHANNEL_OFFSET + 3, random(0, 255) );
  Serial.println("TEST");
  delay(5000);
  // moving heads
  // DmxSimple.write(7 + CHANNEL_OFFSET, 8);
  // DmxSimple.write(8 + CHANNEL_OFFSET, 255);
  // for (int i = 0; i < 128; i += 2) {
  //     Serial.println(i);
  //     DmxSimple.write(5 + CHANNEL_OFFSET, i);
  //         delay(1000);
  // }
}
