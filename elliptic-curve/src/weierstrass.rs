//! Elliptic curves in short Weierstrass form.

pub mod point;
pub mod public_key;

pub use self::{
    point::{CompressedPoint, CompressedPointSize, UncompressedPoint, UncompressedPointSize},
    public_key::PublicKey,
};

use crate::{consts::U1, ScalarBytes};
use core::ops::Add;
use generic_array::ArrayLength;
use subtle::{ConditionallySelectable, CtOption};

#[cfg(feature = "rand_core")]
use crate::secret_key::SecretKey;

#[cfg(feature = "rand_core")]
use rand_core::{CryptoRng, RngCore};

/// Marker trait for elliptic curves in short Weierstrass form
pub trait Curve: super::Curve {}

/// Fixed-base scalar multiplication
pub trait FixedBaseScalarMul: Curve
where
    Self::ElementSize: Add<U1>,
    <Self::ElementSize as Add>::Output: Add<U1>,
    CompressedPoint<Self>: From<Self::AffinePoint>,
    UncompressedPoint<Self>: From<Self::AffinePoint>,
    CompressedPointSize<Self>: ArrayLength<u8>,
    UncompressedPointSize<Self>: ArrayLength<u8>,
{
    /// Affine point type for this elliptic curve
    type AffinePoint: ConditionallySelectable;

    /// Multiply the given scalar by the generator point for this elliptic
    /// curve.
    fn mul_base(scalar: &ScalarBytes<Self>) -> CtOption<Self::AffinePoint>;
}

/// Generate a secret key for this elliptic curve
#[cfg(feature = "rand_core")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
pub trait GenerateSecretKey: Curve {
    /// Generate a random [`SecretKey`] for this elliptic curve using the
    /// provided [`CryptoRng`]
    fn generate_secret_key(rng: impl CryptoRng + RngCore) -> SecretKey<Self>;
}
