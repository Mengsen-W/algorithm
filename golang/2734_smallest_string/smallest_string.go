// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestString(s string) string {
	t := []byte(s)
	indexOfFirstNonA := findFirstNonA(t)
	if indexOfFirstNonA == len(t) {
		t[len(t)-1] = 'z'
		return string(t)
	}
	indexOfFirstA_AfterFirstNonA := findFirstA_AfterFirstNonA(t, indexOfFirstNonA)
	res := []byte{}
	for i, ch := range t {
		if indexOfFirstNonA <= i && i < indexOfFirstA_AfterFirstNonA {
			res = append(res, ch-1)
		} else {
			res = append(res, ch)
		}
	}

	return string(res)
}

func findFirstNonA(t []byte) int {
	for i := 0; i < len(t); i++ {
		if t[i] != 'a' {
			return i
		}
	}
	return len(t)
}

func findFirstA_AfterFirstNonA(t []byte, firstNonA int) int {
	for i := firstNonA; i < len(t); i++ {
		if t[i] == 'a' {
			return i
		}
	}
	return len(t)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"cbabc", "baabc"},
		{"acbbc", "abaab"},
		{"leetcode", "kddsbncd"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestString(test.s), index)
	}
}
