// SPDX-License-Identifier: CC0-1.0

//! Bitcoin block data.
//!
//! This module defines structures and functions for storing the blocks and
//! transactions which make up the Bitcoin system.

pub mod block;
pub mod constants;
pub mod opcodes;
pub mod script;
pub mod transaction;
pub mod witness;

#[rustfmt::skip]                // Keep public re-exports separate.
#[doc(inline)]
pub use self::{
    fee_rate::FeeRate,
    weight::Weight
};

/// Implements `FeeRate` and associated features.
pub mod fee_rate {
    #[cfg(feature = "serde")]
    pub use units::fee_rate::serde;
    /// Re-export everything from the [`units::fee_rate`] module.
    pub use units::fee_rate::FeeRate;
}

/// Provides absolute and relative locktimes.
pub mod locktime {
    pub mod absolute {
        //! Provides type [`LockTime`] that implements the logic around nLockTime/OP_CHECKLOCKTIMEVERIFY.
        //!
        //! There are two types of lock time: lock-by-height and lock-by-time, distinguished by
        //! whether `LockTime < LOCKTIME_THRESHOLD`.

        use io::{BufRead, Write};

        pub use crate::consensus::encode::{self, Decodable, Encodable};

        /// Re-export everything from the `units::locktime::absolute` module.
        #[rustfmt::skip]        // Keep public re-exports separate.
        pub use units::locktime::absolute::{ConversionError, Height, LockTime, ParseHeightError, ParseTimeError, MedianTimePast};

        #[deprecated(since = "TBD", note = "use `MedianTimePast` instead")]
        #[doc(hidden)]
        pub type Time = MedianTimePast;

        impl Encodable for LockTime {
            #[inline]
            fn consensus_encode<W: Write + ?Sized>(&self, w: &mut W) -> Result<usize, io::Error> {
                let v = self.to_consensus_u32();
                v.consensus_encode(w)
            }
        }

        impl Decodable for LockTime {
            #[inline]
            fn consensus_decode<R: BufRead + ?Sized>(r: &mut R) -> Result<Self, encode::Error> {
                u32::consensus_decode(r).map(LockTime::from_consensus)
            }
        }
    }

    pub mod relative {
        //! Provides type [`LockTime`] that implements the logic around nSequence/OP_CHECKSEQUENCEVERIFY.
        //!
        //! There are two types of lock time: lock-by-height and lock-by-time, distinguished by
        //! whether bit 22 of the `u32` consensus value is set.

        /// Re-export everything from the `units::locktime::relative` module.
        pub use units::locktime::relative::{
            DisabledLockTimeError, InvalidHeightError, InvalidTimeError, LockTime,
            NumberOf512Seconds, NumberOfBlocks, TimeOverflowError,
        };

        #[deprecated(since = "TBD", note = "use `NumberOfBlocks` instead")]
        #[doc(hidden)]
        pub type Height = NumberOfBlocks;

        #[deprecated(since = "TBD", note = "use `NumberOf512Seconds` instead")]
        #[doc(hidden)]
        pub type Time = NumberOf512Seconds;
    }
}

/// Implements `Weight` and associated features.
pub mod weight {
    /// Re-export everything from the [`units::weight`] module.
    pub use units::weight::Weight;
}
