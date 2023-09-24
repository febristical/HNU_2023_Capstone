<<<<<<< HEAD
// No optimization ever

use image::{
    ImageFormat::Png,
    ImageBuffer,
    Luma, ImageResult
};
use iced::{
    widget,
    widget::{slider, container, button, column, row, text, image as imageviewer},
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
    setting.window.size = (320, 252);
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
    rendered: ImageBuffer<Luma<u8>, Vec<u8>>,
    selected: RenderStyle,
    width: u32,
    height: u32,
    angle: u8,
}

impl Renderer {
    fn render(&self) -> ImageResult<()> {
        let angle = Deg(self.angle as f32);
        let position = Point3::new(4.0 * angle.cos(), 4.0 * angle.sin(), 4.0);

        let mut camera = cgmath::vec3(- position.x, -position.y, -position.z + 1.0);
        camera = camera / camera.dot(camera).sqrt();

        let mut xpitch = camera.cross(cgmath::vec3(0.0, 0.0, 1.0));
        let mut ypitch = xpitch.cross(camera);
        xpitch = xpitch / xpitch.dot(xpitch).sqrt();
        ypitch = ypitch / ypitch.dot(ypitch).sqrt();

        let mut stepmap: Vec<u8> = vec![0; 57600];
        let mut distancemap: Vec<f32> = vec![0.0; 57600];
        let mut minmap: Vec<f32> = vec![f32::MAX; 57600];
        let mut reachmap: Vec<bool> = vec![false; 57600];
        let mut steps: u8 = 255;

        for x in 0..320_usize {
            for y in 0..180_usize {
                let index = y * 320 + x;

                let xdisplacement = xpitch * ((x as f32 / 320.0) - 0.5);
                let ydisplacement = ypitch * ((y as f32 / 180.0) - 0.5);

                let mut current_position = position;
                let mut current_direction = camera + xdisplacement + ydisplacement;
                current_direction = current_direction / current_direction.dot(current_direction).sqrt();
                
                'renderpath: for step in 0..steps {
                    let march = self.march(current_position);
                    current_position += (current_direction * march);

                    stepmap[index] += 1;
                    distancemap[index] += march;

                    if march < 0.0001 {
                        reachmap[index] = true;
                        break 'renderpath;
                    } else if march < minmap[index] {
                        minmap[index] = march;
                    }
                }
            }
        }

        let maximum_distance = distancemap
            .iter()
            .zip(
                reachmap.iter()
            ).filter(
                |(_, &reached)|
                reached
            ).fold(0.0,
                |acc, (&marched, _)|
                if acc > marched {
                    marched
                } else {
                    acc
                }
            );
        let minimum_distance = distancemap
            .iter()
            .zip(
                reachmap.iter()
            ).filter(
                |(_, &reached)|
                reached
            ).fold(0.0,
                |acc, (&marched, _)|
                if acc < marched {
                    marched
                } else {
                    acc
                }
            );
        let mut rendered_image = ImageBuffer::new(320, 180);

        for (x, y, pixel) in rendered_image.enumerate_pixels_mut() {
            let index = (y * 320 + x) as usize;
            *pixel = match self.selected {
                RenderStyle::Border => {
                    if reachmap[index] == true {
                        if let Some(luma) = stepmap[index].checked_mul(16) {
                            Luma([luma])
                        } else {
                            Luma([255])
                        }
                    } else {
                        Luma([0])
                    }
                }
                RenderStyle::Distance => {
                    if reachmap[index] == true {
                        let luma = (distancemap[index] - minimum_distance) / (maximum_distance - minimum_distance);
                        if luma < 0.2 {
                            Luma([(luma * 1275.0) as u8])
                        } else {
                            Luma([255])
                        }
                    } else {
                        Luma([0])
                    }
                },
                RenderStyle::Fill => {
                    if reachmap[index] == true {
                        Luma([255])
                    } else {
                        Luma([0])
                    }
                },
                _ => {Luma([0])}
            }
        }

        match self.selected {
            RenderStyle::Border => {
                rendered_image.save("renderedborder.png")
            }
            RenderStyle::Distance => {
                rendered_image.save("rendereddistance.png")
            }
            RenderStyle::Fill => {
                rendered_image.save("renderedfill.png")
            }
            RenderStyle::None => {
                rendered_image.save("renderednone.png")
            }
            RenderStyle::Shadow => {
                rendered_image.save("renderedshadow.png")
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
            rendered: ImageBuffer::new(320, 180),
            selected: RenderStyle::None,
            width: 320,
            height: 180,
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
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let buttonsrow = row![
            button("border").on_press(Message::StyleSelected(RenderStyle::Border)).width(80),
            button("fill").on_press(Message::StyleSelected(RenderStyle::Fill)).width(80),
            button("distance").on_press(Message::StyleSelected(RenderStyle::Distance)).width(80),
            button("shadow").on_press(Message::StyleSelected(RenderStyle::Shadow)).width(80),
        ];
        let slider = slider(0..=179, self.angle, Message::SliderChanged).width(320);
        let description = text(format!("angle: {}, style: {:?}", self.angle, self.selected));
        self.render();
        let rendered_image = imageviewer(
            match self.selected {
                RenderStyle::Border => "renderedborder.png",
                RenderStyle::Distance => "rendereddistance.png",
                RenderStyle::Fill => "renderedfill.png",
                RenderStyle::None => "renderednone.png",
                RenderStyle::Shadow => "renderedshadow.png"
            }
        );

        container(column![description, rendered_image, slider, buttonsrow])
        .width(320)
        .height(254)
        .center_x()
        .center_y()
        .into()
    }
}
=======
// only renderer
>>>>>>> 9c2e7ff5b7e4f8772de47e36d08fa0921f260dab
