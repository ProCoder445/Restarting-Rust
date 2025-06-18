#[derive(Debug)]
pub enum Token {
   Number(f64),
   Add,
   Sub,
   Mul,
   Div,
   LParen,
   RParen,
}

impl Token {
    fn tokenizer(equation: &String) -> Vec<String> {
        equation.chars().for_each(|character| {
            match character {
                '+' => {
                    
                }
            }
        });
        vec!["".to_string()] 
    }
}