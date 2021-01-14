/*
 * Giacomo and Silja
 * Lab 5
 */

#define THERMISTORPIN A0
#define LIGHTSENSOR A1

int therm;
int light;

void setup(void) {
    Serial.begin(115200);
}

void loop(void) {
  //thermistor
  therm = analogRead(THERMISTORPIN);
  Serial.print("Thermistor reading ");
  Serial.println(therm);

  //light sensor
  int light = analogRead(LIGHTSENSOR);
  Serial.print("Light sensor reading ");
  Serial.println(light);

  delay(500);
}
