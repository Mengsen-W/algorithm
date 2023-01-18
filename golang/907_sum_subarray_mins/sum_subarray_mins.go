/*
 * @Date: 2022-10-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-28
 * @FilePath: /algorithm/907_sum_subarray_mins/sum_subarray_mins.go
 */

package main

func sumSubarrayMins(arr []int) (ans int) {
	const mod int = 1e9 + 7
	n := len(arr)
	left := make([]int, n)
	right := make([]int, n)
	monoStack := []int{}
	for i, x := range arr {
		for len(monoStack) > 0 && x <= arr[monoStack[len(monoStack)-1]] {
			monoStack = monoStack[:len(monoStack)-1]
		}
		if len(monoStack) == 0 {
			left[i] = i + 1
		} else {
			left[i] = i - monoStack[len(monoStack)-1]
		}
		monoStack = append(monoStack, i)
	}
	monoStack = []int{}
	for i := n - 1; i >= 0; i-- {
		for len(monoStack) > 0 && arr[i] < arr[monoStack[len(monoStack)-1]] {
			monoStack = monoStack[:len(monoStack)-1]
		}
		if len(monoStack) == 0 {
			right[i] = n - i
		} else {
			right[i] = monoStack[len(monoStack)-1] - i
		}
		monoStack = append(monoStack, i)
	}
	for i, x := range arr {
		ans = (ans + left[i]*right[i]*x) % mod
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
		arr := []int{3, 1, 2, 4}
		ans := 17
		assert(sumSubarrayMins(arr) == ans)
	}

	{
		arr := []int{11, 81, 94, 43, 3}
		ans := 444
		assert(sumSubarrayMins(arr) == ans)
	}
}
