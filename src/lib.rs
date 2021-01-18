use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::sync::mpsc;


#[pyfunction]
fn read_array(a: Vec<i32>) {
    for i in 0..a.len() {
        println!("{}", i);
    }
}

#[pyfunction]
fn rayon_square_root(vector: Vec<i32>) -> PyResult<Vec<(i32, f32)>> {
    let (sender, reciever) = mpsc::channel();
    let mut square_roots = Vec::<(i32, f32)>::new();

    vector.par_iter().for_each_with(sender, |s, x| {
        s.send((x.clone(), (x.clone() as f32).sqrt())).unwrap()
    });

    for _ in vector {
        let y = reciever.try_recv().unwrap();
        square_roots.push(y);
    }

    //square_roots.sort_by_key(|tuple| tuple.0);
    Ok(square_roots)
}
#[pyfunction]
fn sort_by_key(vector: Vec<(i32, f32)>, key: i32) -> PyResult<Vec<(i32, f32)>> {
    match key {
        //sort by original number
        0 => {
            let mut new_vector = vector.to_owned();
            new_vector.sort_by_key(|tuple| tuple.0);
            Ok(new_vector)
        }
        //sort by resultant
        1 => {
            let mut new_vector = vector.to_owned();
            new_vector.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
            Ok(new_vector)
        }
        // otherwise do nothing
        2..=i32::MAX => Ok(vector.to_owned()),
        i32::MIN..=-1 => Ok(vector.to_owned()),
    }
}

#[pyfunction]
fn standard_sort(mut vector: Vec<i32>) -> PyResult<Vec<i32>>{
	vector.sort();
	return Ok(vector)
}
/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_array, m)?)?;
    m.add_function(wrap_pyfunction!(rayon_square_root, m)?)?;
    m.add_function(wrap_pyfunction!(sort_by_key, m)?)?;
    m.add_function(wrap_pyfunction!(standard_sort, m)?)?;
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_square_root(vector: Vec<i32>) -> Vec<(i32, f32)> {
        let (sender, reciever) = mpsc::channel();
        let mut square_roots = Vec::<(i32, f32)>::new();
        vector.par_iter().for_each_with(sender, |s, x| {
            s.send((x.clone(), (x.clone() as f32).sqrt())).unwrap()
        });
        for _ in vector {
            let y = reciever.try_recv().unwrap();
            square_roots.push(y);
        }
        square_roots.sort_by_key(|tuple| tuple.0);
        square_roots
    }

    fn test_sort_by_key(vector: Vec<(i32, f32)>, key: i32) -> Vec<(i32, f32)> {
        match key {
            //sort by original number
            0 => {
                let mut new_vector = vector.to_owned();
                new_vector.sort_by_key(|tuple| tuple.0);
                new_vector
            }
            //sort by resultant
            1 => {
                let mut new_vector = vector.to_owned();
                new_vector.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
                new_vector
            }
            // otherwise do nothing
            2..=i32::MAX => vector.to_owned(),
            i32::MIN..=-1 => vector.to_owned(),
        }
    }

    #[test]
    fn test_square() {
        let values = vec![4, 9];
        let results = test_square_root(values);
        assert_eq!(results[0].0, 4);
        assert_eq!(results[0].1, 2.0);
    }
    #[test]
    fn test_sort() {
        let values = vec![4, 9, 16];

        let sort_float = test_square_root(values);
        let sort_ints = sort_float.clone();

        let sort_ints = test_sort_by_key(sort_ints, 0);
        let sort_float = test_sort_by_key(sort_float, 1);

        assert_eq!(sort_ints[0].0, 4);
        assert_eq!(sort_float[0].1, 2.0);

        assert_eq!(sort_ints[2].0, 16);
        assert_eq!(sort_float[2].1, 4.0);
    }
}
