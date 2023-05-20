fn main() {
    println!("\n*** Chapter 04 ***\n");

    let mut v = vec![4, 2, 7, 1, 3];
    println!("Vector: {:?}", v);
    selection_sort(&mut v);
    println!("Sorted: {:?}\n", v);
}

fn selection_sort(data: &mut [i32]) {
    let data_len = data.len();
    for i in 0..data_len {
        let mut lowest = i;
        for j in (i+1)..data_len {
            if data[j] < data[lowest] {
                lowest = j;
            }
        }
        if i != lowest {
            data.swap(i, lowest);
        }
        println!("Sorted index {i}: {:?}", data);
    }
}

//// Rust Extras
#[allow(dead_code)]
fn selection_sort_extra(data: &mut [i32]) {
    let data_len = data.len();
    for i in 0..data_len {
        let min = (i..data_len)
            .min_by_key(|x| data[*x])
            .unwrap();
        data.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut v: Vec<i32> = vec![];
        selection_sort(&mut v);
        assert_eq!(Vec::<i32>::new(), v);

        let mut a = [65, -55, 45, -35, 0, 15, 10];
        selection_sort(&mut a);
        assert_eq!([-55, -35, 0, 10, 15, 45, 65], a);
    }

    #[test]
    fn test_selection_sort_extra() {
        let mut v: Vec<i32> = vec![];
        selection_sort_extra(&mut v);
        assert_eq!(Vec::<i32>::new(), v);

        let mut a = [65, -55, 45, -35, 0, 15, 10];
        selection_sort_extra(&mut a);
        assert_eq!([-55, -35, 0, 10, 15, 45, 65], a);
    }
}