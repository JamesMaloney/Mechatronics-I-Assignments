#define FIRSTSTRING "$GPGGA,112503.101,6507.4485,N,02255.5355,W,1,04,2.26,17.7,M,60.6,M,,*44"
#define SECONDSTRING "$GPGSA,A,3,28,30,20,15,,,,,,,,,2.45,2.26,0.93*02"

void setup() {
  Serial.begin(115200);
}

void loop() {
  Serial.println(FIRSTSTRING);
  Serial.println(SECONDSTRING);
  delay(1000);
}
