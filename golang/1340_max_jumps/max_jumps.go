// Package main ...
package main

import "fmt"

func maxJumps(arr []int, d int) int {
	n := len(arr)
	f := make([]int, n)
	for i := range f {
		f[i] = -1
	}

	var dfs func(int)
	dfs = func(id int) {
		if f[id] != -1 {
			return
		}
		f[id] = 1
		for i := id - 1; i >= 0 && id-i <= d && arr[id] > arr[i]; i-- {
			dfs(i)
			if f[i]+1 > f[id] {
				f[id] = f[i] + 1
			}
		}
		for i := id + 1; i < n && i-id <= d && arr[id] > arr[i]; i++ {
			dfs(i)
			if f[i]+1 > f[id] {
				f[id] = f[i] + 1
			}
		}
	}

	for i := 0; i < n; i++ {
		dfs(i)
	}

	ans := 0
	for _, val := range f {
		if val > ans {
			ans = val
		}
	}
	return ans
}

func main() {
	tests := []struct {
		arr []int
		d   int
		ans int
	}{
		{[]int{6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12}, 2, 4},
		{[]int{3, 3, 3, 3, 3}, 3, 1},
		{[]int{7, 6, 5, 4, 3, 2, 1}, 1, 7},
		{[]int{7, 1, 7, 1, 7, 1}, 2, 2},
		{[]int{66}, 1, 1},
	}
	for _, test := range tests {
		if actual := maxJumps(test.arr, test.d); actual != test.ans {
			panic(fmt.Sprintf("Failed for input: %v with d=%d. Expected: %d, Got: %d", test.arr, test.d, test.ans, actual))
		}
	}
}
