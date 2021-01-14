#include <SPI.h>
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#define OLED_RESET 4

Adafruit_SSD1306 display(OLED_RESET);

String temp = "0";
String hum = "0";

void setup() {
  Serial.begin(115200);
  display.begin(SSD1306_SWITCHCAPVCC, 0x3C);
}

void loop() {
  if(Serial.available() > 0) {
    temp = Serial.readStringUntil(',');
    hum = Serial.readStringUntil(',');
    delay(100);
  }
  // Clear the buffer.
  display.clearDisplay();
  display.setTextColor(WHITE);
  display.setTextSize(1);
  display.setCursor(10, 5);
  display.print("Temp: ");
  display.print(temp);
  display.print(" Celsius");
  display.setCursor(10, 20);
  display.print("Hum: ");
  display.print(hum);
  display.print(" %");
  display.display();
  delay(100);
}
