// Package main ...
package main

import (
	"sort"
	"testing"
	"unicode"

	"github.com/stretchr/testify/assert"
)

func check(code string, isActive bool) bool {
	for _, c := range code {
		if c != '_' && !unicode.IsLetter(c) && !unicode.IsDigit(c) {
			return false
		}
	}
	return isActive
}

func validateCoupons(code []string, businessLine []string, isActive []bool) []string {
	groups := make([][]string, 4)
	for i := range groups {
		groups[i] = make([]string, 0)
	}

	ans := make([]string, 0)

	for i := 0; i < len(code); i++ {
		if code[i] != "" && check(code[i], isActive[i]) {
			switch businessLine[i] {
			case "electronics":
				groups[0] = append(groups[0], code[i])
			case "grocery":
				groups[1] = append(groups[1], code[i])
			case "pharmacy":
				groups[2] = append(groups[2], code[i])
			case "restaurant":
				groups[3] = append(groups[3], code[i])
			}
		}
	}

	for _, group := range groups {
		sort.Strings(group)
		ans = append(ans, group...)
	}

	return ans
}

func main() {
	tests := []struct {
		code         []string
		businessLine []string
		isActive     []bool
		ans          []string
	}{
		{
			[]string{"SAVE20", "", "PHARMA5", "SAVE@20"},
			[]string{"restaurant", "grocery", "pharmacy", "restaurant"},
			[]bool{true, true, true, true},
			[]string{"PHARMA5", "SAVE20"},
		},
		{
			[]string{"GROCERY15", "ELECTRONICS_50", "DISCOUNT10"},
			[]string{"grocery", "electronics", "invalid"},
			[]bool{false, true, true},
			[]string{"ELECTRONICS_50"},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, validateCoupons(test.code, test.businessLine, test.isActive), test.ans, "test %d", index)
	}
}
