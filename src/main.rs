#![feature(test)]

#[allow(unused_imports)]
mod benchmark;
mod renderer;
mod types;

use crate::renderer::core::color;
use crate::renderer::core::vec2::Vec2;
use crate::renderer::core::vec3::{Color, Point3, Vec3};
use crate::renderer::render::{self, RenderParams, Scene};
use crate::renderer::scene::camera::Camera;
use crate::renderer::scene::sphere::Sphere;
use crate::types::result::Result;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;

const MAX_BOUNCES: usize = 50;
const SAMPLES_PER_PIXEL: usize = 100;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const IMAGE_PIXELS: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

const LAST_PIXEL_X: usize = IMAGE_WIDTH - 1;
const LAST_PIXEL_Y: usize = IMAGE_HEIGHT - 1;

enum RenderMessage {
  Pixel(usize, Color),
  ScanlineComplete,
}

struct RenderThreadContext {
  thread_id: usize,
  threads: usize,
  scanlines_per_thread: usize,
  scanline_max: usize,
  sender: Sender<RenderMessage>,
}

fn render_thread(
  context: RenderThreadContext,
  scene: Arc<Scene>,
  params: RenderParams,
) -> Result<()> {
  for iteration in 0..context.scanlines_per_thread {
    let row = context.thread_id + (iteration * context.threads);

    if row >= context.scanline_max {
      break;
    }

    let row_index = row * IMAGE_WIDTH;

    for col in 0..IMAGE_WIDTH {
      context.sender.send(RenderMessage::Pixel(
        row_index + col,
        render::render_pixel(&scene, &params, Vec2::new(col, row)),
      ))?;
    }

    context.sender.send(RenderMessage::ScanlineComplete)?;
  }

  Ok(())
}

fn spawn_render_threads(scene: Arc<Scene>) -> Receiver<RenderMessage> {
  let (sender, receiver) = mpsc::channel();

  let threads = {
    let threads = num_cpus::get();
    let cores = num_cpus::get_physical();

    eprintln!("Running {threads} threads on {cores} cores...");

    threads
  };
  let scanlines_per_thread = (IMAGE_HEIGHT as f64 / threads as f64).ceil() as usize;

  for thread_id in 0..threads {
    let context = RenderThreadContext {
      thread_id,
      threads,
      scanlines_per_thread,
      scanline_max: IMAGE_HEIGHT,
      sender: sender.clone(),
    };
    let scene = Arc::clone(&scene);
    let params = RenderParams {
      last_pixel: Vec2::new(LAST_PIXEL_X, LAST_PIXEL_Y),
      samples_per_pixel: SAMPLES_PER_PIXEL,
      max_bounces: MAX_BOUNCES,
    };

    thread::spawn(move || {
      render_thread(context, scene, params).expect("Render thread did not execute successfully.");
    });
  }

  receiver
}

fn main() -> Result<()> {
  let camera = Camera::new(ASPECT_RATIO);
  let world = Box::new(vec![
    Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
    Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
  ]);

  let mut image = vec![Vec3::default(); IMAGE_PIXELS];

  let receiver = spawn_render_threads(Arc::new(Scene { camera, world }));

  let mut scanlines_remaining = IMAGE_HEIGHT;
  for _ in 0..(IMAGE_PIXELS + IMAGE_HEIGHT) {
    match receiver.recv()? {
      RenderMessage::Pixel(index, color) => {
        image[index] = color;
      }
      RenderMessage::ScanlineComplete => {
        scanlines_remaining -= 1;
        eprint!("\rScanlines remaining: {scanlines_remaining}  ");
      }
    };
  }

  println!("P3");
  println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
  println!("255");
  for pixel_color in image.into_iter().rev() {
    color::print_color(pixel_color);
  }

  eprintln!();
  eprintln!("Done.");

  Ok(())
}
