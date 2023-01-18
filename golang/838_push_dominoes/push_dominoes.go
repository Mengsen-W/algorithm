/*
 * @Date: 2022-02-21 00:58:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-21 01:05:42
 */

package main

func pushDominoes(dominoes string) string {
	s := []byte(dominoes)
	i, n, left := 0, len(s), byte('L')
	for i < n {
		j := i
		for j < n && s[j] == '.' {
			// 找到一段连续的没有被推动的骨牌
			j++
		}
		right := byte('R')
		if j < n {
			right = s[j]
		}
		if left == right {
			// 方向相同，那么这些竖立骨牌也会倒向同一方向
			for i < j {
				s[i] = right
				i++
			}
		} else if left == 'R' && right == 'L' {
			// 方向相对，那么就从两侧向中间倒
			k := j - 1
			for i < k {
				s[i] = 'R'
				s[k] = 'L'
				i++
				k--
			}
		}
		left = right
		i = j + 1
	}
	return string(s)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(pushDominoes("RR.L") == "RR.L")
	assert(pushDominoes(".L.R...LR..L..") == "LL.RR.LLRRLL..")
}
