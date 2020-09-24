use crate::engine::{
    Context, GameResult,
    util
};
use std::{
    thread,
    fs::File,
    io::{BufReader, Cursor, Read},
    path::Path,
    sync::Arc,
};
use rodio::{
    Sink, Decoder,
    Source as RodioSource,
};

pub trait SoundSource {
    fn play(&mut self) -> GameResult<()>;

    fn set_repeat(&mut self, repeat: bool);
    fn set_volume(&mut self, volume: f32);
}

pub struct Source {
    data: Cursor<Arc<[u8]>>,
    sink: Option<Sink>,

    repeat: bool,
    volume: f32,
}

impl Source {
    pub fn new<P>(_ctx: &mut Context, path: P) -> GameResult<Self>
    where
        P: AsRef<Path>
    {
        // https://github.com/RustAudio/rodio/issues/270

        let path = util::get_final_path(path)?;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut buffer = Vec::new();
        let _ = reader.read_to_end(&mut buffer)?;

        let data = Cursor::new(Arc::from(buffer));

        let source = Source {
            data,
            sink: None,

            repeat: false,
            volume: 0.1,
        };

        Ok(source)
    }
}

impl SoundSource for Source {
    fn play(&mut self) -> GameResult<()> {
        let data = self.data.clone();

        let volume = self.volume;
        let repeat = self.repeat;

        let result: GameResult<Sink> = thread::spawn(move || {
            let device = rodio::default_output_device().unwrap();
            let sink = Sink::new(&device);
            sink.set_volume(volume);

            if repeat {
                let source = Decoder::new(data)?
                    .repeat_infinite();
                sink.append(source);
            } else {
                let source = Decoder::new(data)?;
                sink.append(source);
            }
            

            Ok(sink)
        }).join().unwrap();

        let sink = result?;

        self.sink = Some(sink);
        Ok(())
    }

    fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
}