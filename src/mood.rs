/// Struct representing a single mood.
#[derive(Debug, Clone)]
pub struct Mood {
    /// The name of this mood.
    pub name: String,
    /// Level is also the order of a mood, lower = worse.
    pub level: usize,
    /// Hex color of this mood.
    pub color: String,
    /// Lower and upper boundary of this mood, used for stats.
    pub boundaries: (f32, f32),
}

/// Config representing possible moods.
#[derive(Debug, Clone)]
pub struct MoodConfig {
    moods: Vec<Mood>,
}

impl MoodConfig {
    /// Creates a new mood config from a list of tuples containing (name, color).
    pub fn new(moods: Vec<(String, String)>) -> Self {
        let mut ms = vec![];
        let length = moods.len();

        for (level, (name, color)) in moods.into_iter().enumerate() {
            let boundaries = match level {
                0 => (1.0, 1.5),
                x if x == length - 1 => (level as f32 + 0.5, level as f32 + 1.01),
                _ => (level as f32 + 0.5, level as f32 + 1.5),
            };

            let m = Mood {
                name,
                level,
                color,
                boundaries,
            };

            ms.push(m);
        }

        MoodConfig { moods: ms }
    }

    /// Tries to get the [`Mood`] by it's name from this config.
    pub fn get<N: AsRef<str>>(&self, name: N) -> Option<&Mood> {
        let name = name.as_ref();
        self.moods.iter().find(|mood| mood.name == name)
    }
}

impl Default for MoodConfig {
    fn default() -> Self {
        let worst = Mood {
            name: "awful".into(),
            level: 0,
            color: "#6C7679".into(),
            boundaries: (1.0, 1.5),
        };

        let bad = Mood {
            name: "bad".into(),
            level: 1,
            color: "#5579A7".into(),
            boundaries: (1.5, 2.5),
        };

        let meh = Mood {
            name: "meh".into(),
            level: 2,
            color: "#9454A3".into(),
            boundaries: (2.5, 3.5),
        };

        let good = Mood {
            name: "good".into(),
            level: 3,
            color: "#4CA369".into(),
            boundaries: (3.5, 4.5),
        };

        let best = Mood {
            name: "rad".into(),
            level: 4,
            color: "#FF8500".into(),
            boundaries: (4.5, 5.01),
        };

        MoodConfig {
            moods: vec![worst, bad, meh, good, best],
        }
    }
}
