/* converter.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use anyhow::anyhow;

#[derive(PartialEq, Eq)]
enum CurrentPart {
    Integer,
    Decimal,
}

pub struct Converter {
    pub from_radix: u8,
    pub to_radix: u8,
    pub decimal_separator: char,
    pub max_decimals: u8,
}

impl Converter {
    pub fn convert(&self, source: &str) -> anyhow::Result<String> {
        let decimal: f64 = self.to_decimal(source)?;
        let to_radix = self.to_radix as f64;

        let mut integer_part = decimal.floor();
        let mut decimal_part = decimal - integer_part;

        /* We'll be using a VecDeque so we can push chars to the back and front */
        let mut result = std::collections::VecDeque::new();

        /* Convert the integer value */
        while integer_part > 0.0 {
            let remainder = integer_part.rem_euclid(to_radix);
            result.push_front(Self::to_char(remainder as u8).unwrap());
            integer_part = integer_part.div_euclid(to_radix);
        }

        if decimal_part == 0.0 {
            return Ok(result.into_iter().collect());
        }

        result.push_back(self.decimal_separator);

        /* Convert the decimal part */
        let mut decimal_count = 0;
        while decimal_part != 0.0 && decimal_count < self.max_decimals {
            let multiplication = decimal_part * to_radix;
            let integer = multiplication.floor();

            result.push_back(Self::to_char(integer as u8).unwrap());
            decimal_part = multiplication - integer;

            decimal_count += 1;
        }

        Ok(result.into_iter().collect())
    }

    fn to_decimal(&self, source: &str) -> anyhow::Result<f64> {
        let mut decimal: f64 = 0.0;
        let mut decimal_power = 0;
        let mut current_part = CurrentPart::Integer;

        let from_radix = self.from_radix as f64;

        /* Use direct subtitution to convert to base 10 */
        for c in source.chars() {
            if c == self.decimal_separator {
                if current_part == CurrentPart::Decimal {
                    return Err(anyhow!("Double decimal separator found"));
                }
                current_part = CurrentPart::Decimal;
                continue;
            }

            let current_value = Self::to_base10(c).ok_or(anyhow!("Unexpected token: {c}."))?;
            if current_value >= from_radix {
                return Err(anyhow!(
                    "Token '{c}' is not valid for base {radix}",
                    radix = from_radix
                ));
            }

            match current_part {
                CurrentPart::Integer => {
                    decimal *= from_radix as f64;
                    decimal += current_value;
                }
                CurrentPart::Decimal => {
                    decimal_power -= 1;
                    decimal += (current_value as f64) * from_radix.powi(decimal_power);
                }
            }
        }

        Ok(decimal)
    }

    fn to_char(val: u8) -> Option<char> {
        /* Rust does not allow to add to chars, so we must create them from u32s */
        match val {
            0..=9 => char::from_u32(val as u32 + 48),
            10..=35 => char::from_u32(val as u32 + 55),
            _ => None,
        }
    }

    fn to_base10(c: char) -> Option<f64> {
        let point: u32 = c.into();
        /*
         * Rust does not allow to subtract from chars, so we must use the
         * unicode codepoint as a u32 to perform the operations we need
         */
        match c {
            '0'..='9' => Some((point - 48).into()),
            'A'..='Z' => Some((point - 55).into()),
            'a'..='z' => Some((point - 87).into()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base10_16() {
        let converter = super::Converter {
            from_radix: 16,
            to_radix: 10,
            decimal_separator: '.',
            max_decimals: 8,
        };

        assert_eq!(converter.convert("fe.1").unwrap(), "254.0625");

        let converter = super::Converter {
            from_radix: 10,
            to_radix: 16,
            decimal_separator: '.',
            max_decimals: 8,
        };
        assert_eq!(converter.convert("254.0625").unwrap(), "FE.1");
    }
}
