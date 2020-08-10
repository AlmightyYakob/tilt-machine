#include <HardwareSerial.h>

const int ledPin = 13;

void setup()
{
  pinMode(ledPin, OUTPUT);
  pinMode(2, INPUT);
  digitalWrite(2, HIGH);

  Serial.begin(9600);
  Serial.println("End of setup...");
}

void loop() {
  int digitalVal = digitalRead(2);
  if (HIGH == digitalVal) {
    digitalWrite(ledPin, LOW); //turn the led off
    Serial.println("0");
  }
  else {
    digitalWrite(ledPin, HIGH); //turn the led on
    Serial.println("1");
  }
}
