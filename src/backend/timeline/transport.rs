use basedrop::{Handle, Shared, SharedCell};
use rusty_daw_time::SampleTime;

use crate::backend::MAX_BLOCKSIZE;

#[derive(Debug, Clone, Copy)]
pub struct TimelineTransportSaveState {
    pub seek_to: SampleTime,
    pub loop_status: LoopStatus,
}

impl Default for TimelineTransportSaveState {
    fn default() -> Self {
        Self {
            seek_to: SampleTime::new(0),
            loop_status: LoopStatus::Inactive,
        }
    }
}

pub struct TimelineTransportHandle {
    parameters: Shared<SharedCell<Parameters>>,
    coll_handle: Handle,

    seek_to_version: u64,
}

impl TimelineTransportHandle {
    pub fn seek_to(&mut self, seek_to: SampleTime, save_state: &mut TimelineTransportSaveState) {
        save_state.seek_to = seek_to;

        self.seek_to_version += 1;
        let mut params = Parameters::clone(&self.parameters.get());
        params.seek_to = (seek_to, self.seek_to_version);
        self.parameters.set(Shared::new(&self.coll_handle, params));
    }

    pub fn set_status(&mut self, status: TransportStatus) {
        let mut params = Parameters::clone(&self.parameters.get());
        params.status = status;
        self.parameters.set(Shared::new(&self.coll_handle, params));
    }

    /// Set the looping status.
    ///
    /// This will return an error if `loop_end - loop_start` is less than `MAX_BLOCKSIZE` (128).
    pub fn set_loop_status(
        &mut self,
        loop_status: LoopStatus,
        save_state: &mut TimelineTransportSaveState,
    ) -> Result<(), ()> {
        if let LoopStatus::Active {
            loop_start,
            loop_end,
        } = loop_status
        {
            // Make sure loop is valid.
            if loop_end - loop_start < SampleTime::new(MAX_BLOCKSIZE as i64) {
                return Err(());
            }
        }

        save_state.loop_status = loop_status;

        let mut params = Parameters::clone(&self.parameters.get());
        params.loop_status = loop_status;
        self.parameters.set(Shared::new(&self.coll_handle, params));

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Parameters {
    seek_to: (SampleTime, u64),
    status: TransportStatus,
    loop_status: LoopStatus,
}

/// The state of the timeline transport.
pub struct TimelineTransport {
    parameters: Shared<SharedCell<Parameters>>,

    playhead: SampleTime,
    status: TransportStatus,
    loop_status: LoopStatus,

    range_checker: RangeChecker,
    prev_frames: Option<SampleTime>,

    seek_to_version: u64,
}

impl TimelineTransport {
    pub fn new(
        save_state: &TimelineTransportSaveState,
        coll_handle: Handle,
    ) -> (Self, TimelineTransportHandle) {
        // Make sure we are given a valid save state.
        if let LoopStatus::Active {
            loop_start,
            loop_end,
        } = save_state.loop_status
        {
            // Make sure loop is valid.
            assert!(loop_end - loop_start >= SampleTime::new(MAX_BLOCKSIZE as i64));
        }

        let parameters = Shared::new(
            &coll_handle,
            SharedCell::new(Shared::new(
                &coll_handle,
                Parameters {
                    seek_to: (save_state.seek_to, 0),
                    status: TransportStatus::Paused,
                    loop_status: save_state.loop_status,
                },
            )),
        );

        (
            TimelineTransport {
                parameters: Shared::clone(&parameters),
                playhead: save_state.seek_to,
                status: TransportStatus::Paused,
                loop_status: save_state.loop_status,
                range_checker: RangeChecker::Paused,
                prev_frames: None,
                seek_to_version: 0,
            },
            TimelineTransportHandle {
                parameters,
                coll_handle,
                seek_to_version: 0,
            },
        )
    }

    /// Update the state of this transport.
    pub fn update(&mut self, frames: usize) {
        let Parameters {
            seek_to,
            status,
            loop_status,
        } = *self.parameters.get();

        let frames = SampleTime::new(frames as i64);

        // Seek if the gotten a new version of the seek_to value.
        if self.seek_to_version != seek_to.1 {
            self.seek_to_version = seek_to.1;
            self.playhead = seek_to.0;
            self.prev_frames = None;
        }

        self.status = status;
        self.loop_status = loop_status;
        if let TransportStatus::Playing = status {
            // Advance the playhead.
            if let Some(prev_frames) = self.prev_frames {
                let mut did_loop = false;
                if let LoopStatus::Active {
                    loop_start,
                    loop_end,
                } = loop_status
                {
                    if self.playhead < loop_end && self.playhead + prev_frames >= loop_end {
                        let first_frames = loop_end - self.playhead;
                        let second_frames = prev_frames - first_frames;
                        self.playhead = loop_start + second_frames;
                        did_loop = true;
                    }
                }

                if !did_loop {
                    self.playhead += prev_frames;
                }
            }

            self.prev_frames = Some(frames);
        } else {
            self.prev_frames = None;
        }

        self.range_checker = match status {
            TransportStatus::Playing => match loop_status {
                LoopStatus::Inactive => RangeChecker::Playing {
                    end_frame: self.playhead + frames,
                },
                LoopStatus::Active {
                    loop_start,
                    loop_end,
                } => {
                    if self.playhead < loop_end && self.playhead + frames > loop_end {
                        let first_frames = loop_end - self.playhead;
                        let second_frames = frames - first_frames;
                        RangeChecker::Looping {
                            end_frame_1: loop_end,
                            start_frame_2: loop_start,
                            end_frame_2: loop_start + second_frames,
                        }
                    } else {
                        RangeChecker::Playing {
                            end_frame: self.playhead + frames,
                        }
                    }
                }
            },
            _ => RangeChecker::Paused,
        }
    }

    /// The current position of the playhead on the timeline.
    ///
    /// When `status` is of type `Playing`, then this position is of the start of this process
    /// block. (And `playhead + proc_info.frames` is the end position (exclusive) of this process block.)
    #[inline]
    pub fn playhead(&self) -> SampleTime {
        self.playhead
    }

    /// The status of the timeline transport.
    #[inline]
    pub fn status(&self) -> TransportStatus {
        self.status
    }

    /// The status of looping on the timeline transport.
    #[inline]
    pub fn loop_status(&self) -> LoopStatus {
        self.loop_status
    }

    /// Use this to check whether a range of samples lies inside this current process block.
    ///
    /// This will properly handle playing, paused, and looping conditions.
    ///
    /// This will always return false when the transport status is `Paused` or `Clear`.
    ///
    /// * `start` - The start of the range (inclusive).
    /// * `end` - The end of the range (exclusive).
    pub fn is_range_active(&self, start: SampleTime, end: SampleTime) -> bool {
        match self.range_checker {
            RangeChecker::Playing { end_frame } => {
                (start >= self.playhead && start < end_frame)
                    || (end > self.playhead && end <= end_frame)
            }
            RangeChecker::Looping {
                end_frame_1,
                start_frame_2,
                end_frame_2,
            } => {
                (start >= self.playhead && start < end_frame_1)
                    || (end > self.playhead && end <= end_frame_1)
                    || (start >= start_frame_2 && start < end_frame_2)
                    || (end > start_frame_2 && end <= end_frame_2)
            }
            RangeChecker::Paused => false,
        }
    }

    /// Use this to check whether a particular sample lies inside this current process block.
    ///
    /// This will properly handle playing, paused, and looping conditions.
    ///
    /// This will always return false when the transport status is `Paused` or `Clear`.
    pub fn is_sample_active(&self, sample: SampleTime) -> bool {
        match self.range_checker {
            RangeChecker::Playing { end_frame } => sample >= self.playhead && sample < end_frame,
            RangeChecker::Looping {
                end_frame_1,
                start_frame_2,
                end_frame_2,
            } => {
                (sample >= self.playhead && sample < end_frame_1)
                    || (sample >= start_frame_2 && sample < end_frame_2)
            }
            RangeChecker::Paused => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RangeChecker {
    Playing {
        end_frame: SampleTime,
    },
    Looping {
        end_frame_1: SampleTime,
        start_frame_2: SampleTime,
        end_frame_2: SampleTime,
    },
    Paused,
}

/// The status of this transport.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransportStatus {
    /// The transport is currently playing.
    Playing,
    /// The transport is currently paused.
    Paused,
    /// The transport is currently paused, and any active buffers must be cleared.
    Clear,
}

/// The status of looping on this transport.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopStatus {
    /// The transport is not currently looping.
    Inactive,
    /// The transport is currently looping.
    Active {
        /// The start of the loop (inclusive).
        loop_start: SampleTime,
        /// The end of the loop (exclusive).
        loop_end: SampleTime,
    },
}
