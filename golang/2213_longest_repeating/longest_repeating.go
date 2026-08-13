// Package main ...
package main

import (
	"fmt"
	"reflect"

	"github.com/emirpasic/gods/v2/maps/treemap"
)

func longestRepeating(s string, queryCharacters string, queryIndices []int) []int {
	n := len(s)
	arr := []byte(s)
	segs := treemap.New[int, int]()
	lens := treemap.New[int, int]()

	for i := 0; i < n; {
		j := i
		for j < n && arr[j] == arr[i] {
			j++
		}
		segs.Put(i, j-1)
		cnt, _ := lens.Get(j - i)
		lens.Put(j-i, cnt+1)
		i = j
	}

	k := len(queryIndices)
	ans := make([]int, k)

	for q := 0; q < k; q++ {
		pos := queryIndices[q]
		ch := queryCharacters[q]

		if arr[pos] != ch {
			L, _, _ := segs.Floor(pos)
			R, _ := segs.Get(L)
			segs.Remove(L)
			oldLen := R - L + 1
			cnt, _ := lens.Get(oldLen)
			if cnt == 1 {
				lens.Remove(oldLen)
			} else {
				lens.Put(oldLen, cnt-1)
			}

			if L <= pos-1 {
				segs.Put(L, pos-1)
				c, _ := lens.Get(pos - L)
				lens.Put(pos-L, c+1)
			}
			if pos+1 <= R {
				segs.Put(pos+1, R)
				c, _ := lens.Get(R - pos)
				lens.Put(R-pos, c+1)
			}

			newL, newR := pos, pos

			rightKey, _, rightFound := segs.Ceiling(pos + 1)
			if rightFound && rightKey == pos+1 && pos+1 < n && arr[pos+1] == ch {
				rightR, _ := segs.Get(rightKey)
				rightLen := rightR - rightKey + 1
				c, _ := lens.Get(rightLen)
				if c == 1 {
					lens.Remove(rightLen)
				} else {
					lens.Put(rightLen, c-1)
				}
				newR = rightR
				segs.Remove(rightKey)
			}

			leftKey, _, leftFound := segs.Floor(pos - 1)
			if leftFound {
				leftR, _ := segs.Get(leftKey)
				if leftR == pos-1 && arr[pos-1] == ch {
					leftLen := leftR - leftKey + 1
					c, _ := lens.Get(leftLen)
					if c == 1 {
						lens.Remove(leftLen)
					} else {
						lens.Put(leftLen, c-1)
					}
					newL = leftKey
					segs.Remove(leftKey)
				}
			}

			segs.Put(newL, newR)
			c, _ := lens.Get(newR - newL + 1)
			lens.Put(newR-newL+1, c+1)
			arr[pos] = ch
		}

		maxKey, _, _ := lens.Max()
		ans[q] = maxKey
	}

	return ans
}

func main() {
	tests := []struct {
		s               string
		queryCharacters string
		queryIndices    []int
		ans             []int
	}{
		{"babacc", "bcb", []int{1, 3, 3}, []int{3, 3, 4}},
		{"abyzz", "aa", []int{2, 1}, []int{2, 3}},
	}

	for index, test := range tests {
		if !reflect.DeepEqual(longestRepeating(test.s, test.queryCharacters, test.queryIndices), test.ans) {
			fmt.Println(index)
		}
	}
}
