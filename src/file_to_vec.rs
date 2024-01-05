use anyhow::Result;
use std::io::Read;

#[allow(dead_code)]
pub fn as_u8_slice<T>(p: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(p.as_ptr() as *const u8, std::mem::size_of_val(p)) }
}

#[allow(dead_code)]
pub fn as_u8_mut_slice<T>(p: &mut [T]) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(p.as_mut_ptr() as *mut u8, std::mem::size_of_val(p)) }
}

pub fn file_to_vec<P, T>(input_file: P) -> Result<Vec<T>>
where
    P: AsRef<std::path::Path> + std::fmt::Display,
    T: std::marker::Copy,
{
    let mut f = std::fs::File::open(&input_file)?;
    let file_size = std::fs::metadata(&input_file)?.len() as usize;
    if file_size % std::mem::size_of::<T>() != 0 {
        anyhow::bail!(
            "File size of {} must be a multiple of {}.",
            input_file,
            std::mem::size_of::<T>()
        );
    }
    let elements = file_size / std::mem::size_of::<T>();
    // Don't use Vec<u8>. We need more strictly aligned data.
    let mut v = vec![std::mem::MaybeUninit::<T>::uninit(); elements];
    let u8_slice = unsafe { std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut u8, elements * std::mem::size_of::<T>()) };
    f.read_exact(u8_slice)?;

    // Vec<MaybeUninit<T>> to Vec<T>
    let v = {
        let mut v = std::mem::ManuallyDrop::new(v);
        let p = v.as_mut_ptr() as *mut T;
        let len = v.len();
        let cap = v.capacity();
        unsafe { Vec::from_raw_parts(p, len, cap) }
    };
    Ok(v)
}
