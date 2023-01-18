/*
 * @Date: 2022-02-03 14:07:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-03 14:47:20
 */

package main

func findMinFibonacciNumbers(k int) (ans int) {
	f := []int{1, 1}
	for f[len(f)-1] < k {
		f = append(f, f[len(f)-1]+f[len(f)-2])
	}
	for i := len(f) - 1; i >= 0 && k > 0; i-- {
		if k >= f[i] {
			k -= f[i]
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
	assert(findMinFibonacciNumbers(7) == 2)
	assert(findMinFibonacciNumbers(10) == 2)
	assert(findMinFibonacciNumbers(19) == 3)
}
