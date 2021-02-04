use std::io::Read;

#[allow(dead_code)]
pub fn as_u8_slice<T>(p: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(p.as_ptr() as *const u8, std::mem::size_of::<T>() * p.len()) }
}

#[allow(dead_code)]
pub fn as_u8_mut_slice<T>(p: &mut [T]) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(p.as_mut_ptr() as *mut u8, std::mem::size_of::<T>() * p.len()) }
}

pub fn file_to_vec<P, T>(input_file: P) -> std::io::Result<Vec<T>>
where
    P: AsRef<std::path::Path>,
    T: std::marker::Copy,
{
    let mut f = std::fs::File::open(&input_file)?;
    let file_size = std::fs::metadata(&input_file)?.len() as usize;
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
