use std::fs;

fn main() {
    let sensors: Vec<_> = fs::read_dir("/sys/bus/w1/devices").unwrap() // Get list of all devices on onewire bus
        .filter( |f| f.as_ref().unwrap().file_name().into_string().unwrap().starts_with("28-") ) // Filter out any non-temperature sensors
        .inspect( |f| println!("Device found: {}", f.as_ref().unwrap().file_name().into_string().unwrap()))
        .collect();
    loop {
        let temps: Vec<String> = sensors.iter()
            .map(|f| fs::read_to_string(
                    f.as_ref().unwrap().path().to_str().unwrap().to_owned() // Get path of device
                    + "/w1_slave").unwrap() // Add the name of the file we want to read from
                )
            .collect();
        let temps: Vec<f32> = temps.iter()
            .flat_map( |f| f.split_whitespace() ) // Get iterator over "words"
            .map( |w| w.to_owned() ) // &str -> String
            .filter( |w| w.contains("t=")) // Only keep the temperatures
            .map( |w| w.trim_left_matches("t=").to_owned() ) // Make words with temperatures to be numbers only
            .map( |s| s.parse().unwrap() ) // Parse temperatures to integers
            .map( |t: i32| t as f32 / 1000.0 ) // Convert to actual temperatures (the temperature is given as 1000*temp in C
            .collect();
        println!("Temps: {:?}", temps);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
