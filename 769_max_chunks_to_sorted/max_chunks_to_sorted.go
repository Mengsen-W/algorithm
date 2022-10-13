/*
 * @Date: 2022-10-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-13
 * @FilePath: /algorithm/769_max_chunks_to_sorted/max_chunks_to_sorted.go
 */

package main

func maxChunksToSorted(arr []int) (ans int) {
	mx := 0
	for i, x := range arr {
		if x > mx {
			mx = x
		}
		if mx == i {
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
	{
		arr := []int{4, 3, 2, 1, 0}
		ans := 1
		assert(maxChunksToSorted(arr) == ans)
	}

	{
		arr := []int{1, 0, 2, 3, 4}
		ans := 4
		assert(maxChunksToSorted(arr) == ans)
	}
}
