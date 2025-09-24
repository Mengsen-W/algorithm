// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type Allocator struct {
	n      int
	memory []int
}

func Constructor(n int) Allocator {
	return Allocator{
		n:      n,
		memory: make([]int, n),
	}
}

func (a *Allocator) Allocate(size int, mID int) int {
	count := 0
	for i := 0; i < a.n; i++ {
		if a.memory[i] != 0 {
			count = 0
		} else {
			count++
			if count == size {
				for j := i - count + 1; j <= i; j++ {
					a.memory[j] = mID
				}
				return i - count + 1
			}
		}
	}
	return -1
}

func (a *Allocator) FreeMemory(mID int) int {
	count := 0
	for i := 0; i < a.n; i++ {
		if a.memory[i] == mID {
			count++
			a.memory[i] = 0
		}
	}
	return count
}

func main() {
	loc := Constructor(10)
	assert.Equal(&testing.T{}, loc.Allocate(1, 1), 0)
	assert.Equal(&testing.T{}, loc.Allocate(1, 2), 1)
	assert.Equal(&testing.T{}, loc.Allocate(1, 3), 2)
	assert.Equal(&testing.T{}, loc.FreeMemory(2), 1)
	assert.Equal(&testing.T{}, loc.Allocate(3, 4), 3)
	assert.Equal(&testing.T{}, loc.Allocate(1, 1), 1)
	assert.Equal(&testing.T{}, loc.Allocate(1, 1), 6)
	assert.Equal(&testing.T{}, loc.FreeMemory(1), 3)
	assert.Equal(&testing.T{}, loc.Allocate(10, 2), -1)
	assert.Equal(&testing.T{}, loc.FreeMemory(7), 0)
}
