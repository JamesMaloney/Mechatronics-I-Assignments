/*
 * Giacomo Menchi
 * IA 5
 */

#include <Wire.h>
 
void setup() {
  Serial.begin(9600);
  Wire.begin(29);        
  Wire.onReceive(receiveEvent);
}

void receiveEvent(int howMany) {
  while (Wire.available()) {
    int i = Wire.read();
    Serial.println(i);
  }
}
void loop() {
  delay(100);
}
