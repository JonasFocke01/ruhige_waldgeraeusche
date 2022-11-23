#include <CapacitiveSensor.h>
CapacitiveSensor Sensor = CapacitiveSensor(6, 7);
long val;
int pos;
#define led 13

void setup()
{
Serial.begin(9600);
pinMode(led, OUTPUT);
}

void loop()
{
  val = Sensor.capacitiveSensor(10);
  Serial.println(val);
  //Serial.println(digitalRead(7));
}
