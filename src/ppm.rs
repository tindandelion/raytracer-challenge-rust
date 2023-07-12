use std::fs::File;
use std::io::Write;

use crate::canvas::Canvas;
use crate::color::Color;

pub struct PpmFile;

type WriteResult = Result<(), std::io::Error>;

impl PpmFile {
    const MAX_LINE_LENGTH: usize = 70;

    pub fn write(filename: &str, canvas: &Canvas) -> WriteResult {
        let mut file = File::create(filename)?;
        PpmFile::write_file(&mut file, canvas)
    }

    fn write_file(file: &mut File, canvas: &Canvas) -> WriteResult {
        PpmFile::write_header(file, canvas.width(), canvas.height())?;
        PpmFile::write_pixels(file, canvas.pixels())?;
        PpmFile::write_newline(file)
    }

    fn write_header(file: &mut File, width: usize, height: usize) -> WriteResult {
        let header = ["P3", &format!("{width} {height}"), "255", ""].join("\n");
        file.write_all(header.as_bytes())
    }

    fn write_newline(file: &mut File) -> WriteResult {
        writeln!(file, "")
    }

    fn write_pixels(file: &mut File, pixels: &[Color]) -> WriteResult {
        let mut line = String::new();
        for pixel in pixels {
            let pixel_str = format!(
                "{} {} {} ",
                to_int(pixel.0),
                to_int(pixel.1),
                to_int(pixel.2)
            );
            if line.len() + pixel_str.len() >= PpmFile::MAX_LINE_LENGTH {
                file.write_all(line.trim().as_bytes())?;
                PpmFile::write_newline(file)?;
                line = String::new()
            }
            line += &pixel_str;
        }
        file.write_all(line.trim().as_bytes())
    }
}

fn to_int(color_channel: f64) -> u8 {
    (color_channel * 255.0 + 0.5) as u8
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;

    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::io::Seek;
    use std::io::SeekFrom;
    use tempfile::tempfile;

    #[test]
    fn write_ppm_header() {
        let mut output = tempfile().unwrap();
        let canvas = Canvas::new(5, 3);

        PpmFile::write_file(&mut output, &canvas).expect("Unable to write the file");
        let header = read_file_lines(&mut output);
        assert_eq!(header[0..3], vec!["P3", "5 3", "255"])
    }

    #[test]
    fn pixel_data() {
        let mut output = tempfile().unwrap();
        let mut canvas = Canvas::new(10, 2);

        canvas.fill(&Color(1., 0.8, 0.6));

        PpmFile::write_file(&mut output, &canvas).unwrap();
        let content = read_file_lines(&mut output);
        let pixel_data = &content[3..content.len() - 1];
        let counts: usize = pixel_data.iter().map(|line| line.split(' ').count()).sum();
        assert_eq!(10 * 2 * 3, counts);
    }

    #[test]
    fn write_pixel_data_splitting_long_lines() {
        let mut output = tempfile().unwrap();
        let mut canvas = Canvas::new(10, 2);

        canvas.fill(&Color(1., 0.8, 0.6));

        PpmFile::write_file(&mut output, &canvas).unwrap();
        let content = read_file_lines(&mut output);
        assert_eq!(
            content[3..7],
            vec![
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"
            ]
        );
    }

    #[test]
    fn finish_file_with_newline() {
        let mut output = tempfile().unwrap();
        let canvas = Canvas::new(1, 1);

        PpmFile::write_file(&mut output, &canvas).unwrap();
        let content = read_file_lines(&mut output);
        assert_eq!(content.last(), Some(&"".to_string()));
    }

    fn read_file_lines(f: &mut File) -> Vec<String> {
        f.seek(SeekFrom::Start(0)).unwrap();

        let mut content = String::new();
        f.read_to_string(&mut content)
            .expect("Unable to read from file");

        content.split('\n').map(|x| x.to_string()).collect()
    }
}
