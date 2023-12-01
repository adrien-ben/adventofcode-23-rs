pub mod util {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    pub fn lines<P>(path: P) -> impl Iterator<Item = String>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(|l| l.unwrap())
    }
}
