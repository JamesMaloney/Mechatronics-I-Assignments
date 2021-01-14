/*
 * Giacomo and Silja
 * Lab 5
 */

#define Light A0

const int avgCount = 16;
float runningAvgBuffer[avgCount];
int nextRunAvg;

void setup() {
  Serial.begin(9600);
}

void loop() {
  int analogValueLight = analogRead(Light);
  runningAvgBuffer[nextRunAvg++] = analogValueLight;
  if (nextRunAvg >= avgCount) {
    nextRunAvg = 0;
  }

  float runningAvgLight = 0;
  for(int i = 0; i < avgCount; ++i) {
    runningAvgLight += runningAvgBuffer[i];
  }
  runningAvgLight /= avgCount;
  Serial.print("Light: ");
  Serial.println(runningAvgLight);
  delay(100);
}
