// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! The tags extracted from an audio file.

use sndfile::{SndFile, Title, Copyright, Software, Artist, Comment, Date,
              Album, License, TrackNumber, Genre};

/**
 * Structure containing the tags of a sound.
 *
 * If the tags doesn't exist in the sound file, the string is ~"".
 */
#[deriving(Clone, Show, Eq)]
pub struct Tags {
    /// The title of the sound as a StrBuf
    pub title:       StrBuf,
    /// The Copyright of the sound as a StrBuf
    pub copyright:   StrBuf,
    /// The name of the software used to create the sound as a StrBuf
    pub software:    StrBuf,
    /// The name of the artist of the sound as a StrBuf
    pub artist:      StrBuf,
    /// A comment as a StrBuf
    pub comment:     StrBuf,
    /// The creation date of the sound as a StrBuf
    pub date:        StrBuf,
    /// The name of the album where the sound come from as a StrBuf
    pub album:       StrBuf,
    /// The license of the sound as a StrBuf
    pub license:     StrBuf,
    /// The tracknumber of the sound as a StrBuf
    pub trackNumber: StrBuf,
    /// The genre of the sound as a StrBuf
    pub genre:       StrBuf
}

pub fn empty() -> Tags {
    Tags {
        title:       "".to_strbuf(),
        copyright:   "".to_strbuf(),
        software:    "".to_strbuf(),
        artist:      "".to_strbuf(),
        comment:     "".to_strbuf(),
        date:        "".to_strbuf(),
        album:       "".to_strbuf(),
        license:     "".to_strbuf(),
        trackNumber: "".to_strbuf(),
        genre:       "".to_strbuf()
    }
}

pub fn get_sound_tags(file : &SndFile) -> Tags {
    Tags {
        title:       file.get_string(Title).unwrap_or("".to_strbuf()),
        copyright:   file.get_string(Copyright).unwrap_or("".to_strbuf()),
        software:    file.get_string(Software).unwrap_or("".to_strbuf()),
        artist:      file.get_string(Artist).unwrap_or("".to_strbuf()),
        comment:     file.get_string(Comment).unwrap_or("".to_strbuf()),
        date:        file.get_string(Date).unwrap_or("".to_strbuf()),
        album:       file.get_string(Album).unwrap_or("".to_strbuf()),
        license:     file.get_string(License).unwrap_or("".to_strbuf()),
        trackNumber: file.get_string(TrackNumber).unwrap_or("".to_strbuf()),
        genre:       file.get_string(Genre).unwrap_or("".to_strbuf())
    }
}

/// AudioTags trait implemented by all struct who can provides audio.
pub trait AudioTags{
    /// Get the tags of the audio source.
    fn get_tags(&self) -> Tags;
}
