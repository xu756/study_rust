use active_win_pos_rs::get_active_window;
use image::RgbaImage;
use xcap::Window;

fn screen_shot() -> RgbaImage {
    let windows = Window::all().unwrap();
    let name;
    match get_active_window() {
        Ok(active_window) => {
            name = active_window.app_name;
        }
        Err(e) => {
            return RgbaImage::new(100, 100);
        }
    }
    for window in windows {
        println!("name: {:?}", window.app_name());
        if window.app_name() == name {
            return window.capture_image().unwrap();
        }
    }

    return RgbaImage::new(100, 100);
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
