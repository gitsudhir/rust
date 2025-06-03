use std::{
    cmp::{max, min},
    i32::MIN,
};

pub fn largest_container(heights: Vec<i32>) -> i32 {
    // edge case
    if heights.len() == 1 {
        return 0;
    }

    let mut first_pointer = 0;
    let mut last_pointer = heights.len() - 1;
    let mut max_water = MIN;

    while first_pointer < last_pointer {
        // Calculate the water contained between the current pair of lines.
        let water = min(heights[first_pointer], heights[last_pointer])
            * (last_pointer as i32 - first_pointer as i32);
            //  Move the pointers inward, always moving the pointer at the 
            //  shorter line. If both lines have the same height, move both 
            //  pointers inward.
        max_water = max(max_water, water);
        if heights[first_pointer] < heights[last_pointer] {
            first_pointer += 1;
        } else if heights[first_pointer] > heights[last_pointer] {
            last_pointer -= 1;
        } else {
            first_pointer += 1;
            last_pointer -= 1;
        }
    }
    max_water
}
