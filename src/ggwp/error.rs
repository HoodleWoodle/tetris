use std::{
    path::PathBuf,
    fmt::{self, Formatter},
};

#[derive(Debug)]
pub enum GameError {
    FileNotFound(PathBuf),
    ImageError(image::error::ImageError),
    BadIcon(glium::glutin::window::BadIcon),
    TextureCreationError(glium::texture::TextureCreationError),
    SwapBuffersError(glium::SwapBuffersError),
    DrawError(glium::DrawError),
    DisplayCreationError(glium::backend::glutin::DisplayCreationError),
    ProgramCreationError(glium::ProgramCreationError),
    VertexBufferCreationError(glium::vertex::BufferCreationError),
    IndexBufferCreationError(glium::index::BufferCreationError),
    FreetypeError(freetype::error::Error),
    IoError(std::io::Error),
    DecoderError(rodio::decoder::DecoderError),
    AudioError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<glium::glutin::window::BadIcon> for GameError {
    fn from(from: glium::glutin::window::BadIcon) -> GameError {
        GameError::BadIcon(from)
    }
}

impl From<image::error::ImageError> for GameError {
    fn from(from: image::error::ImageError) -> GameError {
        GameError::ImageError(from)
    }
}

impl From<glium::texture::TextureCreationError> for GameError {
    fn from(from: glium::texture::TextureCreationError) -> GameError {
        GameError::TextureCreationError(from)
    }
}

impl From<glium::SwapBuffersError> for GameError {
    fn from(from: glium::SwapBuffersError) -> GameError {
        GameError::SwapBuffersError(from)
    }
}

impl From<glium::DrawError> for GameError {
    fn from(from: glium::DrawError) -> GameError {
        GameError::DrawError(from)
    }
}

impl From<glium::backend::glutin::DisplayCreationError> for GameError {
    fn from(from: glium::backend::glutin::DisplayCreationError) -> GameError {
        GameError::DisplayCreationError(from)
    }
}

impl From<glium::ProgramCreationError> for GameError {
    fn from(from: glium::ProgramCreationError) -> GameError {
        GameError::ProgramCreationError(from)
    }
}

impl From<glium::vertex::BufferCreationError> for GameError {
    fn from(from: glium::vertex::BufferCreationError) -> GameError {
        GameError::VertexBufferCreationError(from)
    }
}

impl From<glium::index::BufferCreationError> for GameError {
    fn from(from: glium::index::BufferCreationError) -> GameError {
        GameError::IndexBufferCreationError(from)
    }
}

impl From<freetype::error::Error> for GameError {
    fn from(from: freetype::error::Error) -> GameError {
        GameError::FreetypeError(from)
    }
}

impl From<std::io::Error> for GameError {
    fn from(from: std::io::Error) -> GameError {
        GameError::IoError(from)
    }
}

impl From<rodio::decoder::DecoderError> for GameError {
    fn from(from: rodio::decoder::DecoderError) -> GameError {
        GameError::DecoderError(from)
    }
}

pub type GameResult<T = ()> = Result<T, GameError>;