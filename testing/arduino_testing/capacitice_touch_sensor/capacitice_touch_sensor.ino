#include <CapacitiveSensor.h>
CapacitiveSensor Sensor = CapacitiveSensor(4, 5);
long val;
int pos;
#define led 13

void setup()
{
Serial.begin(9600);
pinMode(led, OUTPUT);
pinMode(7, INPUT_PULLUP);
}

void loop()
{
  val = Sensor.capacitiveSensor(10);
  Serial.println(val);
  //Serial.println(digitalRead(7));
}
