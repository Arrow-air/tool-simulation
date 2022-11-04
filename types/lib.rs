//! Types used in Simulations

/// Types used for EEL-driven simulations
pub mod eel_types {
    include!("eel/types.rs");
}

/// Types used for configuration-file-driven simulations
pub mod cfg_types {
    include!("cfg/types.rs");
}
