/*
 * Giacomo and Silja
 * Lab 5
 */

int ThermistorPin = 0;
int Vo;
float R1 = 10000;
float logR2, R2, T, Tc, Tf;
float c1 = 1.009249522e-03, c2 = 2.378405444e-04, c3 = 2.019202697e-07;

void setup() {
  Serial.begin(9600);
}

void loop() {

  Vo = analogRead(ThermistorPin);
  R2 = R1 * (1023.0 / (float)Vo - 1.0);
  T = (1.0 / (c1 + c2 * log(R2) + c3 * log(R2) * log(R2) * log(R2)));
  Tc = T - 273.15;
  if (Tc <= 26.0 && Tc >= 0.0){
    Tc = int(((Tc) * 2.0) + 0.5) / 2.0;
    Serial.print("Temperature: ");
    Serial.print(Tc);
    Serial.println(" C");
  }
  else {
    Serial.print("Temperature: ");
    Serial.print(Tc);
    Serial.println(" C");
  }
  delay(500);
}
