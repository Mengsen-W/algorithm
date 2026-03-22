// Package main ...
package main

func findRotation(mat [][]int, target [][]int) bool {
	n := len(mat)
	// 最多旋转 4 次
	for k := 0; k < 4; k++ {
		// 旋转操作
		for i := 0; i < n/2; i++ {
			for j := 0; j < (n+1)/2; j++ {
				mat[i][j], mat[n-1-j][i], mat[n-1-i][n-1-j], mat[j][n-1-i] = mat[n-1-j][i], mat[n-1-i][n-1-j], mat[j][n-1-i], mat[i][j]
			}
		}

		if isEqual(mat, target) {
			return true
		}
	}
	return false
}

func isEqual(mat, target [][]int) bool {
	n := len(mat)
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if mat[i][j] != target[i][j] {
				return false
			}
		}
	}
	return true
}

func main() {
	tests := []struct {
		mat    [][]int
		target [][]int
		ans    bool
	}{
		{[][]int{{0, 1}, {1, 0}}, [][]int{{1, 0}, {0, 1}}, true},
		{[][]int{{0, 1}, {1, 1}}, [][]int{{1, 0}, {0, 1}}, false},
		{[][]int{{0, 0, 0}, {0, 1, 0}, {1, 1, 1}}, [][]int{{1, 1, 1}, {0, 1, 0}, {0, 0, 0}}, true},
	}

	for _, test := range tests {
		ans := findRotation(test.mat, test.target)
		if ans != test.ans {
			panic("wrong")
		}
	}
}
