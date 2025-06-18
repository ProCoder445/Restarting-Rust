mod equationSynthesizer;
mod summation;
mod Tokenizer;

use equationSynthesizer::EquationSynthesizer;
use summation::Summation;


fn main() {
    let mut synthesizer = EquationSynthesizer::new(String::from("i+(i)"));
    let summation = Summation::new(&synthesizer);
    
    synthesizer.parse();
}
