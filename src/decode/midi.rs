use midly::{MidiMessage, Smf, TrackEventKind};
use std::{cmp, fs, path::Path};

pub type Tracks = Vec<Vec<Note>>;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MidiTracks {
    pub(crate) tracks: Tracks,
    pub(crate) duration: u32,
    pub(crate) lowest_note: u8,
    pub(crate) highest_note: u8,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Note {
    pub(crate) start: u32,
    pub(crate) duration: u32,
    pub(crate) key: u8,
}

pub fn decode_midi<P: AsRef<Path>>(path: &P) -> anyhow::Result<MidiTracks> {
    let bytes = fs::read(path.as_ref())?;
    let smf = Smf::parse(&bytes)?;

    Ok(get_tracks(&smf))
}

fn get_tracks(smf: &Smf) -> MidiTracks {
    let mut tracks: Tracks = Vec::new();

    let mut lowest_note = 128;
    let mut highest_note = 0;
    let mut duration = 0;

    smf.tracks.iter().for_each(|track| {
        // (key, delta)
        type NoteData = (u8, u32);

        let mut absolute_time = 0;
        let mut active_notes = Vec::<NoteData>::new();
        let mut current_track = Vec::<Note>::new();

        for event in track.iter() {
            absolute_time += event.delta.as_int();

            if let TrackEventKind::Midi {
                channel: _,
                message,
            } = event.kind
            {
                match message {
                    MidiMessage::NoteOn { key, vel } => {
                        let key = key.as_int();

                        if vel > 0 {
                            active_notes.push((key, absolute_time));
                        } else if let Some(i) = active_notes.iter().position(|(k, _)| *k == key) {
                            let (key, start_time) = active_notes.remove(i);
                            let duration = absolute_time - start_time;

                            lowest_note = cmp::min(lowest_note, key);
                            highest_note = cmp::max(highest_note, key);

                            current_track.push(Note {
                                key,
                                start: start_time,
                                duration,
                            });
                        }
                    }
                    MidiMessage::NoteOff { key, vel: _ } => {
                        if let Some(i) = active_notes.iter().position(|(k, _)| *k == key) {
                            let (key, start_time) = active_notes.remove(i);
                            let duration = absolute_time - start_time;

                            lowest_note = cmp::min(lowest_note, key);
                            highest_note = cmp::max(highest_note, key);

                            current_track.push(Note {
                                key,
                                start: start_time,
                                duration,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        duration = cmp::max(duration, absolute_time);
        tracks.push(current_track);
    });

    MidiTracks {
        tracks,
        lowest_note,
        highest_note,
        duration,
    }
}
