#include <Adafruit_NeoPixel.h>

#define BRIGHTNESS 0.25
#define BUTTON_COUNT 11
#define BUTTON_ROW_START_PIN 10
#define BUTTON_ROW_COUNT 3
#define BUTTON_COL_START_PIN 6
#define BUTTON_COL_COUNT 4

unsigned long timestamp = 0;
Adafruit_NeoPixel pixels = Adafruit_NeoPixel(11, A2, NEO_GRB + NEO_KHZ800);

// (rows + special-buttons) * cols
bool button_click_states            [BUTTON_ROW_COUNT + 1][BUTTON_COL_COUNT];
bool button_click_prevent_ghosting  [BUTTON_ROW_COUNT + 1][BUTTON_COL_COUNT];

int fader_read = 0;
int fader_dest = 0;
int saved_fader_dest = 512;
int fader_last_read = 0;
int fader_read_dest_diff = 0;
bool fader_motor_write = true;
int active_color = 10;

void setup() {
  Serial.begin(250000);

  pinMode(BUTTON_ROW_START_PIN + 0, OUTPUT);
  pinMode(BUTTON_ROW_START_PIN + 1, OUTPUT);
  pinMode(BUTTON_ROW_START_PIN + 2, OUTPUT);
  digitalWrite(BUTTON_ROW_START_PIN + 0, HIGH);
  digitalWrite(BUTTON_ROW_START_PIN + 1, HIGH);
  digitalWrite(BUTTON_ROW_START_PIN + 2, HIGH);

  // special button
  pinMode(4, OUTPUT);
  digitalWrite(4, LOW);
  pinMode(2, INPUT_PULLUP);


  pinMode(BUTTON_COL_START_PIN + 0, INPUT_PULLUP);
  pinMode(BUTTON_COL_START_PIN + 1, INPUT_PULLUP);
  pinMode(BUTTON_COL_START_PIN + 2, INPUT_PULLUP);
  pinMode(BUTTON_COL_START_PIN + 3, INPUT_PULLUP);

  // poti
  pinMode(A1, INPUT);

  // motor fader
  pinMode(3, OUTPUT);
  pinMode(5, OUTPUT);

  pixels.begin();
}

void loop() {
  Serial.write(2);

  read_buttons();

  process_logic();

  move_fader();

  write_leds();
}

void process_logic() {
  if (button_click_states[3][0] && !button_click_prevent_ghosting[3][0]) {
      fader_motor_write = false;
      fader_dest = random(0, 1023);
      Serial.write(30);
      Serial.write(1);
    button_click_prevent_ghosting[3][0] = true;
  } else if (!button_click_states[3][0] && button_click_prevent_ghosting[3][0]) {
    Serial.write(30);
    Serial.write(0);
    button_click_prevent_ghosting[3][0] = false;
  }

  if (button_click_states[0][1] && !button_click_prevent_ghosting[0][1]) {
    active_color = 0;
    Serial.write(25);
    Serial.write(1);
    button_click_prevent_ghosting[0][1] = true;
  } else if (!button_click_states[0][1]) {
    button_click_prevent_ghosting[0][1] = false;
  }
  if (button_click_states[0][2] && !button_click_prevent_ghosting[0][2]) {
    active_color = 1;
    Serial.write(24);
    Serial.write(1);
    button_click_prevent_ghosting[0][2] = true;
  } else if (!button_click_states[0][2]) {
    button_click_prevent_ghosting[0][2] = false;
  }
  if (button_click_states[1][0] && !button_click_prevent_ghosting[1][0]) {
    active_color = 2;
    Serial.write(21);
    Serial.write(1);
    button_click_prevent_ghosting[1][0] = true;
  } else if (!button_click_states[1][0]) {
    button_click_prevent_ghosting[1][0] = false;
  }
  if (button_click_states[1][1] && !button_click_prevent_ghosting[1][1]) {
    active_color = 3;
    Serial.write(29);
    Serial.write(1);
    button_click_prevent_ghosting[1][1] = true;
  } else if (!button_click_states[1][1]) {
    button_click_prevent_ghosting[1][1] = false;
  }
  if (button_click_states[1][2] && !button_click_prevent_ghosting[1][2]) {
    active_color = 4;
    Serial.write(26);
    Serial.write(1);
    button_click_prevent_ghosting[1][2] = true;
  } else if (!button_click_states[1][2]) {
    button_click_prevent_ghosting[1][2] = false;
  }
  if (button_click_states[1][3] && !button_click_prevent_ghosting[1][3]) {
    active_color = 9;
    Serial.write(27);
    Serial.write(1);
    button_click_prevent_ghosting[1][3] = true;
  } else if (!button_click_states[1][3]) {
    button_click_prevent_ghosting[1][3] = false;
  }
  if (button_click_states[2][0] && !button_click_prevent_ghosting[2][0]) {
    active_color = 10;
    Serial.write(20);
    Serial.write(1);
    button_click_prevent_ghosting[2][0] = true;
  } else if (!button_click_states[2][0]) {
    button_click_prevent_ghosting[2][0] = false;
  }
  if (button_click_states[2][1] && !button_click_prevent_ghosting[2][1]) {
    active_color = 7;
    Serial.write(28);
    Serial.write(1);
    button_click_prevent_ghosting[2][1] = true;
  } else if (!button_click_states[2][1]) {
    button_click_prevent_ghosting[2][1] = false;
  }
  if (button_click_states[2][2] && !button_click_prevent_ghosting[2][2]) {
    active_color = 8;
    Serial.write(22);
    Serial.write(1);
    button_click_prevent_ghosting[2][2] = true;
  } else if (!button_click_states[2][2]) {
    button_click_prevent_ghosting[2][2] = false;
  }
  if (button_click_states[2][3] && !button_click_prevent_ghosting[2][3]) {
    active_color = 5;
    Serial.write(23);
    Serial.write(1);
    button_click_prevent_ghosting[2][3] = true;
  } else if (!button_click_states[2][3]) {
    button_click_prevent_ghosting[2][3] = false;
  }
}

void write_leds() {
  timestamp = millis();
  int timestamp_led_blinkable = (timestamp / 150) % 2 == 0;
  int timestamp_color_fadeable = timestamp / 50;
  pixels.setPixelColor(0,  pixels.Color(220 * BRIGHTNESS, 200 * BRIGHTNESS,   0 * BRIGHTNESS));
  pixels.setPixelColor(1,  pixels.Color(  0 * BRIGHTNESS, 200 * BRIGHTNESS,   0 * BRIGHTNESS));
  pixels.setPixelColor(9,  pixels.Color( 67 * BRIGHTNESS, 216 * BRIGHTNESS, 201 * BRIGHTNESS));
  pixels.setPixelColor(5,  pixels.Color(  0 * BRIGHTNESS,   0 * BRIGHTNESS, 150 * BRIGHTNESS));
  pixels.setPixelColor(8,  pixels.Color(238 * BRIGHTNESS, 130 * BRIGHTNESS, 238 * BRIGHTNESS));
  pixels.setPixelColor(7,  pixels.Color(255 * BRIGHTNESS, 192 * BRIGHTNESS, 203 * BRIGHTNESS));
  pixels.setPixelColor(2,  pixels.Color(255 * BRIGHTNESS, 165 * BRIGHTNESS,   0 * BRIGHTNESS));
  pixels.setPixelColor(10, pixels.Color(255 * BRIGHTNESS,   0 * BRIGHTNESS,   0 * BRIGHTNESS));
  pixels.setPixelColor(3,  pixels.Color(timestamp_color_fadeable % 255 * BRIGHTNESS, timestamp_color_fadeable % 70 * BRIGHTNESS, timestamp_color_fadeable % 150 * BRIGHTNESS));
  pixels.setPixelColor(4,  pixels.Color(250 * BRIGHTNESS, 250 * BRIGHTNESS, 250 * BRIGHTNESS));

  switch (active_color) {
    case 0 :
      pixels.setPixelColor(0,  pixels.Color(220 * timestamp_led_blinkable * BRIGHTNESS, 200 * timestamp_led_blinkable * BRIGHTNESS,   0 * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 1 :
      pixels.setPixelColor(1,  pixels.Color(  0 * timestamp_led_blinkable * BRIGHTNESS, 255 * timestamp_led_blinkable * BRIGHTNESS,   0 * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 9 :
      pixels.setPixelColor(9,  pixels.Color( 67 * timestamp_led_blinkable * BRIGHTNESS, 216 * timestamp_led_blinkable * BRIGHTNESS, 201 * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 5 :
      pixels.setPixelColor(5,  pixels.Color(  0 * timestamp_led_blinkable * BRIGHTNESS,   0 * timestamp_led_blinkable * BRIGHTNESS, 255 * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 8 :
      pixels.setPixelColor(8,  pixels.Color(238 * timestamp_led_blinkable * BRIGHTNESS, 130 * timestamp_led_blinkable * BRIGHTNESS, 238 * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 7 :
      pixels.setPixelColor(7,  pixels.Color(255 * timestamp_led_blinkable * BRIGHTNESS, 192 * timestamp_led_blinkable * BRIGHTNESS, 203 * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 2 :
      pixels.setPixelColor(2,  pixels.Color(255 * timestamp_led_blinkable * BRIGHTNESS, 165 * timestamp_led_blinkable * BRIGHTNESS,   0 * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 10:
      pixels.setPixelColor(10, pixels.Color(255 * timestamp_led_blinkable * BRIGHTNESS,   0 * timestamp_led_blinkable * BRIGHTNESS,   0 * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 3 :
      pixels.setPixelColor(3,  pixels.Color((timestamp_color_fadeable % 255) * timestamp_led_blinkable * BRIGHTNESS, (timestamp_color_fadeable % 150) * timestamp_led_blinkable * BRIGHTNESS, (timestamp_color_fadeable % 90) * timestamp_led_blinkable * BRIGHTNESS));
      break;
    case 4 :
      pixels.setPixelColor(4,  pixels.Color(250 * timestamp_led_blinkable * BRIGHTNESS, 250 * timestamp_led_blinkable * BRIGHTNESS, 250 * timestamp_led_blinkable * BRIGHTNESS));
      break;
  }
  pixels.show();
}

void read_buttons() {

  for (int i = 0; i < BUTTON_ROW_COUNT; i++) {
    if (i > 0) {
      digitalWrite(BUTTON_ROW_START_PIN + i - 1, HIGH);
    }
    digitalWrite(BUTTON_ROW_START_PIN + i, LOW);
    for (int j = 0; j < BUTTON_COL_COUNT; j++) {
      if (!digitalRead(j + BUTTON_COL_START_PIN)) {
        button_click_states[i][j] = true;
      } else {
        button_click_states[i][j] = false;
      }
      if ( button_click_states[i][j] ) {
        // Serial.print("    ");
        // Serial.print(i);
        // Serial.print("/");
        // Serial.print(j);
        // Serial.print("\n");
      }
    }
  }
  digitalWrite(BUTTON_ROW_START_PIN + BUTTON_ROW_COUNT - 1, HIGH);

  // special button
  if (!digitalRead(2)) {
    button_click_states[3][0] = true;
    // Serial.println("Special button is pressed");
  } else {
    button_click_states[3][0] = false;
  }

  // poti
  fader_read = map(analogRead(A1), 0, 1023, 1023, 0);
  if ((fader_read < fader_last_read - 15 || fader_read > fader_last_read + 15) && !fader_motor_write) {
    Serial.write(31);
    Serial.write(map(fader_read, 0, 1023, 0, 255));
    fader_last_read = fader_read;
  }
  fader_read_dest_diff = abs(fader_read - fader_dest);
}

void move_fader() {
  if ( fader_read_dest_diff > 2 && fader_motor_write ) {
    if ( fader_read < fader_dest ) {
      analogWrite(3, 0);
      analogWrite(5, map(fader_read_dest_diff, 0, 1023, 90, 250));
    } else {
      analogWrite(3, map(fader_read_dest_diff, 0, 1023, 90, 250));
      analogWrite(5, 0);
    }
  } else {
    fader_motor_write = false;
    saved_fader_dest = fader_read;
    analogWrite(3, 0);
    analogWrite(5, 0);
  }
}
