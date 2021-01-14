//Defines stepper motor connections and steps per revolution (which is fixed 200 for this motor)
#define dirPin 2
#define stepPin 3
#define stepsPerRevolution 200

void setup() {
  pinMode(stepPin, OUTPUT);
  pinMode(dirPin, OUTPUT);
}

void loop() {
  //Sets the spinning direction clockwise
  digitalWrite(dirPin, HIGH);
  
  //Spins the stepper motor for 2 revolutions fast:
  for(int i = 0; i < 2 * stepsPerRevolution; i++) {
    digitalWrite(stepPin, HIGH);
    delayMicroseconds(500);
    digitalWrite(stepPin, LOW);
    delayMicroseconds(500);
  }
  delay(1000);
  
  //Sets the spinning direction counterclockwise:
  digitalWrite(dirPin, LOW);
  
  //Spins the stepper motor for 2 revolutions fast again:
  for(int i = 0; i < 2 * stepsPerRevolution; i++) {
    digitalWrite(stepPin, HIGH);
    delayMicroseconds(500);
    digitalWrite(stepPin, LOW);
    delayMicroseconds(500);
  }
  delay(1000);
}
