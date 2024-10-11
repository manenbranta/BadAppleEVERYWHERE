use std::{
    {thread,time,fs},
    io::{self, BufWriter, Write}
};
use crossterm::{cursor::{Hide, MoveTo}, ExecutableCommand};
use core::error;


/// The total number of frames for the animation
const NUM_FRAMES:u16 = 6572;

/// Common parts used in the paths of all frames
const PATH_COMMON:(&str, &str) = ("../common/res/out",".jpg.txt");

/// How much time the animation pauses for each frame
const FRAME_TIME:time::Duration = time::Duration::from_millis(30);

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut stdout = io::stdout();
    let mut buf = BufWriter::new(stdout.lock());

    let mut cur_frame:u16 = 0;

    // Hides the cursor from the screen.
    stdout.execute(Hide)?;

    for _ in 0..NUM_FRAMES {
        cur_frame += 1;

        // Move the cursor to the top left of the screen, effectively
        // overwriting the previous frame.
        // This is much cheaper than clearing the screen every frame.
        stdout.execute(MoveTo(0,0))?;

        let frame:String = 
        fs::read_to_string(format!("{}{}{}", PATH_COMMON.0, pad_str(&cur_frame.to_string()), PATH_COMMON.1))
        .expect("Could not read file!");

        writeln!(buf, "{}", frame)?;

        // The buffer needs to flush every frame, otherwise 
        // the cursor will not effectively change positions.
        buf.flush()?;

        thread::sleep(FRAME_TIME);
    }
    Ok(())
}

/// This function returns a string padded with enough 0's 
/// to make it four digits.
/// 
/// ### Examples
/// ```
/// let num = pad_str("17");
/// 
/// assert_eq!(num, "0017");
/// ```
fn pad_str(string: &str) -> String {
    let string = format!("{}{}", "000", string);
    string.chars().skip(string.len() - 4).collect()
}