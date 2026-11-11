#include <Arduino.h>

constexpr uint8_t ACK_OK = 0x06;
constexpr uint8_t ACK_ERR = 0x15;

constexpr u_size MAX_PACKET_SIZE = 22;

const u_size get_packet_size(uint8_t animation_type) {
	switch (animation_type) {
		case 0:
			return 9;
		case 1:
			return 17;
		case 2:
			return 19;
		case 3:
			return 16;
		case 4:
			return 22;
		case 5:
			return 38;
		default:
			return 0;
	}
}

void setup() {
    Serial.begin(115200);
	Serial.setTimeout(50);
}

void loop() {
    if (Serial.available() <= 0) {
		return;
    }

	uint8_t buffer[MAX_PACKET_SIZE];
	buffer[0] = Serial.read();

	size_t packet_size = get_packet_size(buffer[0]);

	if (packet_size == 0) {
		Serial.write(ACK_ERR);
		return;
	}

	size_t bytes_read = Serial.readBytes(reinterpret_cast<char*>(&buffer[1]), packet_size - 1);

	if (bytes_read != packet_size - 1) {
		Serial.write(ACK_ERR);
		return;
	} 

	Serial.write(ACK_OK);
}
