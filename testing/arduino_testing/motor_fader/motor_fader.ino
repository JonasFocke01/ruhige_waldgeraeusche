// pins
int motorA = 5;
int motorB = 6;
int enablePin = 3;
int button = 41;
int poti = 1;

// config
int target[] = {800, 100, 1000, 0, 500};
int targetsTotal = 5;
int theThreshold = 30;
int movingTimeout = 300;
long myTime = 0;

// programming stuff
int buttonState = 0;         // current state of the button
int lastButtonState = 0;     // previous state of the button
int currentTarget = 0;

void setup() {
  Serial.begin(9600);
  pinMode(enablePin, OUTPUT);
  pinMode(button, INPUT_PULLUP);
}

void loop() {
  Serial.println(analogRead(poti));
  buttonState = digitalRead(button);

  if (buttonState != lastButtonState) {
    if (buttonState == HIGH) {
      currentTarget++;
      if (currentTarget > targetsTotal - 1) currentTarget = 0;
      myTime = millis();      
    }
  }

  if (myTime + movingTimeout > millis()) {
    digitalWrite(enablePin, HIGH);
    if (1000 > (target[currentTarget] + theThreshold) ) {
      analogWrite(motorA, 255);
      analogWrite(motorB, 0);
    } else if (1000 < (target[currentTarget] - theThreshold) ) {
      analogWrite(motorA, 0);
      analogWrite(motorB, 255);
    } else {
      analogWrite(motorA, 0);
      analogWrite(motorB, 0);
    }
  } else {
    digitalWrite(enablePin, LOW);
  }

  lastButtonState = buttonState;
}
