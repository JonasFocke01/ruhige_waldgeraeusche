
#include "Arduino.h" 
#include<DmxSimple.h>

#define DMX_DATA_PIN 3

void setup() {
  
  Serial.begin(9600);
  
  DmxSimple.usePin(DMX_DATA_PIN);

  pinMode(LED_BUILTIN, OUTPUT);
  
}

void loop() {

  while(!Serial.available());
  if (Serial.read() == '6'){
    delay(30);
  Serial.write(byte(69));
  delay(30);
  if (Serial.read() == '67') {
    Serial.write(byte(67));
    if (Serial.read() == '125'){
      Serial.write(byte(125));
      if (Serial.read() == '254'){
        Serial.write(byte(254));
        }
    }
  }
  }
  }
