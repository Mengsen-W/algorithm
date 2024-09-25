// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func distinctNames(ideas []string) int64 {
	names := make(map[byte]map[string]struct{})
	for _, idea := range ideas {
		if _, exists := names[idea[0]]; !exists {
			names[idea[0]] = make(map[string]struct{})
		}
		names[idea[0]][idea[1:]] = struct{}{}
	}

	getIntersectSize := func(a, b map[string]struct{}) int {
		count := 0
		for key := range a {
			if _, found := b[key]; found {
				count++
			}
		}
		return count
	}

	var ans int64
	for preA, setA := range names {
		for preB, setB := range names {
			if preA == preB {
				continue
			}
			intersect := getIntersectSize(setA, setB)
			ans += int64(len(setA)-intersect) * int64(len(setB)-intersect)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		ideas []string
		ans   int64
	}{
		{[]string{"coffee", "donuts", "time", "toffee"}, 6},
		{[]string{"lack", "back"}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distinctNames(test.ideas), index)
	}
}
