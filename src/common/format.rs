//! Optional TOON format support for serialization
//!
//! This module provides TOON (Token-Oriented Object Notation) serialization support
//! when the `toon_wip` feature is enabled. TOON is a compact, human-readable format
//! that uses indentation-based structure similar to YAML.
//!
//! # Note
//! The Gemini API itself uses JSON for transport. This module is provided for
//! convenience when you want to work with TOON format in your local application code,
//! such as serializing function responses or storing data.
//!
//! # Example
//! ```rust,ignore
//! #[cfg(feature = "toon_wip")]
//! use gemini_rust::common::format::toon;
//!
//! #[cfg(feature = "toon_wip")]
//! {
//!     let data = MyStruct { field: "value" };
//!     let toon_string = toon::to_string(&data)?;
//!     let parsed: MyStruct = toon::from_str(&toon_string)?;
//! }
//! ```

#[cfg(feature = "toon_wip")]
pub mod toon {
    //! TOON format serialization and deserialization functions
    //! 
    //! These are convenience wrappers around the `rtoon` crate.
    
    use serde::{Deserialize, Serialize};

    /// Serialize a value to TOON format string
    pub fn to_string<T: Serialize>(value: &T) -> Result<String, Box<dyn std::error::Error>> {
        rtoon::to_toon(value, None).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// Deserialize a value from TOON format string
    pub fn from_str<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, Box<dyn std::error::Error>> {
        rtoon::from_toon(s, None).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}


