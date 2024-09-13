extern crate kani;
use kani::mem::can_dereference;

/// Function that adds an offset to a pointer.
/// 
/// The `count` parameter represents the number of elements of type `T` to offset by.
/// The `object_size` parameter represents the total size of the allocated object (in bytes) 
/// to ensure we stay within bounds.
/// 
/// # Safety
/// This function assumes that:
/// - `ptr` must be valid and dereferenceable.
/// - The computed offset should not exceed the allocated object size.
/// - The entire range between `ptr` and `ptr.add(count)` must remain within bounds.
fn kani_pointer_add<T>(ptr: *const T, count: usize, object_size: usize) {
    unsafe {
        // Precondition: The pointer must be dereferenceable
        kani::assume(can_dereference(ptr));

        // Precondition: Ensure the pointer's offset does not exceed the object size
        let size_of_t = std::mem::size_of::<T>();
        kani::assume(count * size_of_t <= object_size);

        // Perform the pointer arithmetic
        let offset_ptr = ptr.add(count);

        // Post-condition: Ensure the result pointer is still within bounds of the allocated object
        let end_of_object = (ptr as usize + object_size) as *const T;

        // Assert that the resulting pointer is within bounds, with a detailed message if it fails
        kani::assert(
            offset_ptr <= end_of_object,
            "Pointer offset is out of bounds."
        );
    }
}

/// Verifies the safety of pointer offset operation.
///
/// The `count` parameter represents the number of elements of type `T` to offset by.
/// The `object_size` parameter represents the total size of the allocated object (in bytes)
/// to ensure we stay within bounds.
///
/// # Safety
/// This function assumes that:
/// - `ptr` must be valid and dereferenceable.
/// - The computed offset should not exceed the allocated object size.
/// - The entire range between `ptr` and `ptr.offset(count)` must remain within bounds.
fn kani_pointer_offset<T>(ptr: *const T, count: isize, object_size: usize) {
    unsafe {
        // Precondition: The pointer must be dereferenceable
        kani::assume(can_dereference(ptr));

        // Precondition: Ensure the pointer's offset does not exceed the object size
        let size_of_t = std::mem::size_of::<T>();
        let max_offset = (object_size / size_of_t) as isize;

        // The offset should be within valid bounds to prevent overflow
        kani::assume(count >= 0 && count <= max_offset);

        // Perform the pointer offset operation
        let offset_ptr = ptr.offset(count);

        // Post-condition: Ensure the result pointer is still within bounds of the allocated object
        let end_of_object = (ptr as usize + object_size) as *const T;

        // Assert that the resulting pointer is within bounds, with a detailed message if it fails
        kani::assert(
            offset_ptr <= end_of_object,
            "Pointer offset is out of bounds."
        );
    }
}

#[kani::proof]
fn verify_pointer_add() {
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();
    let object_size = s.len(); // In bytes, the size of the allocated object

    // Test adding offsets within bounds
    kani_pointer_add(ptr, 1, object_size); // Adding an offset of 1
    kani_pointer_add(ptr, 2, object_size); // Adding an offset of 2
}

#[kani::proof]
fn verify_pointer_offset() {
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();
    let object_size = s.len(); // In bytes, the size of the allocated object

    // Test offsetting within bounds
    kani_pointer_offset(ptr, 1, object_size); // Offset by 1
    kani_pointer_offset(ptr, 2, object_size); // Offset by 2
}