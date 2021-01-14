/*
 * Giacomo Menchi
 * IA 5
 */

int x = 0;

void setup()
{
  Serial.begin(115200);
}

void loop() // run over and over
{
  String s = "" + x;
  Serial.println(x);
  x += 1;
  delay(2000);
}
