use iced::{
    widget::{self, slider, button, column, image, image::Handle, container},
    Alignment, Element, Sandbox, Settings, theme::Radio
};

#[derive(Debug, Clone, Copy)]
struct Radius(u8);

struct Circle {
    radius: Radius,
    pixels: Vec<u8>,
    width: usize,
    height: usize
}


fn main() -> iced::Result {
    let mut setting = Settings::default();
    setting.window.size = (480, 480);
    Circle::run(setting)
}

impl Sandbox for Circle {
    type Message = Radius;

    fn new() -> Self {
        Self {
            radius: Radius(128),
            pixels: vec![0_u8; 320 * 320 * 4],
            width: 320,
            height: 320
        }
    }

    fn title(&self) -> String {
        "test".into()
    }

    fn update(&mut self, message: Self::Message) {
        self.radius = message;

        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
                let fill = match
                (x - 160).pow(2) + (y - 160).pow(2)
                < (self.radius.0 as isize).pow(2)
                {
                        true => 255,
                        false => 0
                };

                let index = ((y as usize * self.height) + x as usize) * 4;

                self.pixels[index] = fill;
                self.pixels[index + 1] = fill;
                self.pixels[index + 2] = fill;
                self.pixels[index + 3] = 255;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let handle = Handle::from_pixels(
                self.width as u32,
                self.height as u32,
                self.pixels.clone()
            );
        let drawed = image::Image::new(handle.clone());

        println!("{:?}", handle.data());

        let slider = slider(0..=255_u8, self.radius.0, Radius).width(255);

        container(column![drawed, slider])
        .width(480)
        .height(480)
        .into()
    }
}