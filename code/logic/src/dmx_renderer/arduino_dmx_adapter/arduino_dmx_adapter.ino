#include <DmxSimple.h>

int index;
int incomming_byte;
int incomming_bytes[512];

void setup()
{
  Serial.begin(115200);

  DmxSimple.usePin(6); // digital output for DMX serial data

  pinMode(LED_BUILTIN, OUTPUT);

  index = 0;
}

void loop() {

  // wait for start byte
  while(true){
    while (!Serial.available());
    if (Serial.read() == 69) break;
  }
  // read dmx array data
  for (int i = 0; i < 512; i++) {
    while (!Serial.available());
    incomming_bytes[i] = Serial.read();
  }

  // debug feedback
  for (int i = 0; i < 512; i++) {
    Serial.println(incomming_bytes[i]);
  }

  // write dmx
  for (int i = 0; i < 512; i++) {
    DmxSimple.write(i + 1, incomming_bytes[i]);
  }
}
