#[derive(Copy, Clone, Debug)]
pub enum Rate {
    PowerSalving,
    Office,
    Gaming,
    ESport,
    K2,
    K4,
    K8,
}

#[derive(Copy, Clone)]
pub struct PollingRateBuilder {
    pub rate: Rate,
    buffer: [u8; 9],
}

impl PollingRateBuilder {
    pub fn new(rate: Rate) -> Self {
        let rate_to_hex = PollingRateBuilder::rate_to_hex(&rate);

        PollingRateBuilder {
            buffer: [
                0x06,
                0x09,
                0x01,
                rate_to_hex,
                rate_to_hex ^ 0xff,
                0x00,
                0x00,
                0x00,
                0x00,
            ],
            rate,
        }
    }

    pub fn rate_to_hex(rate: &Rate) -> u8 {
        match rate {
            Rate::PowerSalving => 0x08,
            Rate::Office => 0x04,
            Rate::Gaming => 0x02,
            Rate::ESport => 0x01,
            Rate::K2 => 0x16,
            Rate::K4 => 0x32,
            Rate::K8 => 0x48,
        }
    }

    pub fn set_rate(&mut self, rate: Rate) {
        let rate_to_hex = PollingRateBuilder::rate_to_hex(&rate);

        self.buffer[3] = rate_to_hex;
        self.buffer[4] = rate_to_hex ^ 0xff;

        self.rate = rate;
    }
}
