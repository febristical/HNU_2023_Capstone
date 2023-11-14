use bridge::respond_to_dart;
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
use with_request::handle_request;
use std::sync::{
    Arc,
};

mod bridge;
mod messages;
mod sample_functions;
mod with_request;
mod linear;

/// This `hub` crate is the entry point for the Rust logic.
/// Always use non-blocking async functions such as `tokio::fs::File::open`.
async fn main() {
    // This is `tokio::sync::mpsc::Reciver` that receives the requests from Dart.
    let mut request_receiver = bridge::get_request_receiver();
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    tokio::spawn(sample_functions::stream_mandelbrot());
    tokio::spawn(sample_functions::run_debug_tests());
    while let Some(request_unique) = request_receiver.recv().await {
        tokio::spawn(async {
            let response_unique = handle_request(request_unique).await;
            respond_to_dart(response_unique);
        });
    }
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
    width: usize,
    height: usize,
    angle: u8,
}

impl Renderer {
    async fn render(&mut self) {
        let angle = self.angle as f32;
        let position = linear::Vector3(4.0 * angle.cos(), 4.0 * angle.sin(), 4.0);
        let (selected, width, height) = (self.selected, self.width, self.height);
        let mut rendered = vec![0; width * height * 4];
        let rendered_arc = Arc::new(Mutex::new(rendered));
        let (tx, mut rx) = mpsc::channel::<()>(1);

        let mut camera = linear::Vector3(- position.0, -position.1, -position.2 + 1.0);
        camera = camera / camera.norm();

        let mut xpitch = camera.cross(linear::Vector3(0.0, 0.0, 1.0));
        let mut ypitch = xpitch.cross(camera);
        xpitch = xpitch / xpitch.dot(xpitch).sqrt();
        ypitch = ypitch / ypitch.dot(ypitch).sqrt();

        let steps: u8 = 24;
        let mut marchedmap = vec![0.0; self.width as usize * self.height as usize];

        let arc_clone = Arc::clone(&rendered_arc);
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let mut rendered = arc_clone.lock().await;

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

            drop(rendered);
            tx_clone.send(()).await.unwrap();
        }).await.unwrap();
        
        rx.recv().await.unwrap();
        self.rendered = rendered_arc.lock().await.clone();
    }
}

fn march(position: linear::Vector3) -> f32 {
    let relative = position - linear::Vector3(0.0, 0.0, 1.0);
    let (projection_radius, altitude) = ((relative.1.powi(2) + relative.2.powi(2)).sqrt(), relative.0);
    ((projection_radius - 0.75).powi(2) + altitude.powi(2)).sqrt() - 0.25
}