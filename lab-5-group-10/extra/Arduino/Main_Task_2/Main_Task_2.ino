/*
 * Giacomo and Silja
 * Lab 5
 */

#define THERMISTORPIN A0

void setup(void) {
  Serial.begin(9600);
}

void loop(void) {
  float reading = analogRead(THERMISTORPIN);
  Serial.print("Analog reading ");
  Serial.println(reading);
  delay(1000);
}
