use std::fs;
use std::fs::File;
use std::io::Write;

struct PpmFile;

impl PpmFile {
    fn write(file: &mut File, width: u64, height: u64) -> Result<(), std::io::Error> {
        let header = ["P3", &format!("{width} {height}"), "255", ""].join("\n");
        file.write_all(header.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::io::Seek;
    use std::io::SeekFrom;
    use std::ops::Range;
    use tempfile::tempfile;

    #[test]
    fn write_ppm_header() {
        let mut output = tempfile().unwrap();

        PpmFile::write(&mut output, 5, 3).expect("Unable to write the file");
        let header = read_file_lines(&mut output, 0..4);
        assert_eq!(header, vec!["P3", "5 3", "255", ""])
    }

    fn read_file_lines(f: &mut File, lines_range: Range<usize>) -> Vec<String> {
        f.seek(SeekFrom::Start(0)).unwrap();

        let mut content = String::new();
        f.read_to_string(&mut content)
            .expect("Unable to read from file");

        let lines: Vec<String> = content.split('\n').map(|x| x.to_string()).collect();
        lines[lines_range].to_vec()
    }
}
