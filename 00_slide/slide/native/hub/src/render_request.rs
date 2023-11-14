use crate::bridge::send_rust_signal;
use crate::bridge::{
    RustOperation,
    RustRequest,
    RustResponse,
    RustSignal
};
use tokio_with_wasm::tokio;

use tokio::{
    sync::{
        mpsc,
        mpsc::{
            Sender,
            Receiver
        },
        Mutex
    }
};

use std::sync::{
    Arc,
};

use prost::Message;

use crate::linear;

pub async fn handle_render_request(rust_request: RustRequest) -> RustResponse {
    use crate::messages::render_request::{RenderRequest, RenderResponse};


    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => RustResponse::default(),
        RustOperation::Update => {
            let message_bytes = rust_request.message.unwrap();
            let request_message: RenderRequest = RenderRequest::decode(message_bytes.as_slice()).unwrap();
        
            let angle = request_message.angle;
            let style = request_message.style;
        
            let renderer = Renderer {
                selected: match style {
                    1 => RenderStyle::Border,
                    2 => RenderStyle::Distance,
                    _ => RenderStyle::Fill,
                },
                width: 640,
                height: 480,
                angle: angle,
            };

            let image = renderer.render();

            let mut response_message = RenderResponse {
                image: image,
            };

            RustResponse {
                successful: true,
                message: Some(response_message.encode_to_vec()),
                blob: None,
            }
        },
        RustOperation::Delete => RustResponse::default(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderStyle {
    Fill,
    Border,
    Distance,
}

struct Renderer {
    selected: RenderStyle,
    width: usize,
    height: usize,
    angle: f64,
}

impl Renderer {
    fn render(&self) -> Vec<u8> {
        let angle = self.angle as f32;
        let position = linear::Vector3(4.0 * angle.cos(), 4.0 * angle.sin(), 4.0);
        let (selected, width, height) = (self.selected, self.width, self.height);
        let mut rendered = vec![0; width * height * 4];

        let mut camera = linear::Vector3(- position.0, -position.1, -position.2 + 1.0);
        camera = camera / camera.norm();

        let mut xpitch = camera.cross(linear::Vector3(0.0, 0.0, 1.0));
        let mut ypitch = xpitch.cross(camera);
        xpitch = xpitch / xpitch.dot(xpitch).sqrt();
        ypitch = ypitch / ypitch.dot(ypitch).sqrt();

        let steps: u8 = 24;
        let mut marchedmap = vec![0.0; self.width as usize * self.height as usize];

        for x in 0..width as usize {
            for y in 0..height as usize {
                let index = y * width as usize + x;
                let mut min = f32::MAX;

                let xdisplacement = ((x as f32 / width as f32) - 0.5) * xpitch;
                let ydisplacement = ((y as f32 / height as f32) - 0.5) * ypitch;

                let mut current_position = position;
                let mut current_direction = camera + xdisplacement + ydisplacement;
                current_direction = current_direction / current_direction.dot(current_direction).sqrt();
                
                for step in 0..steps {
                    let march = march(current_position);
                    current_position += (march * current_direction);

                    marchedmap[index] += march;

                    if march < 0.005 {
                        (rendered[index * 4],
                        rendered[index * 4 + 1],
                        rendered[index * 4 + 2],
                        rendered[index * 4 + 3]) = match selected
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
                            rendered[index * 4],
                            rendered[index * 4 + 1],
                            rendered[index * 4 + 2],
                            rendered[index * 4 + 3]
                        ) = match selected {
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

        rendered
    }
}

fn march(position: linear::Vector3) -> f32 {
    let p = position - linear::Vector3(0.0, 0.0, 1.0);
    let (r, z) = ((p.1.powi(2) + p.2.powi(2)).sqrt(), p.0);
    return ((r - 0.75).powi(2) + z.powi(2)).sqrt() - 0.25;
}