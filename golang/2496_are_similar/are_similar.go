// Package main ...
package main

func areSimilar(mat [][]int, k int) bool {
	m := len(mat)
	n := len(mat[0])
	k %= n

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if mat[i][j] != mat[i][(j+k)%n] {
				return false
			}
		}
	}
	return true
}

func main() {
	tests := []struct {
		mat [][]int
		k   int
		ans bool
	}{
		{[][]int{{1, 2, 1, 2}, {5, 5, 5, 5}, {6, 3, 6, 3}}, 2, true},
		{[][]int{{2, 2}, {2, 2}}, 3, true},
		{[][]int{{1, 2}}, 1, false},
	}

	for _, test := range tests {
		if got := areSimilar(test.mat, test.k); got != test.ans {
			panic(got)
		}
	}
}
