use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn parse_line(line: String) -> Vec<f64> {
    let line = line.split_whitespace();
    line.map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>()
}

pub struct CubeData {
    n_atoms: u32,
    data: Vec<f64>,
    origin: Vec<f64>
}

impl CubeData {
    fn new() -> Self {
        CubeData{n_atoms: 0,
                 data: Vec::new(),
                 origin: Vec::new()
                }
    }
}

pub fn read_cube(file_name: &str) -> std::io::Result<CubeData> {
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);
    let mut cube_data = CubeData::new();
    println!("{}", cube_data.n_atoms);

    let mut lines = reader.lines();
    lines.next();
    lines.next();
    let line3 = parse_line(lines.next().unwrap()?);
    cube_data.n_atoms = line3[0] as u32;
    let origin = Vec::from(&line3[1..]);

    let line4 = parse_line(lines.next().unwrap()?);
    let line5 = parse_line(lines.next().unwrap()?);
    let line6 = parse_line(lines.next().unwrap()?);

    let shape_x = line3[0];
    let vec_x = Vec::from(&line3[1..]);
    let shape_y = line3[1];
    let shape_z = line3[2];

    // for line in reader.lines() {
    //     println!("{}", line.unwrap())
    // }
    Ok(cube_data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_file() {
        super::read_cube("tdc.cube").unwrap();
        assert_eq!(2 + 2, 4);
        assert_ne!(true, true)
    }
}
