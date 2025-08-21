use std::{
    collections::{HashSet, VecDeque},
    fs, usize,
};

#[derive(Debug)]
enum FileBlocks {
    File(File),
    Empty(EmptyBlock),
}

/// Structure of a file in the filesystem. Each file has an id and a size.
#[derive(Debug)]
struct File {
    /// File id.
    id: u64,
    /// File size.
    size: u64,
}

impl File {
    /// Create a new file with an id and size.
    ///
    /// # Arguments
    ///     * `id` - File id.
    ///     * `size` - File size.
    fn new(id: u64, size: u64) -> Self {
        Self { id, size }
    }
}

/// Structure of an empty block in the filesystem. Each empty block has a size.
#[derive(Debug)]
struct EmptyBlock {
    /// Block size.
    size: u64,
}

impl EmptyBlock {
    /// Create a new empty block with a size.
    ///
    /// # Arguments
    ///     * `size` - Block size.
    fn new(size: u64) -> Self {
        Self { size }
    }
}

// Calculate the sum of numbers between two numbers.
///
/// # Arguments
///   * `start` - Start number.
///   * `end` - End number.
fn sum_between(start: u64, end: u64) -> u64 {
    assert!(start <= end);
    let n = end - start + 1;
    n * (start + end) / 2
}

/// Organize the filesystem by moving files to the leftmost free block from the file.
/// This function does not allow for fragmentation i.e. a given file will not be broken into
/// seperate blocks when moved.
/// The function returns the checksum of the organized filesystem.
///
/// # Arguments
///    * `file_blocks` - Vector of file blocks in the filesystem. Each element in the
///       vector represents the size of the blocks. It is in alternating between blocks
///       of a file and empty blocks i.e. the first element is a file with a size equivalent
///       to the value at index 0.
fn organize_filesystem(file_blocks: &Vec<u64>) -> u64 {
    let mut unorganized_files = file_blocks.clone();
    let mut organized_files_idx: HashSet<usize> = HashSet::new();
    let mut organized_filesystem: VecDeque<FileBlocks> = VecDeque::new();
    let mut leftmost_blocks_idx: usize = 1;
    let mut rightmost_unorganized_file_idx = file_blocks.len() - 1;
    if file_blocks.len() % 2 == 0 {
        // If there are an even number of blocks, the
        // rightmost unorganized index is decremented by 1.
        rightmost_unorganized_file_idx -= 1;
    }

    // The approach to organization in the function tries to fill the left most empty space first.
    // i.e. it finds the right most file that is less than or equal to the size of the left most empty
    // blocks.

    organized_filesystem.push_back(FileBlocks::File(File::new(0, unorganized_files[0])));
    organized_files_idx.insert(0);

    while rightmost_unorganized_file_idx > leftmost_blocks_idx {
        let mut file_moved = false;
        let mut right_idx = rightmost_unorganized_file_idx;

        // Find the right most file that can fill the leftmost empty block.
        while right_idx > leftmost_blocks_idx {
            if unorganized_files[right_idx] > unorganized_files[leftmost_blocks_idx]
                || organized_files_idx.contains(&right_idx)
            {
                // Current right file cannot fill the empty blocks if it has already been moved
                // or it is too big for the empty blocks.
                right_idx -= 2;
                continue;
            }

            organized_filesystem.push_back(FileBlocks::File(File::new(
                (right_idx / 2) as u64,
                unorganized_files[right_idx],
            )));
            organized_files_idx.insert(right_idx);
            unorganized_files[leftmost_blocks_idx] -= unorganized_files[right_idx];
            file_moved = true;

            break;
        }

        if file_moved && right_idx == rightmost_unorganized_file_idx {
            // If the rightmost unorganized file has been moved, we have a new rightmost unorganized file.
            rightmost_unorganized_file_idx -= 2;
        }

        if unorganized_files[leftmost_blocks_idx] == 0 || !file_moved {
            if unorganized_files[leftmost_blocks_idx] != 0 {
                // If the current leftmost block has been not been used up at this point, it cannot be filled up.
                match organized_filesystem.back_mut() {
                    Some(FileBlocks::File(_)) => {
                        organized_filesystem.push_back(FileBlocks::Empty(EmptyBlock::new(
                            unorganized_files[leftmost_blocks_idx],
                        )));
                    }
                    Some(FileBlocks::Empty(empty)) => {
                        empty.size += unorganized_files[leftmost_blocks_idx];
                    }
                    _ => {}
                }
            }

            // Move the next leftmost block.
            leftmost_blocks_idx += 1;

            if leftmost_blocks_idx % 2 == 0 && !organized_files_idx.contains(&leftmost_blocks_idx) {
                // If the next left most block is a file and it has not been moved, it cannot be moved.
                organized_filesystem.push_back(FileBlocks::File(File::new(
                    (leftmost_blocks_idx / 2) as u64,
                    unorganized_files[leftmost_blocks_idx],
                )));
                organized_files_idx.insert(leftmost_blocks_idx);

                // Move to the next empty block sicne the current leftmost block is a file that cannot be moved.
                leftmost_blocks_idx += 1;
            }

            // If the leftmost block was a file and it has been moved prior, this is considered an empty block now.
        }
    }

    // Calculate checksum of organized files.
    let mut cummulative_sum = 0;
    let mut current_block_idx = 0;
    for file_block in &organized_filesystem {
        match file_block {
            FileBlocks::File(file) => {
                let block_idx_sum =
                    sum_between(current_block_idx, current_block_idx + file.size - 1);
                cummulative_sum += file.id * block_idx_sum;
                current_block_idx += file.size;
            }
            FileBlocks::Empty(empty) => {
                current_block_idx += empty.size;
            }
        }
    }

    cummulative_sum
}

fn organize_filesystem_fragmented(file_blocks: &Vec<u64>) -> u64 {
    // Since the file representation alternates between occupied and free blocks
    // the number of files can be calculated by taking advantage of integer division
    // to account for when there is an odd number of blocks.
    let file_count = (file_blocks.len() + 1) / 2;

    let mut left_idx = 0;
    let mut right_idx = file_blocks.len() - 1;
    if file_blocks.len() % 2 == 0 {
        // If there are an even number of blocks, the
        right_idx -= 1;
    }

    // Vector of files in order after defragmentation. Each value in the vector is the
    // a tuple: (file_id, file_size).
    let mut compact_files: Vec<(u64, u64)> = Vec::new();

    let mut left_size = file_blocks[left_idx];
    let mut left_file_id = 0;
    let mut right_file_id = file_count as u64 - 1;
    let mut right_size = file_blocks[right_idx];

    let mut adjust_leftmost = true;
    let mut adjust_rightmost = false;

    while left_idx < right_idx {
        if adjust_leftmost {
            // Store next leftmost file.
            left_size = file_blocks[left_idx];
            compact_files.push((left_file_id, left_size));
            left_file_id += 1;

            // Move to next leftmost free block
            left_idx += 1;
            left_size = file_blocks[left_idx];

            // If leftmost free block has a value of 0, we need to adjust again.
            //
            adjust_leftmost = left_size == 0;
            left_idx += adjust_leftmost as usize;

            continue;
        }

        if adjust_rightmost {
            // Move to next rightmost occupied block.
            right_idx -= 2;
            right_file_id -= 1;
            right_size = file_blocks[right_idx];

            adjust_rightmost = false;
            continue;
        }

        if right_size <= left_size {
            // Store rightmost file in leftmost free blocks
            compact_files.push((right_file_id, right_size));
            left_size -= right_size;
            right_size = 0;
            adjust_rightmost = true;
            adjust_leftmost = left_size == 0;
        } else {
            // Store rightmost file to leftmost free blocks
            compact_files.push((right_file_id, left_size));
            right_size -= left_size;
            adjust_leftmost = true;
        }

        left_idx += adjust_leftmost as usize;
    }

    // Record any left overs in the right block.
    if right_size > 0 && (left_idx == right_idx) {
        compact_files.push((right_file_id, right_size));
    }

    let mut checksum = 0;
    let mut cummulative_size = 0;
    for (file_id, file_size) in compact_files {
        checksum += file_id * (cummulative_size..cummulative_size + file_size).sum::<u64>();
        cummulative_size += file_size;
    }

    checksum
}

pub fn main(file_path: String) {
    let content: Vec<u64> = fs::read_to_string(file_path)
        .unwrap()
        .chars()
        .map(|x| {
            x.to_digit(10)
                .unwrap() as u64
        })
        .collect();

    let checksum = organize_filesystem_fragmented(&content);
    println!("Checksum Fragmented: {checksum}");

    let checksum = organize_filesystem(&content);
    println!("Checksum Non-Fragmented: {checksum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let content: Vec<u64> = String::from("2333133121414131402")
            .chars()
            .map(|x| {
                x.to_digit(10)
                    .unwrap() as u64
            })
            .collect();
        assert!(organize_filesystem_fragmented(&content) == 1928);
        assert!(organize_filesystem(&content) == 2858);
    }

    #[test]
    fn test2() {
        let content: Vec<u64> = String::from("111112121")
            .chars()
            .map(|x| {
                x.to_digit(10)
                    .unwrap() as u64
            })
            .collect();
        assert!(organize_filesystem_fragmented(&content) == 23);
        assert!(organize_filesystem(&content) == 23);
    }
}
