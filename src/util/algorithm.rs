pub unsafe fn copy<T: Copy>(begin: *mut T, end: *mut T, copy_to: *mut T) {
    let mut ptr = begin;
    let mut copy_ptr = copy_to;

    //If collision
    if super::math::is_in_range(begin as usize, end as usize, copy_to as usize) {
        //We need the buffer to hold only the caracters necessary
        //How did I get there? try it out in a notepad, I cant explain
        //the same begin range will serve us as buffer
        let buffer_size = super::math::min(
            copy_to as isize - begin as isize,
            end as isize - copy_to as isize
        );
        //We can loop in the buffer, advancing the index and then modulus that to buffer_size
        let mut buffer_index = 0;
        let copy_start = copy_to as isize;

        while ptr != end {
            let copy_copy = *copy_ptr;

            if (ptr as isize) < copy_start {
                *copy_ptr = *ptr;
            } else {
                *copy_ptr = *begin.offset(
                    //The difference between the start of copy, modulated to buffer capacity
                    (ptr as isize - copy_start) % buffer_size
                );
            }

            *begin.offset(buffer_index) = copy_copy;
            buffer_index = (buffer_index + 1) % buffer_size;

            ptr = ptr.offset(1);
            copy_ptr = copy_ptr.offset(1);
        }

        //At the end, restore the start (that will be the start from the copy)
        //Note that this will not be recursive as ranges will never overlap again
        copy(copy_to, copy_to.offset(buffer_size), begin);
    } else {
        while ptr != end {
            *copy_ptr = *ptr;

            ptr = ptr.offset(1);
            copy_ptr = copy_ptr.offset(1);
        }
    }
}
