/*
 * Giacomo and Silja
 * Lab 5
 */

#define SERIESRESISTOR 10000
#define LIGHTPIN A0
#define THERMISTORPIN A1

#define A 0.001125308852122
#define B 0.000234711863267
#define C 0.000000085663516
float rangeMax;
float rangeMin;
double tolerance;
int range;
bool set = false;

void setup(void) {
  Serial.begin(115200);
}

void loop(void) {
  String toSend;
  if(Serial.available()>0){
    rangeMin = Serial.readStringUntil(';').toFloat();
    rangeMax = Serial.readStringUntil(';').toFloat();
    tolerance = Serial.readStringUntil(';').toDouble();
    range = 1 / tolerance;
    set = true;
  }

  if(set) {
    float readingThermo;
    float readingLight;
    readingThermo = analogRead(THERMISTORPIN);
    readingLight = analogRead(LIGHTPIN);
    double volt = 5.0 * readingThermo / 1023;
    double rt = SERIESRESISTOR * (5.0 / volt) - SERIESRESISTOR;

    float temp = 1 / (A + (B * log(rt)) + (C * log(rt) * log(rt) * log(rt))) - 273.15;

    if(temp < rangeMax && temp > rangeMin) {
      int x = temp;
      double v = temp - x;
    }
    else {
      int celsius = temp;
    }
    Serial.print(readingLight);
    Serial.print(",");
    Serial.print(temp);
    Serial.print(",");
    delay(10000);
  }
}
