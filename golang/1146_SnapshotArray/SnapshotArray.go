/*
 * @Date: 2024-04-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-26
 * @FilePath: /algorithm/golang/1146_SnapshotArray/SnapshotArray.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

type SnapshotArray struct {
	snapCnt int
	data    [][][2]int
}

func Constructor(length int) SnapshotArray {
	return SnapshotArray{
		snapCnt: 0,
		data:    make([][][2]int, length),
	}
}

func (s *SnapshotArray) Set(index int, val int) {
	s.data[index] = append(s.data[index], [2]int{s.snapCnt, val})
}

func (s *SnapshotArray) Snap() int {
	s.snapCnt++
	return s.snapCnt - 1
}

func (s *SnapshotArray) Get(index int, snapId int) int {
	x := sort.Search(len(s.data[index]), func(i int) bool {
		return s.data[index][i][0] > snapId
	})
	if x == 0 {
		return 0
	}
	return s.data[index][x-1][1]
}

func main() {
	snapshotArray := Constructor(3)
	snapshotArray.Set(0, 5)
	assert.Equal(&testing.T{}, snapshotArray.Snap(), 0)
	snapshotArray.Set(0, 6)
	assert.Equal(&testing.T{}, snapshotArray.Get(0, 0), 5)
}
