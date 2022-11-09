use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    // Parse args
    let mut args = std::env::args().skip(1);
    let rule: u8 = args
        .next()
        .unwrap_or("30".into())
        .parse()
        .expect("Rule must be integer");
    let width: usize = args
        .next()
        .unwrap_or("1000".into())
        .parse()
        .expect("Width must be integer");
    let height: usize = args
        .next()
        .unwrap_or("1000".into())
        .parse()
        .expect("Height must be integer");
    let output_path: String = args.next().unwrap_or("out.pbm".into());

    // Setup grid
    let mut grid = vec![false; width];
    grid[width / 2] = true;

    // Simulate
    let image = sim(grid, height, rule);

    // Write image
    //let image = pack_bools(&image);
    write_pbm(output_path, width, height, &image)
}

fn sim(mut grid: Vec<bool>, iters: usize, rule: u8) -> Vec<bool> {
    let mut tmp = grid.clone();

    let mut image = vec![];
    for _ in 0..iters {
        image.extend_from_slice(&grid);

        for (grid, tmp) in grid.windows(3).zip(tmp.iter_mut().skip(1)) {
            let n = bits_to_byte(grid);
            *tmp = rule_lookup(rule, n);
        }
        std::mem::swap(&mut grid, &mut tmp);
    }
    image
}

fn rule_lookup(rule: u8, bin: u8) -> bool {
    (rule >> bin) & 1 == 1
}

fn bits_to_byte(data: &[bool]) -> u8 {
    assert!(data.len() <= u8::BITS as usize);
    data.iter().fold(0, |mut acc, &x| {
        acc <<= 1;
        acc | u8::from(x)
    })
}

/*
fn pack_bools(data: &[bool]) -> Vec<u8> {
    data.chunks(8).map(bits_to_byte).collect()
}
*/

fn write_pbm(path: impl AsRef<Path>, width: usize, height: usize, data: &[bool]) -> io::Result<()> {
    let mut f = BufWriter::new(File::create(path)?);
    /*
    let num_bits = data.len() * 8;
    dbg!(num_bits, width, height);
    assert!(
        num_bits <= width * height,
        "Image area is larger than provided data",
    );
    */
    writeln!(f, "P1")?;
    writeln!(f, "{} {}", width, height)?;
    for row in data.chunks_exact(width) {
        for &k in row {
            if k {
                write!(f, "0")?;
            } else {
                write!(f, "1")?;
            }
        }
        writeln!(f)?;
    }
    //f.write_all(data)?;
    Ok(())
}
