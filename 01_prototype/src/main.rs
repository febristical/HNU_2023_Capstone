// No optimization ever

use image::{
    ImageBuffer,
    Luma, ImageResult
};
use iced::{
    widget::{self,
        slider, container, button, column, row, text,
        image::Image, image::Handle},
    Alignment, Element, Sandbox, Settings, theme::Radio
};
use cgmath::{
    prelude::*,
    Point3,
    Vector3,
    Matrix3,
    Deg, point3
};

fn main() -> iced::Result {
    let mut setting = Settings::default();
    setting.window.size = (640, 480);
    Renderer::run(setting)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderStyle {
    Border,
    Fill,
    Distance,
    Shadow,
    None
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StyleSelected(RenderStyle),
    SliderChanged(u8)
}

struct Renderer {
    rendered: Vec<u8>,
    selected: RenderStyle,
    width: u32,
    height: u32,
    angle: u8,
}

impl Renderer {
    fn render(&mut self) {
        let angle = Deg(self.angle as f32);
        let position = Point3::new(4.0 * angle.cos(), 4.0 * angle.sin(), 4.0);

        let mut camera = cgmath::vec3(- position.x, -position.y, -position.z + 1.0);
        camera = camera / camera.dot(camera).sqrt();

        let mut xpitch = camera.cross(cgmath::vec3(0.0, 0.0, 1.0));
        let mut ypitch = xpitch.cross(camera);
        xpitch = xpitch / xpitch.dot(xpitch).sqrt();
        ypitch = ypitch / ypitch.dot(ypitch).sqrt();
        let mut steps: u8 = 64;

        let mut steps: u8 = 24;
        let mut marchedmap = vec![0.0; self.width as usize * self.height as usize];

        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let index = y * self.width as usize + x;
                let mut min = f32::MAX;

                let xdisplacement = xpitch * ((x as f32 / self.width as f32) - 0.5);
                let ydisplacement = ypitch * ((y as f32 / self.height as f32) - 0.5);

                let mut current_position = position;
                let mut current_direction = camera + xdisplacement + ydisplacement;
                current_direction = current_direction / current_direction.dot(current_direction).sqrt();
                
                for step in 0..steps {
                    let march = self.march(current_position);
                    current_position += (current_direction * march);

                    marchedmap[index] += march;

                    if march < 0.005 {
                        (self.rendered[index * 4],
                        self.rendered[index * 4 + 1],
                        self.rendered[index * 4 + 2],
                        self.rendered[index * 4 + 3]) = match self.selected
                        {
                            RenderStyle::Fill => {
                                (255, 255, 255, 255)
                            },
                            RenderStyle::Distance => {
                                let marched = (54.0 / (marchedmap[index] - 3.75)) as u8;
                                (marched, marched, marched, 255)
                            }
                            _ => (0, 0, 0, 255)
                        };

                        break;
                    } else if march < min {
                        min = march;
                    } else if step + 1 == steps || marchedmap[index] > 8.0 {
                        (
                            self.rendered[index * 4],
                            self.rendered[index * 4 + 1],
                            self.rendered[index * 4 + 2],
                            self.rendered[index * 4 + 3]
                        ) = match self.selected {
                            RenderStyle::Border => {
                                let luma = (2.0 / min) as u8;
                                (luma, luma, luma, 255)
                            },
                            _ => (0, 0, 0, 255)
                        };

                        break;
                    }
                }
            }
        }
    }

    fn march(&self, position: Point3<f32>) -> f32 {
        let relative = position - cgmath::point3(0.0, 0.0, 1.0);
        let (projection_radius, altitude) = ((relative.y.powi(2) + relative.z.powi(2)).sqrt(), relative.x);
        ((projection_radius - 0.75).powi(2) + altitude.powi(2)).sqrt() - 0.25
    }
}

impl Sandbox for Renderer {
    type Message = Message;

    fn new() -> Self {
        Self {
            rendered: vec![0; 640 * 360 * 4],
            selected: RenderStyle::None,
            width: 640,
            height: 360,
            angle: 0
        }
    }

    fn title(&self) -> String {
        "Prototype".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::StyleSelected(style) => self.selected = style,
            Message::SliderChanged(angle) => self.angle = angle,
        }

        self.render();
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let buttonsrow = row![
            button("border").on_press(Message::StyleSelected(RenderStyle::Border)).width(80),
            button("fill").on_press(Message::StyleSelected(RenderStyle::Fill)).width(80),
            button("distance").on_press(Message::StyleSelected(RenderStyle::Distance)).width(80),
            button("shadow").on_press(Message::StyleSelected(RenderStyle::Shadow)).width(80),
        ];
        let slider = slider(0..=179, self.angle, Message::SliderChanged).width(640);
        let description = text(format!("angle: {}, style: {:?}", self.angle, self.selected));
        let rendered_image = Image::new(
            Handle::from_pixels(
                640,
                360,
                self.rendered.clone()
            )
        );

        container(column![description, rendered_image, slider, buttonsrow])
        .width(640)
        .height(480)
        .center_x()
        .center_y()
        .into()
    }
}
