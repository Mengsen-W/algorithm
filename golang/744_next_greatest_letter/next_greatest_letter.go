// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func nextGreatestLetter(letters []byte, target byte) byte {
	if target >= letters[len(letters)-1] {
		return letters[0]
	}
	i := sort.Search(len(letters)-1, func(i int) bool { return letters[i] > target })
	return letters[i]
}

func main() {
	tests := []struct {
		letters []byte
		target  byte
		expect  byte
	}{
		{[]byte{'c', 'f', 'j'}, 'a', 'c'},
		{[]byte{'c', 'f', 'j'}, 'c', 'f'},
		{[]byte{'x', 'x', 'y', 'y'}, 'z', 'x'},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expect, nextGreatestLetter(test.letters, test.target), index)
	}
}
