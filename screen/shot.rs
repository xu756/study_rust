use image::RgbaImage;
use xcap::Monitor;

fn screen_shot() -> RgbaImage {
    let monitors = Monitor::all().unwrap();
    return monitors[0].capture_image().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_screen_shot() {
        let image = screen_shot();
        image.save("shot/test.png").unwrap();
    }
}
