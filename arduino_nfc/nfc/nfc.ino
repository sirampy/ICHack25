#include <Wire.h>
#include <PN532_I2C.h>
#include <PN532.h>
#include <NfcAdapter.h>

// Define custom I2C pins
#define SDA_PIN 10
#define SCL_PIN 11

PN532_I2C pn532_i2c(Wire);
NfcAdapter nfc = NfcAdapter(pn532_i2c);

void setup() {
    Serial.begin(115200);
    while (!Serial); // Wait for Serial Monitor to open

    // Set custom I2C pins for RP2040
    Wire.setSDA(SDA_PIN);
    Wire.setSCL(SCL_PIN);
    Wire.begin();

    Serial.println("Initializing NFC Module...");
    nfc.begin();
}

void loop() {
    if (nfc.tagPresent()) {
        NfcTag tag = nfc.read();
        Serial.println("NFC Tag Detected!");
        tag.print();
    }
    delay(5000);
}
