/*
 * Giacomo Menchi
 * IA 5
 */

#include <Wire.h> 
int x;

void setup() {
  Wire.begin(29);
  Serial.begin(9600);
  Wire.onReceive(receiveEvent);
  Wire.onRequest(sendEvent);
}

void receiveEvent(int howMany)
{
  while(Wire.available() > 1)
  {
    char c = Wire.read();
  }
  x = Wire.read();
  Serial.println(x); 
}

void sendEvent(){
  Wire.write(++x);
}

void loop() {
  delay(100);
}
