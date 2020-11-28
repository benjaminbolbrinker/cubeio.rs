use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn parse_line(line: String) -> Vec<f64> {
    let line = line.split_whitespace();
    line.map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>()
}

#[allow(dead_code)]
pub struct Atom {
    an: u32,
    charge: f32,
    pos: Vec<f64> 
}

pub struct CubeData {
    n_atoms: u32,
    origin: Vec<f64>,
    shape: Vec<u32>,
    data: Vec<f64>,
    cell: Vec<Vec<f64>>,
    atoms: Vec<Atom>
}

impl CubeData {
    fn new() -> Self {
        CubeData{
            n_atoms: 0,
            origin: vec![0f64; 3],
            shape: vec![0u32; 3],
            cell: vec![vec![0f64; 3]; 3],
            atoms: Vec::new(),
            data: Vec::new()
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
    cube_data.origin = Vec::from(&line3[1..]);

    let line4 = parse_line(lines.next().unwrap()?);
    let line5 = parse_line(lines.next().unwrap()?);
    let line6 = parse_line(lines.next().unwrap()?);

    cube_data.shape[0] = line4[0] as u32;
    cube_data.cell[0] = Vec::from(&line4[1..]);
    cube_data.shape[1] = line5[0] as u32;
    cube_data.cell[1] = Vec::from(&line5[1..]);
    cube_data.shape[2] = line6[0] as u32;
    cube_data.cell[2] = Vec::from(&line6[1..]);

    let mut idx = 0;
    while idx < cube_data.n_atoms {
        let line_atom = parse_line(lines.next().unwrap()?);
        cube_data.atoms.push(Atom {
            an: line_atom[0] as u32,
            charge: line_atom[1] as f32,
            pos: Vec::from(&line_atom[2..])
        });
        idx += 1;
    }

    for line in lines {
        cube_data.data.append(&mut parse_line(line?));
    }

    Ok(cube_data)
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    #[test]
    fn read_file() {
        let cube_data = super::read_cube("tdc.cube").unwrap();
        assert_eq!(cube_data.n_atoms, 30);
        assert_eq!(cube_data.origin, vec![0f64; 3]);
        assert_eq!(cube_data.shape, vec![79u32, 52u32, 85u32]);
        assert_eq!(cube_data.atoms[0].an, 3u32);
        assert_eq!(cube_data.atoms[cube_data.atoms.len() - 1].an, 16u32);
        approx_eq!(f32, cube_data.atoms[0].charge, 0f32);
        assert_eq!(cube_data.data.len(), (cube_data.shape[0]*cube_data.shape[1]*cube_data.shape[2]) as usize);
    }
}