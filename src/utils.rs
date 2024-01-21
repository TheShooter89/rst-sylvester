use local_ip_address::local_ip;
use qrcode::QrCode;

use image::Luma;

pub fn generate_qrcode_link() -> Result<(), std::io::Error> {
    let my_local_ip = local_ip().unwrap();
    let website_address_string = format!("http://{}:3000/about", my_local_ip);

    let qrcode = QrCode::new(website_address_string.into_bytes()).unwrap();

    let image = qrcode.render::<Luma<u8>>().min_dimensions(200, 200).build();
    image.save("static/img/qrcode_link.png").unwrap();

    Ok(())
}
