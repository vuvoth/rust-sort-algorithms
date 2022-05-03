mod bublesort {
    pub fn bublesort(slice: &mut [i64]) {
        let mut swaped = true;        
        while swaped {
            swaped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swaped = true;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::bublesort::bublesort;

    #[test]
    fn it_works() {

        let mut arr = vec![1, 7, 3, -9, 5];
        
        bublesort(&mut arr);

        assert_eq!(arr, vec![-9, 1, 3, 5, 7]);
    }
}