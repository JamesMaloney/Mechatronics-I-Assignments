int Vo, V1;
void setup() {
    Serial.begin(115200);
}

void loop() {
    int analogValue = analogRead(A0);
    Serial.print(analogValue);
    Serial.print(",");
    delay(50);
}
