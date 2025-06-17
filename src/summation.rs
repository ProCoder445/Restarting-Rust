use super::equationSynthesizer::EquationSynthesizer;

pub struct Summation<'a> {
    equationSynth: &'a EquationSynthesizer<'a>
}

impl<'a> Summation<'a> {
   pub fn new(synthesizer: &'a EquationSynthesizer) -> Summation<'a> {
       Summation {equationSynth: synthesizer}
   }
}