use super::equationSynthesizer::EquationSynthesizer;

pub struct Summation {
    equationSynth: EquationSynthesizer
}

impl Summation {
   pub fn new(synthesizer: EquationSynthesizer) -> Summation {
       Summation {equationSynth: synthesizer}
   } 
}