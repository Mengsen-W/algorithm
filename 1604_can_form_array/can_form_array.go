/*
 * @Date: 2022-09-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-22
 * @FilePath: /algorithm/1604_can_form_array/can_form_array.go
 */

package main

func canFormArray(arr []int, pieces [][]int) bool {
	index := make(map[int]int, len(pieces))
	for i, p := range pieces {
		index[p[0]] = i
	}
	for i := 0; i < len(arr); {
		j, ok := index[arr[i]]
		if !ok {
			return false
		}
		for _, x := range pieces[j] {
			if arr[i] != x {
				return false
			}
			i++
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		arr := []int{15, 88}
		pieces := [][]int{{88}, {15}}
		assert(canFormArray(arr, pieces))
	}

	{
		arr := []int{49, 18, 16}
		pieces := [][]int{{16, 18, 49}}
		assert(!canFormArray(arr, pieces))
	}

	{
		arr := []int{91, 4, 64, 78}
		pieces := [][]int{{78}, {4, 64}, {91}}
		assert(canFormArray(arr, pieces))
	}
}
