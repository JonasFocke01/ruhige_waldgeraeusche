int button_states[5] = {1, 2, 3, 5, 4};

void setup() {
  Serial.begin(2000000);

 pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {

  Serial.write( 96 );
  for ( int i = 0; i < sizeof( button_states ) / 2; i++ ) {
    Serial.write( button_states[i] );
  }

  delay(1000);
    
//   while(!Serial.available());

//   digitalWrite(LED_BUILTIN, HIGH);
//   delay(Serial.read());
//     digitalWrite(LED_BUILTIN, LOW);
  
}
