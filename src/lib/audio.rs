use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use sha2::{Sha256, Digest};

pub fn blow_entropy() -> Vec<u8> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to get default input device");
    let config: cpal::StreamConfig = device.default_input_config().expect("Failed to get default StreamConfig").into();

    let recorded_samples = Arc::new(Mutex::new(Vec::<f32>::new()));
    let recorded_samples_clone = Arc::clone(&recorded_samples);

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut samples = recorded_samples_clone.lock().unwrap();
            samples.extend_from_slice(data);
        },
        |err| {
            eprintln!("Error occurred on stream: {}", err);
        },
    ).expect("Failed to build input stream");

    stream.play().expect("Failed to start the input stream");

    // Record for 10 seconds
    std::thread::sleep(std::time::Duration::from_secs(10));

    // Stop the stream and return the recorded samples
    drop(stream); // Dropping the stream stops it

    // collect the samples as floating points
    let samples: Vec<f32> = Arc::try_unwrap(recorded_samples)
        .expect("Failed to unwrap Arc")
        .into_inner()
        .expect("Failed to get Mutex inner");

    // extract byte array from floating points
    let mut bytes = Vec::with_capacity(samples.len() * std::mem::size_of::<f32>());
    for &num in &samples {
        bytes.extend_from_slice(&num.to_ne_bytes());
    }

    // use the byte array as preimage for a sha256 hash
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash_result = hasher.finalize();

    let entropy: Vec<u8> = hash_result.to_vec();

    entropy

}