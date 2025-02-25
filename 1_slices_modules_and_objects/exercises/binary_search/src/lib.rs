/// Finds the index of `value` in the sorted slice `v`
/// Returns the index if found, None otherwise
pub fn binary_search(v: &[i32], value: i32) -> Option<usize> {
    let mut index = None;

    if v.len() > 0 {

        let mid: usize = v.len() / 2;


        if v[mid] == value {
            index = Some(mid);
        }
        else if v[mid] < value { 
            index = binary_search(&v[mid+1..], value).map(|x| mid + x + 1);
        }
        else {
            index = binary_search(&v[..mid], value);
        }
    }

     
    index

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_empty() {
        let v = vec![];

        let index = binary_search(&v, 42);

        assert_eq!(index, None);
    }

    #[test]
    fn find_one() {
        let v = vec![1];

        let index = binary_search(&v, 1);

        assert_eq!(index, Some(0));
    }

    #[test]
    fn find_middle() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = binary_search(&v, 15);

        assert_eq!(index, Some(3));
    }

    #[test]
    fn find_right() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = binary_search(&v, 40);

        assert_eq!(index, Some(5));
    }

    #[test]
    fn find_left() {
        let v = vec![1, 7, 10, 15, 23, 40, 41, 50];

        let index = binary_search(&v, 10);

        assert_eq!(index, Some(2));
    }

    #[test]
    fn find_edge_right() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = binary_search(&v, 41);

        assert_eq!(index, Some(6));
    }

    #[test]
    fn find_edge_left() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = binary_search(&v, 1);

        assert_eq!(index, Some(0));
    }

    #[test]
    fn find_not_there() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = binary_search(&v, 14);

        assert_eq!(index, None);
    }

    #[test]
    fn find_edge_not_there_left() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = binary_search(&v, 0);

        assert_eq!(index, None);
    }

    #[test]
    fn find_edge_not_there_right() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = binary_search(&v, 20000);

        assert_eq!(index, None);
    }
}
