/*
 * @Date: 2022-04-24 09:47:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-24 09:56:58
 * @FilePath: /algorithm/868_binary_gap/binary_gap.go
 */

package main

func binaryGap(n int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for i, last := 0, -1; n > 0; i++ {
		if n&1 == 1 {
			if last != -1 {
				ans = max(ans, i-last)
			}
			last = i
		}
		n >>= 1
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(binaryGap(22) == 2)
	assert(binaryGap(8) == 0)
	assert(binaryGap(5) == 2)
}
