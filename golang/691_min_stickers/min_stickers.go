/*
 * @Date: 2022-05-14 09:03:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-14 09:40:42
 * @FilePath: /algorithm/691_min_stickers/min_stickers.go
 */

package main

func minStickers(stickers []string, target string) int {
	m := len(target)
	f := make([]int, 1<<m)
	for i := range f {
		f[i] = -1
	}
	f[0] = 0

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	var dp func(int) int
	dp = func(mask int) int {
		if f[mask] != -1 {
			return f[mask]
		}
		f[mask] = m + 1
		for _, sticker := range stickers {
			left := mask
			cnt := [26]int{}
			for _, c := range sticker {
				cnt[c-'a']++
			}
			for i, c := range target {
				if mask>>i&1 == 1 && cnt[c-'a'] > 0 {
					cnt[c-'a']--
					left ^= 1 << i
				}
			}
			if left < mask {
				f[mask] = min(f[mask], dp(left)+1)
			}
		}
		return f[mask]
	}
	ans := dp(1<<m - 1)
	if ans <= m {
		return ans
	}
	return -1
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}

	assert(minStickers([]string{"with", "example", "science"}, "thehat"), 3)
	assert(minStickers([]string{"notice", "possible"}, "basicbasic"), -1)
}
