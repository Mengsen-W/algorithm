// Package main ...
package main

import (
	"fmt"
	"reflect"
	"sort"
)

func maxNumOfSubstrings(s string) []string {
	// 记录每个字符的第一次和最后一次出现位置
	pos := make(map[byte][]int)

	for i := 0; i < len(s); i++ {
		ch := s[i]
		if _, exists := pos[ch]; !exists {
			pos[ch] = []int{i, i}
		} else {
			pos[ch][1] = i
		}
	}

	// 所有合法的区间
	type Interval struct {
		left, right int
	}
	valid := []Interval{}

	for _, range_ := range pos {
		l, r := range_[0], range_[1]
		nl, nr := l, l

		for nl >= l || nr <= r {
			i := nl
			if nl < l {
				i = nr
			}

			// 当前处理的是字符 s[i]
			l_t := pos[s[i]][0]
			r_t := pos[s[i]][1]

			// 当前区间左侧还有该字符，需要向左扩展
			if l_t < l {
				l = l_t
			}

			// 当前区间右侧还有该字符，需要向右扩展
			if r_t > r {
				r = r_t
			}

			// 当前处理的是左指针
			if i == nl {
				nl--
			}

			// 当前处理的是右指针
			if i == nr {
				nr++
			}
		}

		valid = append(valid, Interval{l, r})
	}

	// 按右端点升序排序
	sort.Slice(valid, func(i, j int) bool {
		return valid[i].right < valid[j].right
	})

	// 贪心选择互不重叠的区间
	ans := []string{}
	end := -1

	for _, interval := range valid {
		if interval.left > end {
			ans = append(ans, s[interval.left:interval.right+1])
			end = interval.right
		}
	}

	return ans
}

func main() {
	tests := []struct {
		s   string
		ans []string
	}{
		{"adefaddaccc", []string{"e", "f", "ccc"}},
		{"abbaccd", []string{"d", "bb", "cc"}},
	}

	for index, test := range tests {
		if !reflect.DeepEqual(maxNumOfSubstrings(test.s), test.ans) {
			fmt.Println(index)
		}
	}
}
