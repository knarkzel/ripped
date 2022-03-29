use crate::*;
use peppi::model::enums::character::External;
use peppi::model::enums::stage::Stage;
use peppi::{model::game::Game, serde::de::Opts};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// Helpers
#[throws]
fn metadata(replay: &PathBuf) -> Game {
    let mut buffer = BufReader::new(File::open(replay)?);
    let opts = Opts { skip_frames: true };
    peppi::game(&mut buffer, Some(opts), None)?
}

pub struct Player {
    pub character: External,
    pub name_tag: Option<String>,
}

// Replay
pub struct Replay {
    pub stage: Stage,
    pub players: Vec<Player>,
}

impl Replay {
    #[throws]
    pub fn new(replay: &PathBuf) -> Self {
        let metadata = metadata(replay)?;
        let stage = metadata.start.stage;
        let players = metadata
            .start
            .players
            .into_iter()
            .map(|it| Player {
                character: it.character,
                name_tag: it.name_tag,
            })
            .collect();
        Self { stage, players }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| 2 + 2);
    }
}
