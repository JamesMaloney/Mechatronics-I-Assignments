#include <SPI.h>
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#define OLED_RESET 4

Adafruit_SSD1306 display(OLED_RESET);

String temp = "45";
String hum = "3";

void setup() {
   display.begin(SSD1306_SWITCHCAPVCC, 0x3C);
   Serial.begin(115200);
}

void loop() {
  if(Serial.available() > 0) {
    temp = Serial.readStringUntil(',');
    hum = Serial.readStringUntil(',');
    delay(100);
  }
  display.clearDisplay();
  drawText(temp, hum);
  drawTemperature( 74, 5, 50, 11, temp.toInt());
  drawHumidity( 74, 17, 50, 11, hum.toInt());
  display.display();
  delay(100);
}

void drawText(String temp, String hum) {
  display.setTextColor(WHITE);
  display.setTextSize(1);
  display.setCursor(5, 7);
  display.print("Temp: ");
  display.print(temp);
  display.print((char)247);
  display.print("C");
  display.setCursor(5, 19);
  display.print("Hum: ");
  display.print(hum);
  display.print(" %");
}

void drawHumidity(int x, int y, int width, int height, int humidity) {
  float bar = ((float)(width-4) / 100) * humidity;
  display.drawRect(x, y, width, height, WHITE);
  display.fillRect(x+2, y+2, bar , height-4, WHITE);
}

void drawTemperature(int x, int y, int width, int height, int temperature) {
  float bar = ((float)(width-4) / 100) * temperature * 2;
  display.drawRect(x, y, width, height, WHITE);
  display.fillRect(x+2, y+2, bar , height-4, WHITE);
}
