/*
 * Giacomo and Silja
 * Lab 5
 */

int ThermistorPin = 0;
int Vo;
float tolerance;
float R1 = 10000;
float logR2, R2, T, Tc, Tf;
float c1 = 1.009249522e-03, c2 = 2.378405444e-04, c3 = 2.019202697e-07;
String x, y, z;
void setup() {
    Serial.begin(115200);
}

void loop() {
  if (Serial.available()>0){
     x = Serial.readStringUntil(',');
     y = Serial.readStringUntil(',');
     z = Serial.readStringUntil(',');
       tolerance = 1 / z.toFloat();
  }
  Vo = analogRead(ThermistorPin);
  R2 = R1 * (1023.0 / (float)Vo - 1.0);
  T = (1.0 / (c1 + c2 * log(R2) + c3 * log(R2) * log(R2) * log(R2)));
  Tc = T - 273.15;
  if (Tc <= y.toFloat() && Tc >= x.toFloat()){
    Tc = int((((Tc) * tolerance) + z.toFloat())) / tolerance;
    Serial.print("Temperature: ");
    Serial.print(Tc);
    Serial.println(" C");
  } else {
    Serial.print("Temperature: ");
    Serial.print(Tc);
    Serial.println(" C");
  }
  delay(500);
}
