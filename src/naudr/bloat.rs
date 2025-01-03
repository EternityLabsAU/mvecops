use crate::Vmatrix;
use crate::Trigonometric;

use crate::GlobalCurveData;

// First step is skippable, the bload data can just copy the initial input
/// Write on the global output the minimal points on the set that contain the rest of the data. A point
/// on a set can "contain" the data surrounding it if all data at a "radius" r is also occupied. The maximum
/// radius you can go from a single point and walk a full square while always having data on the path is the
/// value that entry gets.
///
/// # Testing
///
/// Running a test of the crate generates a file called "bloating.txt" from the "samplekanji.txt" to show
/// an example of the operation.
///
pub fn write_bloats(global_data: &mut GlobalCurveData, input_data: &Vmatrix<u32>) {
    let row_size = global_data.row_size;
    let data_size = row_size * row_size;
    
    let mut bloat_level: u32 = 1;
    let maximum_bloat: u32 = (row_size / 2) as u32;

    let mut data_array = &input_data.data;
    let mut result_array = &mut global_data.curves_global_output.data;
    let mut bloat_increased = true;
    while bloat_level < maximum_bloat && bloat_increased {
        bloat_increased = false;
        for i in 0..data_size {
            if input_data.test_border_index(i) {
                continue;
            }
            if data_array[i] == 1 && result_array[i] != bloat_level {
                if trace_at(input_data, bloat_level, row_size, i) {
                    result_array[i] = bloat_level;
                    bloat_increased = true;
                }
            }
        }
        bloat_level += 1;
    }

    while bloat_level > 0 {
        for i in 0..data_size {
            if input_data.test_border_index(i) {
                continue;
            }
            if data_array[i] == 1 && result_array[i] == bloat_level {
                let bloat_limit = bloat_level + 1;
                for j in 1..bloat_limit {
                    clean_at(input_data, bloat_level, j, row_size, i, result_array);
                }
            }
        }
        bloat_level -= 1;
    }
}

/// Given a point that already has a radius assigned, this method allows to eliminate entries at a smaller
/// radius, given that it no longer provides information. If the method finds an entry equal or bigger to
/// the input radius, it doesn't delete it.
///
pub fn clean_at(full_data: &Vmatrix<u32>, bloat_level_cleaning: u32, bloat_level_cleaned: u32, row_size: usize, current_index: usize, result_array: &mut Vec<u32>) -> bool {
    let input_data = &full_data.data;

    let bloat_half_step = bloat_level_cleaned - 1;
    let bloat_full_step = bloat_half_step * 2;

    let mut current_direction = Trigonometric::SIN;

    let mut index_at_12 = current_index;
    for x in 0..bloat_half_step {
        index_at_12 = Trigonometric::get_index_from_direction(index_at_12, row_size, &current_direction, 0);
        if full_data.test_border_index(index_at_12) {
            return false;
        }
    }
    if result_array[index_at_12] < bloat_level_cleaning {
        result_array[index_at_12] = 0;
    }

    current_direction = Trigonometric::COS;
    let mut index_until_hpone = index_at_12;
    for x in 0..bloat_half_step {
        index_until_hpone = Trigonometric::get_index_from_direction(index_until_hpone, row_size, &current_direction, 0);
        if result_array[index_until_hpone] < bloat_level_cleaning {
            result_array[index_until_hpone] = 0;
        }
        if full_data.test_border_index(index_until_hpone) {
            return false;
        }
    }

    current_direction = Trigonometric::NSIN;
    let mut index_until_hpfour = index_until_hpone;
    for x in 0..bloat_full_step {
        index_until_hpfour = Trigonometric::get_index_from_direction(index_until_hpfour, row_size, &current_direction, 0);
        if result_array[index_until_hpfour] < bloat_level_cleaning {
            result_array[index_until_hpfour] = 0;
        }
        if full_data.test_border_index(index_until_hpfour) {
            return false;
        }
    }

    current_direction = Trigonometric::NCOS;
    let mut index_until_hpseven = index_until_hpfour;
    for x in 0..bloat_full_step {
        index_until_hpseven = Trigonometric::get_index_from_direction(index_until_hpseven, row_size, &current_direction, 0);
        if result_array[index_until_hpseven] < bloat_level_cleaning {
            result_array[index_until_hpseven] = 0;
        }
        if full_data.test_border_index(index_until_hpseven) {
            return false;
        }
    }

    current_direction = Trigonometric::SIN;
    let mut index_until_hpeleven = index_until_hpseven;
    for x in 0..bloat_full_step {
        index_until_hpeleven = Trigonometric::get_index_from_direction(index_until_hpeleven, row_size, &current_direction, 0);
        if result_array[index_until_hpeleven] < bloat_level_cleaning {
            result_array[index_until_hpeleven] = 0;
        }
        if full_data.test_border_index(index_until_hpeleven) {
            return false;
        }
    }

    current_direction = Trigonometric::COS;
    let mut index_until_12 = index_until_hpeleven;
    for x in 0..bloat_half_step {
        index_until_12 = Trigonometric::get_index_from_direction(index_until_12, row_size, &current_direction, 0);
        if result_array[index_until_12] < bloat_level_cleaning {
            result_array[index_until_12] = 0;
        }
        if full_data.test_border_index(index_until_12) {
            return false;
        }
    }

    return true;
}

/// Check if walking a square of radius <bloat_level> around the given <current_index>, all the entries have data.
/// If they don't contain data or, while trying to walk around the given, the index falls to the borders of the
/// matrix, it returns false.
///
pub fn trace_at(full_data: &Vmatrix<u32>, bloat_level: u32, row_size: usize, current_index: usize) -> bool {
    let input_data = &full_data.data;

    let bloat_half_step = bloat_level - 1;
    let bloat_full_step = bloat_half_step * 2;

    let mut current_direction = Trigonometric::SIN;

    let mut index_at_12 = current_index;
    for x in 0..bloat_half_step {
        index_at_12 = Trigonometric::get_index_from_direction(index_at_12, row_size, &current_direction, 0);
        if full_data.test_border_index(index_at_12) {
            return false;
        }
    }
    if input_data[index_at_12] < 1 {
        return false;
    }

    current_direction = Trigonometric::COS;
    let mut index_until_hpone = index_at_12;
    for x in 0..bloat_half_step {
        index_until_hpone = Trigonometric::get_index_from_direction(index_until_hpone, row_size, &current_direction, 0);
        if input_data[index_until_hpone] < 1 {
            return false;
        }
        if full_data.test_border_index(index_until_hpone) {
            return false;
        }
    }

    current_direction = Trigonometric::NSIN;
    let mut index_until_hpfour = index_until_hpone;
    for x in 0..bloat_full_step {
        index_until_hpfour = Trigonometric::get_index_from_direction(index_until_hpfour, row_size, &current_direction, 0);
        if input_data[index_until_hpfour] < 1 {
            return false;
        }
        if full_data.test_border_index(index_until_hpfour) {
            return false;
        }
    }

    current_direction = Trigonometric::NCOS;
    let mut index_until_hpseven = index_until_hpfour;
    for x in 0..bloat_full_step {
        index_until_hpseven = Trigonometric::get_index_from_direction(index_until_hpseven, row_size, &current_direction, 0);
        if input_data[index_until_hpseven] < 1 {
            return false;
        }
        if full_data.test_border_index(index_until_hpseven) {
            return false;
        }
    }

    current_direction = Trigonometric::SIN;
    let mut index_until_hpeleven = index_until_hpseven;
    for x in 0..bloat_full_step {
        index_until_hpeleven = Trigonometric::get_index_from_direction(index_until_hpeleven, row_size, &current_direction, 0);
        if input_data[index_until_hpeleven] < 1 {
            return false;
        }
        if full_data.test_border_index(index_until_hpeleven) {
            return false;
        }
    }

    current_direction = Trigonometric::COS;
    let mut index_until_12 = index_until_hpeleven;
    for x in 0..bloat_half_step {
        index_until_12 = Trigonometric::get_index_from_direction(index_until_12, row_size, &current_direction, 0);
        if input_data[index_until_12] < 1 {
            return false;
        }
        if full_data.test_border_index(index_until_12) {
            return false;
        }
    }

    return true;
}