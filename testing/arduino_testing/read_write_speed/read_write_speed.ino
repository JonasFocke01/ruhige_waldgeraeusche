int testbyte;

void setup() {
  Serial.begin(2000000);

 testbyte = 70;

 pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {

  
    
  while(!Serial.available());

  digitalWrite(LED_BUILTIN, HIGH);
  delay(Serial.read());
digitalWrite(LED_BUILTIN, LOW);
  
}
