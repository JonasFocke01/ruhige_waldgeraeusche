#include <DmxSimple.h>

int index;
int incomming_byte;

void setup()
{
  Serial.begin(2000000);

  DmxSimple.usePin(6); // digital output for DMX serial data

  pinMode(LED_BUILTIN, OUTPUT);

  index = 1;
}

void loop() {

  while (!Serial.available()){}

  incomming_byte = Serial.read();
  if (incomming_byte == 69) {
    index = 1;
  } else if (index < 513) {
    DmxSimple.write(index, incomming_byte);
    index += 1;
  }
}
