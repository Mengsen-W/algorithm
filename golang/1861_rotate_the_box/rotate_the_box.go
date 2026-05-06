// Package main ...
package main

import (
	"bytes"
	"reflect"
)

func rotateTheBox(boxGrid [][]byte) [][]byte {
	m, n := len(boxGrid), len(boxGrid[0])
	ans := make([][]byte, n)
	for i := range ans {
		ans[i] = bytes.Repeat([]byte{'.'}, m)
	}

	for i, row := range boxGrid {
		k := n - 1
		for j := n - 1; j >= 0; j-- {
			if row[j] == '*' { // 障碍物
				ans[j][m-1-i] = '*'
				k = j - 1 // 障碍物左边最近的石头，在旋转后掉落到 j-1
			} else if row[j] == '#' { // 石头
				ans[k][m-1-i] = '#' // 旋转后，石头掉落到 k
				k--
			}
		}
	}

	return ans
}

func main() {
	tests := []struct {
		boxGrid [][]byte
		ans     [][]byte
	}{
		{
			[][]byte{
				{'#', '.', '#'},
			},
			[][]byte{
				{'.'},
				{'#'},
				{'#'},
			},
		},
		{
			[][]byte{
				{'#', '.', '*', '.'},
			},
			[][]byte{
				{'#', '.'},
				{'#', '#'},
				{'*', '*'},
				{'.', '.'},
			},
		},
		{
			[][]byte{
				{'#', '#', '*', '.', '*', '.'},
				{'#', '#', '#', '*', '.', '.'},
				{'#', '#', '#', '.', '#', '.'},
			},
			[][]byte{
				{'.', '#', '#'},
				{'.', '#', '#'},
				{'#', '#', '*'},
				{'#', '*', '.'},
				{'#', '.', '*'},
				{'#', '.', '.'},
			},
		},
	}

	for _, test := range tests {
		ans := rotateTheBox(test.boxGrid)
		if !reflect.DeepEqual(ans, test.ans) {
			panic("wrong")
		}
	}
}
