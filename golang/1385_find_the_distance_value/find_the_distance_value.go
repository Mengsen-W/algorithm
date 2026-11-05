// Package main ...
package main

import "sort"

func findTheDistanceValue(arr1 []int, arr2 []int, d int) int {
	sort.Ints(arr2)
	cnt := 0

	for _, x := range arr1 {
		p := sort.SearchInts(arr2, x)

		ok := true
		if p < len(arr2) {
			ok = ok && (arr2[p]-x > d)
		}
		if p-1 >= 0 && p-1 < len(arr2) {
			ok = ok && (x-arr2[p-1] > d)
		}

		if ok {
			cnt++
		}
	}
	return cnt
}

func main() {
	tests := []struct {
		arr1 []int
		arr2 []int
		d    int
		ans  int
	}{
		{[]int{4, 5, 8}, []int{10, 9, 1, 8}, 2, 2},
		{[]int{1, 4, 2, 3}, []int{-4, -3, 6, 10, 20, 30}, 3, 2},
		{[]int{2, 1, 100, 3}, []int{-5, -2, 10, -3, 7}, 6, 1},
	}

	for _, t := range tests {
		if ans := findTheDistanceValue(t.arr1, t.arr2, t.d); ans != t.ans {
			panic("wrong answer")
		}
	}
}
