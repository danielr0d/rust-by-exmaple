use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of slice: {}", slice[0]);\
    println!("The slice has {} elements", slice.len());
}
