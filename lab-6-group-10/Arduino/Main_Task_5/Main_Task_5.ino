int Vo, V1;
void setup() {
    Serial.begin(115200);
}

void loop() {
    int analogValue = analogRead(A0);
    int analogValue2 = analogRead(A1);
    Serial.print(analogValue);
    Serial.print(",");
    Serial.print(analogValue2);
    Serial.print(",");
    delay(50);
}
