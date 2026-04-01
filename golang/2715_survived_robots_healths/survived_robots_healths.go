// Package main ...
package main

import (
	"fmt"
	"reflect"
	"sort"
)

func survivedRobotsHealths(positions []int, healths []int, directions string) []int {
	n := len(positions)
	idx := make([]int, n)
	for i := 0; i < n; i++ {
		idx[i] = i
	}

	sort.Slice(idx, func(a, b int) bool {
		return positions[idx[a]] < positions[idx[b]]
	})

	type Robot struct {
		index  int
		health int
		dir    byte
	}
	stack := make([]Robot, 0)

	for _, i := range idx {
		curIdx := i
		curHp := healths[i]
		curDir := directions[i]

		for len(stack) > 0 {
			prev := stack[len(stack)-1]
			if prev.dir == 'R' && curDir == 'L' {
				stack = stack[:len(stack)-1]
				if prev.health > curHp {
					curIdx = prev.index
					curHp = prev.health - 1
					curDir = prev.dir
				} else if prev.health < curHp {
					curHp -= 1
				} else {
					curIdx = -1
					break
				}
			} else {
				break
			}
		}

		if curIdx != -1 {
			stack = append(stack, Robot{curIdx, curHp, curDir})
		}
	}

	sort.Slice(stack, func(a, b int) bool {
		return stack[a].index < stack[b].index
	})

	ans := make([]int, len(stack))
	for i, r := range stack {
		ans[i] = r.health
	}
	return ans
}

func main() {
	tests := []struct {
		positions  []int
		healths    []int
		directions string
		want       []int
	}{
		{[]int{5, 4, 3, 2, 1}, []int{2, 17, 9, 15, 10}, "RRRRR", []int{2, 17, 9, 15, 10}},
		{[]int{3, 5, 2, 6}, []int{10, 10, 15, 12}, "RLRL", []int{14}},
		{[]int{1, 2, 5, 6}, []int{10, 10, 11, 11}, "RLRL", []int{}},
	}

	for _, tt := range tests {
		got := survivedRobotsHealths(tt.positions, tt.healths, tt.directions)
		if !reflect.DeepEqual(got, tt.want) {
			fmt.Printf("survivedRobotsHealths(%v, %v, %v) = %v, want %v\n", tt.positions, tt.healths, tt.directions, got, tt.want)
		}
	}
}
