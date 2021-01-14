/*
 * Giacomo Menchi
 * IA 5
 */

#include <Wire.h>
String s = "";

void setup() {
  Serial.begin(9600);
  Wire.begin(29);
  Wire.onReceive(receiveEvent);
  Wire.onRequest(requestEvent);
}

void receiveEvent(int howMany) {
  while(Wire.available()) {
    char c = Wire.read();
    s+=c;
  }
}

void requestEvent() {
  Wire.write(s.c_str()); 
  s = "";
}

void loop() {
  delay(100);
}
