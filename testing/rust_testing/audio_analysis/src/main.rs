use portaudio;
use std::sync::mpsc::*;

fn main() {

    // Construct a portaudio instance that will connect to a native audio API
    let pa = portaudio::PortAudio::new().expect("Unable to init PortAudio"); 
    print!("PortAudio initialized\n");
    // Collect information about the default microphone
    let mic_index = pa.default_input_device().expect("Unable to get default device");
    let mic = pa.device_info(mic_index).expect("unable to get mic info");

    print!("mic started\n");

    // Set parameters for the stream settings.
    // We pass which mic should be used, how many channels are used,
    // whether all the values of all the channels should be passed in a 
    // single audiobuffer and the latency that should be considered 
    let input_params = portaudio::StreamParameters::<f32>::new( mic_index, 1, true, mic.default_low_input_latency);

    // Settings for an inputstream.
    // Here we pass the stream parameters we set before,
    // the sample rate of the mic and the amount values we want to receive
    let input_settings = portaudio::InputStreamSettings::new(input_params, mic.default_sample_rate, 256);

    print!("inputs set\n");

    // Creating a channel so we can receive audio values asynchronously
    let (sender, receiver) = channel(); 

    // A callback function that should be as short as possible so we send all the info to a different thread
    let callback = move |portaudio::InputStreamCallbackArgs {buffer, .. }| {
        match sender.send(buffer) {
            Ok(_) => portaudio::Continue, 
            Err(_) => portaudio::Complete
        }
    };

    // Creating & starting the input stream with our settings & callback
    let mut stream = pa.open_non_blocking_stream(input_settings, callback).expect("Unable to create stream"); 
    stream.start().expect("Unable to start stream");

    //Printing values every time we receive new ones while the stream is active
    while stream.is_active().unwrap() {
       while let Ok(buffer) = receiver.try_recv() {
            print!("{:?}\r", buffer[0]); 
       }
    }
}
