use std::io::Read;

pub fn file_to_vec<T>(input_file: &str) -> std::io::Result<Vec<T>> {
    let mut f = std::fs::File::open(input_file)?;
    let file_size = std::fs::metadata(input_file)?.len() as usize;
    let elements_len = file_size / std::mem::size_of::<T>();
    // Don't use Vec<u8>. We need more strictly aligned data.
    let mut v = Vec::<T>::with_capacity(elements_len);
    unsafe {
        v.set_len(elements_len);
    }
    let u8_slice = unsafe {
        std::slice::from_raw_parts_mut(
            v.as_mut_slice().as_mut_ptr() as *mut u8,
            elements_len * std::mem::size_of::<T>(),
        )
    };
    f.read_exact(u8_slice)?;
    Ok(v)
}
