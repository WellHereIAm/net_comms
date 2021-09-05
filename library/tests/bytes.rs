use library::bytes;

# [test]
fn bytes_slice() {

    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bytes = bytes::Bytes::from_vec(vec.clone());
    assert_eq!(vec[0..4], bytes[0..4]);
}