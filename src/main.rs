use id3::{Error, ErrorKind, Tag, TagLike, Frame};

// Using TSO2 - AlbumArtistSortOrder, for our purposes here
const SHORT_ARTIST_TAG_NAME: &str = "TSO2";


fn main() {
    let mut tag = match Tag::read_from_path("C:\\temp\\test_file.mp3") {
        Ok(tag) => tag,
        Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
        Err(..) => return (),
    };
    
    let artist_long_name:&str;
    if let Some(album_artist) = tag.album_artist() {
        println!("album_artist: {}", album_artist);
        artist_long_name = album_artist;
    }
    else if let Some(album_artist) = tag.artist() {
        println!("artist: {}", album_artist);
        artist_long_name = album_artist;
    }
    else {    
        println!("no album_artist");
        return ();
    }

    if let Some(short_artist) = tag.get(SHORT_ARTIST_TAG_NAME) {
        println!("short_artist: {}", short_artist);
    }
    else {
        println!("Setting \"Short artist name\" = {}", artist_long_name.to_string());
        tag.add_frame(Frame::text(SHORT_ARTIST_TAG_NAME, artist_long_name.to_string()));
        println!("added short artist");
        let _ = tag.write_to_path("C:\\temp\\test_file.mp3", id3::Version::Id3v24);
    }
}
