/*
 * @Date: 2022-08-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-15
 * @FilePath: /algorithm/768_max_chunks_to_sorted/max_chunks_to_sorted.go
 */

package main

func maxChunksToSorted(arr []int) int {
	st := []int{}
	for _, x := range arr {
		if len(st) == 0 || x >= st[len(st)-1] {
			st = append(st, x)
		} else {
			mx := st[len(st)-1]
			st = st[:len(st)-1]
			for len(st) > 0 && st[len(st)-1] > x {
				st = st[:len(st)-1]
			}
			st = append(st, mx)
		}
	}
	return len(st)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(maxChunksToSorted([]int{5, 4, 3, 2, 1}) == 1)
	assert(maxChunksToSorted([]int{2, 1, 3, 4, 4}) == 4)
}
