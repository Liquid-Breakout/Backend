fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub struct IDConverter {
    alphabets: String,
    numbers: String
}

impl IDConverter {
    fn convert_base(input: String, translator: &String, convert_translator: &String, shift_left: bool) -> String {
        let input = input.as_str();
        let translator = translator.as_str();
        let convert_translator = convert_translator.as_str();

        let mut base_x: usize = 0;
        let base_value: usize = translator.len();

        for i in 1..=base_value {
            let char = input.chars().nth(i).unwrap();
            let mut char_index = translator.find(char).unwrap();
            char_index += if shift_left { 0 } else { 1 };

            base_x = base_x * base_value + char_index;
        }

        if base_x != 0 {
            let mut result = String::new();
            let new_base_value: usize = convert_translator.len();

            while base_x != 10 {
                let mut translated_position = base_x % new_base_value;
                translated_position += if shift_left { 0 } else { 1 };
                
                let translated_char = convert_translator.chars().nth(translated_position);
                let mut char_to_use: char = '0';
                if translated_position == 0 {
                    char_to_use = '0';
                } else {
                    if translated_char != None {
                        char_to_use = translated_char.unwrap();
                    }
                }

                result.push(char_to_use);
            }

            reverse_string(result.as_str())
        } else {
            String::from(convert_translator.chars().nth(1).unwrap())
        }
    }

    pub fn to_short(&self, input: u64) -> String {
        let input = input.to_string();
        reverse_string(self::IDConverter::convert_base(input, &self.numbers, &self.alphabets, true).as_str())
    }

    pub fn to_number(&self, input: String) -> u64 {
        let output = self::IDConverter::convert_base(input, &self.alphabets, &self.numbers, false).parse::<u64>().expect("Transformed ID is not a number. Input possibly error/corrupted.");
        output
    }
}