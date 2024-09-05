use image::{DynamicImage, ImageOutputFormat};
use std::fs::File;
use std::io::BufReader;
use webp::Decoder;


pub fn webp_to_png(webp_file_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ouvrir le fichier WebP
    let file = File::open(webp_file_path)?;
    let mut reader = BufReader::new(file);

    // Décoder l'image WebP
    let decoder = Decoder::new(&mut reader);
    let webp_image = decoder.decode()?.to_image();

    // Convertir l'image en DynamicImage
    let dynamic_image = DynamicImage::ImageRgba8(webp_image);

    // Sauvegarder l'image en format PNG
    dynamic_image.save(output_file_path)?;

    Ok(())
}
