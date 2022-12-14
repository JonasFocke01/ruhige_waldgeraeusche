#define INPUT_BUTTON 50

int button_states[5] = {1, 2, 3, 5, 4};

unsigned long render_timestamp;

void setup() {
  Serial.begin(1000000);

 pinMode(LED_BUILTIN, OUTPUT);

 pinMode(INPUT_BUTTON, INPUT_PULLUP);

 render_timestamp = millis();
}

void loop() {
  if ((!digitalRead(INPUT_BUTTON) && millis() - render_timestamp > 200) || millis() - render_timestamp > 1000) {
    Serial.write( 96 );
    for ( int i = 0; i < sizeof( button_states ) / 2; i++ ) {
      Serial.write( button_states[i] );
    }
    render_timestamp = millis();
  }
}
