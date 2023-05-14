/*
 * @Date: 2023-05-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-14
 * @FilePath: /algorithm/rust/1054_rearrange_barcodes/rearrange_barcodes.rs
 */

pub fn rearrange_barcodes(mut barcodes: Vec<i32>) -> Vec<i32> {
    let n = barcodes.len();
    let mut j = 0;
    for i in 0..n - 1 {
        if barcodes[i] != barcodes[i + 1] {
            continue;
        }
        j = i + 2;
        while j < n && barcodes[i] == barcodes[j] {
            j += 1;
        }
        if j == n {
            break;
        }
        barcodes.swap(i + 1, j);
    }
    if j < n {
        return barcodes;
    }
    for i in (1..n).rev() {
        if barcodes[i] != barcodes[i - 1] {
            continue;
        }
        let mut j = i - 2;
        while barcodes[i] == barcodes[j] {
            j -= 1;
        }
        barcodes.swap(i - 1, j);
    }
    return barcodes;
}

fn main() {
    {
        let barcodes = vec![1, 1, 1, 2, 2, 2];
        let ans = vec![1, 2, 1, 2, 1, 2];
        assert_eq!(rearrange_barcodes(barcodes), ans);
    }

    {
        let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
        let ans = vec![1, 2, 1, 2, 1, 3, 1, 3];
        assert_eq!(rearrange_barcodes(barcodes), ans);
    }
}
