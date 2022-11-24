#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod place;
#[macro_use]
mod note;
#[macro_use]
mod decay;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde;

#[cfg(feature = "serde")]
pub use self::serde::{DecayDeser, DecayDeserInner, DecayDeserItem};
pub use decay::{Decay, DecayRoot, IntoDecay};
pub use note::Note;
pub use place::{CodePlace, CodePlaceChain};
