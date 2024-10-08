package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func destCity(paths [][]string) string {
	citiesA := map[string]bool{}
	for _, path := range paths {
		citiesA[path[0]] = true
	}
	for _, path := range paths {
		if !citiesA[path[1]] {
			return path[1]
		}
	}
	return ""
}

func main() {
	tests := []struct {
		paths [][]string
		ans   string
	}{}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, destCity(test.paths), index)
	}
}
