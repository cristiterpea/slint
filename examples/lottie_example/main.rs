// This file demonstrates the Rust backend implementation for the LottiePlayer Slint component.
// It uses the `rlottie-rs` crate to load and render Lottie animations and provides the
// necessary callbacks for the `Lottie` global defined in `lottie_player.slint`.

use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString};
use std::sync::Mutex;
use rlottie_rs::{Animation, Surface};

// Represents the state of the currently loaded Lottie animation.
// This includes the animation object itself, frame counters, and rendering surface.
struct LottieAnimationState {
    animation: Option<Animation>, // Holds the rlottie-rs Animation object when loaded.
    current_frame: usize,         // Current frame number to be rendered.
    total_frames: usize,          // Total number of frames in the animation.
    surface: Option<Surface>,     // rlottie-rs Surface used as a rendering buffer.
}

impl LottieAnimationState {
    fn new() -> Self {
        LottieAnimationState {
            animation: None,
            current_frame: 0,
            total_frames: 0,
            surface: None,
        }
    }
}

// Use once_cell for thread-safe, global static initialization of LOTTIE_STATE.
// LOTTIE_STATE holds the single, globally accessible Lottie animation state.
// A Mutex is used to ensure safe concurrent access if Slint were to call callbacks from different threads,
// though typically Slint callbacks are on the main GUI thread.
use once_cell::sync::Lazy;

static LOTTIE_STATE: Lazy<Mutex<LottieAnimationState>> = Lazy::new(|| Mutex::new(LottieAnimationState::new()));

// Includes the Slint generated Rust code from `main_ui.slint` (and its imports like `lottie_player.slint`)
// as processed by the `build.rs` script.
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?; // Instantiate the main UI component defined in `main_ui.slint`.

    // Get a handle to the Lottie global object instance from the Slint UI.
    // This allows attaching Rust functions to the callbacks defined in the `Lottie` global in Slint.
    let lottie_global = ui.global::<Lottie>();

    // Callback for when the Slint component requests to load a Lottie animation.
    // `path`: The file path of the Lottie animation.
    // Returns an integer handle: 0 for success, negative for errors.
    lottie_global.on_load_lottie(move |path: SharedString| {
        let mut state = LOTTIE_STATE.lock().unwrap();
        // Clear any previously loaded animation state.
        state.animation = None;
        state.surface = None;
        state.current_frame = 0;
        state.total_frames = 0;

        // Attempt to load the animation using rlottie-rs.
        match Animation::from_file(path.as_str()) {
            Ok(anim) => {
                state.total_frames = anim.totalframes();
                let (width, height) = (anim.width(), anim.height());
                // Ensure dimensions are valid before creating a surface.
                if width > 0 && height > 0 {
                    state.surface = Some(Surface::new(width, height)); // Create a rendering surface.
                    state.animation = Some(anim); // Store the loaded animation.
                    println!("Lottie loaded successfully: {} frames, {}x{}", state.total_frames, width, height);
                    0 // Return 0 for success.
                } else {
                    eprintln!("Lottie loaded but has invalid dimensions: {}x{}", width, height);
                    -2 // Return -2 for invalid dimensions error.
                }
            }
            Err(e) => {
                eprintln!("Error loading Lottie animation from path '{}': {:?}", path, e);
                -1 // Return -1 for general loading errors (file not found, parse error, etc.).
            }
        }
    });

    // Callback for when the Slint component requests to render a frame.
    // `_lottie_id`: Ignored for this single-instance example.
    // Returns a Slint Value struct: `{ frame: Image, is_finished: bool }`.
    lottie_global.on_render_frame(move |_lottie_id: i32| {
        let mut state = LOTTIE_STATE.lock().unwrap();
        // Proceed only if an animation is loaded and a surface exists.
        if let (Some(animation), Some(surface)) = (&mut state.animation, &mut state.surface) {
            // If current_frame is beyond total_frames, animation is considered finished.
            if state.current_frame >= state.total_frames {
                let frame_data = slint::Value::new_struct(&[
                    ("frame", slint::Value::from(slint::Image::default())),
                    ("is_finished", slint::Value::from(true)),
                ]);
                return frame_data;
            }

            // Render the current frame onto the surface.
            animation.render(state.current_frame, surface);
            let buffer = surface.data(); // This is &[u32] (ARGB format from rlottie).

            // Convert ARGB pixel data from rlottie to RGBA8 format expected by Slint.
            let mut rgba_buffer_vec: Vec<Rgba8Pixel> = Vec::with_capacity(buffer.len());
            for &argb_pixel in buffer {
                let a = (argb_pixel >> 24) as u8;
                let r = (argb_pixel >> 16) as u8;
                let g = (argb_pixel >> 8) as u8;
                let b = argb_pixel as u8;
                rgba_buffer_vec.push(Rgba8Pixel { r, g, b, a });
            }
            
            // Create a Slint SharedPixelBuffer from the RGBA data.
            let pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                &rgba_buffer_vec,
                animation.width() as u32,
                animation.height() as u32,
            );

            let image = Image::from_rgba8(pixel_buffer); // Create a Slint Image.
            
            let is_finished = state.current_frame + 1 >= state.total_frames;
            state.current_frame += 1; // Advance to the next frame.

            // Construct the return value expected by Slint: a struct with `frame` and `is_finished`.
            let frame_data = slint::Value::new_struct(&[
                ("frame", slint::Value::from(image)),
                ("is_finished", slint::Value::from(is_finished)),
            ]);
            
            return frame_data;

        } else {
            // If no animation is loaded, return a default empty frame marked as finished.
            let default_frame_data = slint::Value::new_struct(&[
                ("frame", slint::Value::from(slint::Image::default())),
                ("is_finished", slint::Value::from(true)),
            ]);
            return default_frame_data;
        }
    });

    // Callback for when the Slint component requests to delete/unload the Lottie animation.
    // `_lottie_id`: Ignored for this single-instance example.
    lottie_global.on_delete_lottie(move |_lottie_id: i32| {
        let mut state = LOTTIE_STATE.lock().unwrap();
        // Reset all fields in the animation state.
        state.animation = None;
        state.surface = None;
        state.current_frame = 0;
        state.total_frames = 0;
        println!("Lottie animation deleted.");
    });

    // Callback for when the Slint component requests to reset the animation to its first frame.
    // `_lottie_id`: Ignored for this single-instance example.
    lottie_global.on_reset_animation(move |_lottie_id: i32| {
        let mut state = LOTTIE_STATE.lock().unwrap();
        state.current_frame = 0; // Set current frame back to 0.
        println!("Lottie animation reset to frame 0.");
    });

    // Run the Slint event loop.
    ui.run()
}
