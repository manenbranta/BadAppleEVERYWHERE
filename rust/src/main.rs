use std::{
    {thread,time,fs},
    io::{self, BufWriter, Write}
};
use crossterm::{cursor::{Hide, MoveTo}, ExecutableCommand};
use core::error;

const NUM_FRAMES:u16 = 6572;
const PATH_COMMON:(&str, &str) = ("../common/res/out",".jpg.txt");
const FRAME_TIME:time::Duration = time::Duration::from_millis(30);

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut stdout = io::stdout();
    let mut buf = BufWriter::new(stdout.lock());

    let mut cur_frame:u16 = 0;

    stdout.execute(Hide)?;

    for _ in 0..NUM_FRAMES {
        cur_frame += 1;

        stdout.execute(MoveTo(0,0))?;

        let frame:String = 
        fs::read_to_string(format!("{}{}{}", PATH_COMMON.0, pad_str(&cur_frame.to_string()), PATH_COMMON.1))
            .expect("Could not read file!");

        writeln!(buf, "{}", frame)?;

        buf.flush()?;

        thread::sleep(FRAME_TIME);
    }
    Ok(())
}

fn pad_str(string: &str) -> String {
    let string = format!("{}{}", "000", string);
    string.chars().skip(string.len() - 4).collect()
}