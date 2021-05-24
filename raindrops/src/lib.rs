pub fn raindrops(n: u32) -> String {
    let mut raindrop_sound = "".to_owned();
    if n%3 == 0 {
        raindrop_sound.push_str("Pling")
    }
    if n%5 == 0 {
        raindrop_sound.push_str("Plang")
    }
    if n%7 == 0 {
        raindrop_sound.push_str("Plong")
    }
    if raindrop_sound.is_empty() {
        raindrop_sound = n.to_string();
    }
    return raindrop_sound
}

