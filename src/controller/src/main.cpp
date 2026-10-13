#include <Arduino.h>

void setup() {
    Serial.begin(115200);
}

void loop() {
    if (Serial.available() > 0) {
	uint8_t value = Serial.read();

	if (value == 0xAA) {
	    Serial.write(0x55);
	}
    }
}
