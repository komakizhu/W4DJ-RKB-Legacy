use id3::TagLike;
use id3::frame::Picture;

pub(crate) fn get_image_mime_type(bytes: &[u8]) -> &'static str {
    if bytes.len() < 12 {
        return "image/*";
    }

    match &bytes[..12] {
        [0x89, 0x50, 0x4e, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => "image/png",
        [0xFF, 0xD8, 0xFF, 0xE0 | 0xE1 | 0xE2 | 0xE3 | 0xE8, ..] => "image/jpeg",
        [0x52, 0x49, 0x46, 0x46, _, _, _, _, 0x57, 0x45, 0x42, 0x50] => "image/webp",
        [0x47, 0x49, 0x46, 0x38, ..] => "image/gif",
        [0x42, 0x4d, ..] => "image/bmp",
        _ => "image/*",
    }
}

pub(crate) fn build_id3_tag_from_parts(
    title: &str,
    album: &str,
    artists: &[String],
    image: &[u8],
) -> id3::Tag {
    let mut tag = id3::Tag::new();
    tag.set_title(title);
    tag.set_album(album);
    tag.set_artist(artists.join(", "));

    if !image.is_empty() {
        tag.add_frame(Picture {
            mime_type: get_image_mime_type(image).to_owned(),
            picture_type: id3::frame::PictureType::CoverFront,
            description: String::new(),
            data: image.to_vec(),
        });
    }

    tag
}

pub(crate) fn build_id3_tag_from_flac(tag: &metaflac::Tag) -> id3::Tag {
    let comments = tag.vorbis_comments();
    let title = comments
        .and_then(|block| block.title())
        .and_then(|values| values.first())
        .map(String::as_str)
        .unwrap_or_default();
    let album = comments
        .and_then(|block| block.album())
        .and_then(|values| values.first())
        .map(String::as_str)
        .unwrap_or_default();
    let artists = comments
        .and_then(|block| block.artist())
        .cloned()
        .unwrap_or_default();
    let image = tag
        .pictures()
        .find(|picture| picture.picture_type == metaflac::block::PictureType::CoverFront)
        .or_else(|| tag.pictures().next())
        .map(|picture| picture.data.as_slice())
        .unwrap_or_default();

    build_id3_tag_from_parts(title, album, &artists, image)
}
