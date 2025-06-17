mod equationSynthesizer;
mod summation;

use equationSynthesizer::EquationSynthesizer;
use summation::Summation;


fn main() {
    let synthesizer = EquationSynthesizer::new();
    let summation = Summation::new(synthesizer);    
}
