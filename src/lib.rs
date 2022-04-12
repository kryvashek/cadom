#[macro_use]
mod place;
#[macro_use]
mod note;
#[macro_use]
mod decay;

pub use decay::{Decay, IntoDecay};
pub use note::Note;
pub use place::{CodePlace, CodePlaceChain};
