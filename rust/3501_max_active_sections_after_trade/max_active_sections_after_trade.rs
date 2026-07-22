struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let n = s.len();
        let m = queries.len();
        let s_bytes = s.as_bytes();

        let mut cnt1: i32 = 0;
        for &c in s_bytes {
            if c == b'1' {
                cnt1 += 1;
            }
        }

        // left[i]：表示以位置 i 结尾，与 s[i] 相同的连续区块长度
        let mut left = vec![0i32; n];
        // right[i]：表示以位置 i 开始，与 s[i] 相同的连续区块长度
        let mut right = vec![0i32; n];

        for i in 0..n {
            left[i] = if i > 0 && s_bytes[i - 1] == s_bytes[i] {
                left[i - 1] + 1
            } else {
                1
            };
        }
        for i in (0..n).rev() {
            right[i] = if i < n - 1 && s_bytes[i + 1] == s_bytes[i] {
                right[i + 1] + 1
            } else {
                1
            };
        }

        let mut ans = vec![-1i32; m];
        let block_size = (n as f64).sqrt() as usize;
        let block_size = if block_size < 1 { 1 } else { block_size };
        // 长度大于块长的询问
        let mut long_queries: Vec<[usize; 4]> = Vec::new();

        let brute_force = |l: usize, r: usize| -> i32 {
            let mut i = l;
            let mut best = 0i32;
            let mut prev = i32::MIN;

            while i <= r {
                let start = i;
                while i <= r && s_bytes[i] == s_bytes[start] {
                    i += 1;
                }
                if s_bytes[start] == b'0' {
                    let cur = (i - start) as i32;
                    if prev != i32::MIN && prev + cur > best {
                        best = prev + cur;
                    }
                    prev = cur;
                }
            }
            best
        };

        for i in 0..m {
            let l = queries[i][0] as usize;
            let r = queries[i][1] as usize;
            if r - l + 1 > block_size {
                long_queries.push([l / block_size, l, r, i]);
            } else {
                // 长度小于块长的询问，暴力计算
                ans[i] = cnt1 + brute_force(l, r);
            }
        }

        // 以询问左端点所在块的 ID 为第一关键字，询问右端点为第二关键字排序
        long_queries.sort_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                a[2].cmp(&b[2])
            }
        });

        let mut sub_zero_blocks: VecDeque<i32> = VecDeque::new();
        let mut l_ptr: usize = 0;
        let mut r_ptr: usize = 0;
        let mut best_gain: i32 = 0;

        for i in 0..long_queries.len() {
            let bid = long_queries[i][0];
            let l = long_queries[i][1];
            let r = long_queries[i][2];
            let qid = long_queries[i][3];

            if i == 0 || bid > long_queries[i - 1][0] {
                // 遍历到一个新的块, 进行初始化操作
                l_ptr = (bid + 1) * block_size - 1; // L 初始化为该块右端点
                r_ptr = (bid + 1) * block_size; // R 初始化为下一块左端点
                sub_zero_blocks.clear();
                best_gain = 0;
            }

            while r_ptr <= r {
                let mut sz = right[r_ptr] as usize;
                if r - r_ptr + 1 < sz {
                    sz = r - r_ptr + 1;
                }
                if s_bytes[r_ptr] == b'0' {
                    if !sub_zero_blocks.is_empty() && r_ptr > 0 && s_bytes[r_ptr - 1] == b'0' {
                        if let Some(back) = sub_zero_blocks.pop_back() {
                            sub_zero_blocks.push_back(back + sz as i32);
                        }
                    } else {
                        sub_zero_blocks.push_back(sz as i32);
                    }
                    if sub_zero_blocks.len() >= 2 {
                        let last = *sub_zero_blocks.back().unwrap();
                        let second_last = sub_zero_blocks[sub_zero_blocks.len() - 2];
                        best_gain = best_gain.max(last + second_last);
                    }
                }
                r_ptr += sz;
            }

            // 移动左端点 L 前，备份 bestGain 的值
            let tmp_best_gain = best_gain;
            // 移动左端点前，subZeroBlocks第一个元素（如果有）的值
            let tmp_first_value = sub_zero_blocks.front().copied();
            // 记录移动左端点 L 的过程中，从左侧加入的数字数量
            let mut cnt = 0;

            while l_ptr >= l {
                let mut sz = left[l_ptr] as usize;
                if l_ptr - l + 1 < sz {
                    sz = l_ptr - l + 1;
                }
                if s_bytes[l_ptr] == b'0' {
                    if !sub_zero_blocks.is_empty() && l_ptr + 1 < n && s_bytes[l_ptr + 1] == b'0' {
                        if let Some(front) = sub_zero_blocks.pop_front() {
                            sub_zero_blocks.push_front(front + sz as i32);
                        }
                    } else {
                        sub_zero_blocks.push_front(sz as i32);
                        cnt += 1;
                    }
                    if sub_zero_blocks.len() >= 2 {
                        let first = *sub_zero_blocks.front().unwrap();
                        let second = sub_zero_blocks[1];
                        best_gain = best_gain.max(first + second);
                    }
                }
                if l_ptr >= sz {
                    l_ptr -= sz;
                } else {
                    break;
                }
            }

            // 回答询问
            ans[qid] = best_gain + cnt1;
            // 还原左端点 L
            l_ptr = (bid + 1) * block_size - 1;
            if l_ptr >= n {
                l_ptr = n - 1;
            }
            // 还原 bestGain
            best_gain = tmp_best_gain;
            // 还原 subZeroBlocks
            for _ in 0..cnt {
                sub_zero_blocks.pop_front();
            }
            if let Some(first_val) = tmp_first_value {
                if !sub_zero_blocks.is_empty() {
                    sub_zero_blocks.pop_front();
                    sub_zero_blocks.push_front(first_val);
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        ("01", vec![vec![0, 1]], vec![1]),
        (
            "0100",
            vec![vec![0, 3], vec![0, 2], vec![1, 3], vec![2, 3]],
            vec![4, 3, 1, 1],
        ),
        (
            "1000100",
            vec![vec![1, 5], vec![0, 6], vec![0, 4]],
            vec![6, 7, 2],
        ),
        (
            "01010",
            vec![vec![0, 3], vec![1, 4], vec![1, 3]],
            vec![4, 4, 2],
        ),
    ];

    for (s, queries, ans) in tests {
        assert_eq!(
            Solution::max_active_sections_after_trade(s.to_string(), queries),
            ans
        );
    }
}
