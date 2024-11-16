use ring::digest::{Context, SHA256};

pub fn hash(data: Vec<u8>) -> String {
    let mut context = Context::new(&SHA256);
    context.update(&data);
    let digest = context.finish();
    hex::encode(digest.as_ref())
}

pub async fn img_convert_to_webp(img: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let origin_image = image::load_from_memory(&*img)?.to_rgba8();
    let webp_encoder =
        webp::Encoder::from_rgba(&origin_image, origin_image.width(), origin_image.height());
    let webp_image = webp_encoder.encode(85.0);
    Ok(webp_image.to_vec())
}
