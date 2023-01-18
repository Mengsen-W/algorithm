/*
 * @Date: 2022-06-03 23:11:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-03 23:16:11
 * @FilePath: /algorithm/829_consecutive_numbers_sum/consecutive_numbers_sum.go
 */

package main

func isKConsecutive(n, k int) bool {
	if k%2 == 1 {
		return n%k == 0
	}
	return n%k != 0 && 2*n%k == 0
}

func consecutiveNumbersSum(n int) (ans int) {
	for k := 1; k*(k+1) <= n*2; k++ {
		if isKConsecutive(n, k) {
			ans++
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(consecutiveNumbersSum(5) == 2)
	assert(consecutiveNumbersSum(9) == 3)
	assert(consecutiveNumbersSum(15) == 4)
}
