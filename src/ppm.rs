use std::fs::File;
use std::io::Write;

use crate::drawing::Canvas;
use crate::drawing::Color;

type WriteResult = Result<(), std::io::Error>;

pub fn write_ppm(filename: &str, canvas: &Canvas) -> WriteResult {
    File::create(filename)
        .and_then(|file| PpmWriter::write_file(file, canvas))
        .and(Ok(()))
}

struct PpmWriter<'a> {
    file: &'a File,
}

impl PpmWriter<'_> {
    const MAX_LINE_LENGTH: usize = 70;

    fn write_file(file: File, canvas: &Canvas) -> Result<File, std::io::Error> {
        let mut ppm = PpmWriter { file: &file };
        ppm.write_header(canvas.width(), canvas.height())?;
        ppm.write_pixels(canvas.pixels())?;
        Ok(file)
    }

    fn write_header(&mut self, width: usize, height: usize) -> WriteResult {
        let header = ["P3", &format!("{width} {height}"), "255", ""].join("\n");
        self.file.write_all(header.as_bytes())
    }

    fn write_pixels(&mut self, pixels: &[Color]) -> WriteResult {
        let bytes = pixels
            .iter()
            .flat_map(|pix| pix.to_a())
            .map(|channel| to_int(channel));

        let mut line = String::new();
        for byte in bytes {
            let byte_str = format!("{} ", byte);
            if line.len() + byte_str.len() >= Self::MAX_LINE_LENGTH {
                self.write_line(&line)?;
                line = String::new()
            }
            line += &byte_str;
        }
        self.write_line(&line)
    }

    fn write_line(&mut self, line: &str) -> WriteResult {
        self.file.write_all(line.trim().as_bytes())?;
        writeln!(self.file, "")
    }
}

fn to_int(color_channel: f64) -> u8 {
    (color_channel * 255.0 + 0.5) as u8
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn write_ppm_header() {
        let canvas = Canvas::new(5, 3);

        let output = PpmWriter::write_file(tempfile(), &canvas).expect("Unable to write the file");
        let header = read_file_lines(output);
        assert_eq!(header[0..3], vec!["P3", "5 3", "255"])
    }

    #[test]
    fn pixel_data() {
        let canvas = Canvas::new(10, 2);
        let output = PpmWriter::write_file(tempfile(), &canvas).unwrap();

        let content = read_file_lines(output);
        let pixel_data = &content[3..content.len() - 1];
        let bytes_written: usize = pixel_data.iter().flat_map(|line| line.split(' ')).count();
        assert_eq!(10 * 2 * 3, bytes_written);
    }

    #[test]
    fn write_pixel_data_splitting_long_lines() {
        let mut canvas = Canvas::new(10, 2);
        canvas.fill(&Color::new(1., 0.8, 0.6));

        let output = PpmWriter::write_file(tempfile(), &canvas).unwrap();
        let content = read_file_lines(output);
        assert_eq!(
            content[3..7],
            vec![
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255",
                "204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153"
            ]
        );
    }

    #[test]
    fn finish_file_with_newline() {
        let canvas = Canvas::new(1, 1);

        let output = PpmWriter::write_file(tempfile(), &canvas).unwrap();
        let content = read_file_lines(output);
        assert_eq!(content.last(), Some(&"".to_string()));
    }

    fn read_file_lines(mut f: File) -> Vec<String> {
        f.seek(SeekFrom::Start(0)).unwrap();

        let mut content = String::new();
        f.read_to_string(&mut content)
            .expect("Unable to read from file");

        content.split('\n').map(|x| x.to_string()).collect()
    }

    fn tempfile() -> File {
        tempfile::tempfile().unwrap()
    }
}
