#include <Wire.h>
#include <PN532_I2C.h>
#include <PN532.h>
#include <NfcAdapter.h>

#define NDEF_USE_SERIAL

PN532_I2C pn532_i2c(Wire);
NfcAdapter nfc = NfcAdapter(pn532_i2c);
String tagId = "2424";
byte nuidPICC[4];
 
void setup(void) 
{
  Wire.begin(10, 11); // Custom I2C pins (SDA=GPI10, SCL=GPI11 for ESP32/RP2040)
  Serial.begin(115200);
  Serial.println("System initialized");
  nfc.begin();
}
 
void loop() 
{
 readNFC();
}
 
void readNFC() 
{
 if (nfc.tagPresent())
 {
   NfcTag tag = nfc.read();
   tag.print();
   tagId = tag.getUidString();
 }
 delay(5000);
}