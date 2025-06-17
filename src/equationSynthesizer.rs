
pub struct EquationSynthesizer<'a> {
    equation: String,
    parts: Option<Vec<&'a str>>
}

impl<'a> EquationSynthesizer<'a> {
   pub fn new(equation: String) -> Self {
       EquationSynthesizer {equation, parts: Option::None}
   }

    pub fn seperate(self: &Self, pattern: &str) -> Vec<&str> {
       self.equation.split(pattern).collect()
    }

    pub fn parse(self: &mut Self) {

        let parts = self.equation.chars().collect::<Vec<char>>();

        let mut letter= ' ';

        parts.iter().for_each(|&part| letter = part )
        
        


    }
}