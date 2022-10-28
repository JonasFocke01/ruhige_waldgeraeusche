#define BYTEARRAY_SIZE 16


byte testbyte;

unsigned int debug_timestamp, index = 0;

void setup() {
	Serial.begin(2000000);

 // memset(testbyte, 0x01000001, BYTEARRAY_SIZE / 8);

 // for ( int i = 0; i < BYTEARRAY_SIZE - 1; i++ ) {
 //  testbyte[i] = 0x41;
 // }

 testbyte = 0x42;

  debug_timestamp = millis(); 
}


void loop() {
  Serial.print("B");
}
