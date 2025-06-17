
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
        let mut parenthetical: (Option<String>, Option<Vec<String>>);
        let mut terms: i8;
        let mut paren1 = self.seperate("(");
        println!("{}", self.equation);
        println!("paren1: {:#?}", paren1);
    }
}