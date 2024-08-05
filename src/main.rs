use std::fs;
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    // Start the Python script
    let mut child = Command::new("python3")
        .arg("face_detection.py")
        .spawn()
        .expect("Failed to start Python script");

    let mut last_detection = Instant::now();
    let mut face_detected = false;
    let detection_duration = Duration::from_secs(3);
    let mut face_detected_for = Duration::from_secs(0);

    loop {
        if fs::metadata("face_coords.json").is_ok() {
            let file_content = fs::read_to_string("face_coords.json").unwrap_or_default();

            // Check if the file contains an empty list
            if file_content.trim() == "[]" {
                // File contains "[]", meaning no face is detected
                face_detected = false;
                face_detected_for = Duration::from_secs(0); // Reset continuous detection duration
            } else {
                // File has content other than "[]", meaning face is detected
                if !face_detected {
                    // Face was not previously detected; start the timer
                    face_detected = true;
                    last_detection = Instant::now();
                } else {
                    // Face was detected; check if it has been continuously detected
                    face_detected_for = Instant::now().duration_since(last_detection);
                    if face_detected_for > detection_duration {
                        // Face has been detected continuously for more than 5 seconds
                        println!("FIRING");
                        face_detected = false; // Reset detection status after firing

                        // Optionally, create an action flag file if needed
                        fs::write("action_flag.txt", "FIRING").expect("Failed to write action flag file");
                    }
                }
            }
        } else {
            // File not found, reset detection status
            face_detected = false;
            face_detected_for = Duration::from_secs(0); // Reset continuous detection duration
        }

        // Sleep for a short period before the next check
        sleep(Duration::from_secs(1));
    }

    // Ensure the Python script is terminated when Rust program exits
    let _ = child.kill();
}
