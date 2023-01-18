/*
 * @Date: 2023-01-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-14
 * @FilePath: /algorithm/1819_count_different_subsequence_gc_ds/count_different_subsequence_gc_ds.go
 */

package main

func countDifferentSubsequenceGCDs(nums []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	gcd := func(num1, num2 int) int {
		for num1 != 0 {
			num1, num2 = num2%num1, num1
		}
		return num2
	}
	maxVal := 0
	for _, num := range nums {
		maxVal = max(maxVal, num)
	}
	occured := make([]bool, maxVal+1)
	for _, num := range nums {
		occured[num] = true
	}
	for i := 1; i <= maxVal; i++ {
		subGcd := 0
		for j := i; j <= maxVal; j += i {
			if occured[j] {
				if subGcd == 0 {
					subGcd = j
				} else {
					subGcd = gcd(subGcd, j)
				}
				if subGcd == i {
					ans++
					break
				}
			}
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
	{
		nums := []int{6, 10, 3}
		ans := 5
		assert(countDifferentSubsequenceGCDs(nums) == ans)
	}

	{
		nums := []int{5, 15, 40, 5, 6}
		ans := 7
		assert(countDifferentSubsequenceGCDs(nums) == ans)
	}
}
