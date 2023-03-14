#define CLK   4
#define LATCH 3
#define DATA  2

#define DELAY 1     // Adjust this delay if you get any instabilities

void setup() {
  Serial.begin(9600);

  pinMode(DATA, INPUT);

  pinMode(LATCH, OUTPUT);
  pinMode(CLK, OUTPUT);
}

void pulse(uint16_t pin, uint16_t delayMilis) {
  digitalWrite(pin, HIGH);
  delay(delayMilis);     // This might be unnecessary but it did not produce any noticable input delay
  digitalWrite(pin, LOW);
}

void loop() {
  pulse(LATCH, DELAY);

  int buff = 0;

  for (int i = 0; i < 8; i++) {
    if(!digitalRead(DATA)) {
      buff |= 1 << 7 - i;
    }

    pulse(CLK, DELAY);
  }

  Serial.write(buff);
}
