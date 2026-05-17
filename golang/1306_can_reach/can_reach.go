// Package main ...
package main

func canReach(arr []int, start int) bool {
	if arr[start] == 0 {
		return true
	}

	n := len(arr)
	used := make([]bool, n)
	used[start] = true
	q := []int{start}

	for len(q) > 0 {
		u := q[0]
		q = q[1:]
		for _, v := range []int{u + arr[u], u - arr[u]} {
			if 0 <= v && v < n && !used[v] {
				if arr[v] == 0 {
					return true
				}
				q = append(q, v)
				used[v] = true
			}
		}
	}

	return false
}

func main() {
	tests := []struct {
		arr   []int
		start int
		ans   bool
	}{
		{[]int{4, 2, 3, 0, 3, 1, 2}, 5, true},
		{[]int{4, 2, 3, 0, 3, 1, 2}, 0, true},
		{[]int{3, 0, 2, 1, 2}, 2, false},
	}

	for _, test := range tests {
		ans := canReach(test.arr, test.start)
		if ans != test.ans {
			panic(test)
		}
	}
}
