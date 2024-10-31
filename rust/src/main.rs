use core::error;
use crossterm::{
    cursor::{Hide, MoveTo},
    ExecutableCommand,
};
use rodio::{source::Source, Decoder, OutputStream};
use std::{
    fs,
    thread, 
    time,
    io::{self, BufReader, BufWriter, Read, Write},
};

/// The total number of frames for the animation
const NUM_FRAMES: u16 = 6572;

/// Common parts used in the paths of all resources
const PATH_COMMON: (&str, &str) = ("../common/res/", ".jpg.txt");

/// How much time the animation pauses for each frame
const FRAME_TIME: time::Duration = time::Duration::from_millis(33);

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut stdout = io::stdout();
    let mut buf = BufWriter::new(stdout.lock());

    let mut cur_frame: u16 = 0;

    // Hides the cursor from the screen.
    stdout.execute(Hide)?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio_file =
        BufReader::new(fs::File::open(format!("{}badapple.wav", PATH_COMMON.0)).expect("Wav file for the song not found"));
    let audio_src = Decoder::new(audio_file).unwrap();

    stream_handle.play_raw(audio_src.convert_samples()).unwrap();

    for _ in 0..NUM_FRAMES {
        let frame_start = time::Instant::now();

        cur_frame += 1;

        // Move the cursor to the top left of the screen, effectively
        // overwriting the previous frame.
        // This is much cheaper than clearing the screen every frame.
        stdout.execute(MoveTo(0, 0))?;

        let frame: String = match fs::File::open(format!(
            "{}out{}{}",
            PATH_COMMON.0,
            pad_str(&cur_frame.to_string()),
            PATH_COMMON.1
        )) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut content = String::new();

                match reader.read_to_string(&mut content) {
                    Ok(_) => content,
                    Err(err) => {
                        eprintln!("Error reading frame {} from file: {}", cur_frame, err);
                        break;
                    }
                }
            }
            Err(err) => {
                eprint!(
                    "An error occured while trying to open the file for the frame {}: {}",
                    cur_frame, err
                );
                break;
            }
        };

        writeln!(buf, "{}", frame)?;

        // The buffer needs to flush every frame, otherwise
        // the cursor will not effectively change positions.
        buf.flush()?;

        let frame_duration = frame_start.elapsed();

        if frame_duration < FRAME_TIME {
            thread::sleep(FRAME_TIME - frame_duration);
        }

        /*let frame_end = frame_start + FRAME_TIME;

        // A busy while loop is used here instead of thread.sleep(),
        // as thread.sleep() can make the animation speed imprecise.
        while time::Instant::now() < frame_end {}*/
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