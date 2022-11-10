#include <stdlib.h>
#include <Adafruit_NeoPixel.h>

Adafruit_NeoPixel pixels;

unsigned long led_timestamp;

int identifyer;

void setup() {

  Serial.begin(2000000);
  Serial.setTimeout(0);

  pinMode(LED_BUILTIN, OUTPUT);

  identifyer = 2;

  pixels = Adafruit_NeoPixel(150, 3, NEO_RGB + NEO_KHZ800);

  led_timestamp = millis();

  pixels.begin();
}

void loop() {
  digitalWrite(LED_BUILTIN, LOW);
  if (Serial.available() && Serial.parseInt() == 69) {
    digitalWrite(LED_BUILTIN, HIGH);
    delay(10000);
    // Serial.println(Serial.parseInt());
    // Serial.println(Serial.parseInt());
    // Serial.println(Serial.parseInt());
    for (int i = 0; i < 150; i++) {
      pixels.setPixelColor(i, pixels.Color(Serial.read(), Serial.read(), Serial.read()));
    }
    Serial.println("LOOP END");
    pixels.show();
  } else {
    //Serial.println(identifyer);
  }
  
}
