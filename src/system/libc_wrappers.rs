use libc;

pub fn pipe() -> (i32, i32) {
    let mut descriptors = [0; 2];

    assert_eq!(unsafe { libc::pipe(descriptors.as_mut_ptr()) }, 0);

    (descriptors[0], descriptors[1])
}

pub fn dup(source: i32) -> i32 {
    unsafe { libc::dup(source) }
}

pub fn dup2(source: i32, destination: i32) {
    assert!(unsafe { libc::dup2(source, destination) } > -1);
}

pub fn close(descriptor: i32) {
    assert_eq!(unsafe { libc::close(descriptor) }, 0);
}
