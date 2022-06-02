//! # Program Layer - (The middle layer)
//!
//! This layer owns the actual state of the program.
//!
//! It is solely in charge of mutating this state. The backend layer and
//! the UI layer cannot mutate this state directly (with the exception of
//! some UI-specific state that does not need to be undo-able such as
//! panel or window size). The backend layer indirectly mutates this state
//! by sending events, and the ui layer indirectly mutates this state by
//! calling methods on the ProgramState struct. The program layer is in
//! charge of handling these events and properly mutating the state accordingly.
//!
//! The program layer owns the audio thread and is in charge of
//! connecting to the system's audio and MIDI devices. It also owns the
//! handle to the BackendLayerHandle struct.
//!
//! The program layer is also in charge of some offline DSP such as
//! resampling audio clips.

pub mod events;
pub mod program_state;

pub use program_state::ProgramState;

/// This is in charge of keeping track of state for the whole program.
///
/// The UI must continually call `ProgramLayer::poll()` (on every frame or an
/// equivalent timer).
pub struct ProgramLayer {
    /// The state of the whole program.
    ///
    /// Unless explicitely stated, the UI may NOT directly mutate the state of any
    /// of these variables. It is intended for the UI to call the methods on this
    /// struct in order to mutate state.
    pub state: ProgramState,
}

impl ProgramLayer {
    pub fn new() -> Result<Self, ()> {
        todo!()
    }

    pub fn poll(&mut self) {
        // TODO
    }
}
