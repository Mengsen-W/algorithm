/*
 * @Date: 2023-12-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-16
 * @FilePath: /algorithm/golang/2276_CountIntervals/CountIntervals.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/emirpasic/gods/maps/treemap"
	"github.com/stretchr/testify/assert"
)

type CountIntervals struct {
	*treemap.Map
	cnt int
}

func Constructor() CountIntervals {
	return CountIntervals{
		treemap.NewWithIntComparator(), 0,
	}
}

func (c *CountIntervals) Add(left int, right int) {
	for k, v := c.Floor(right); k != nil && v.(int) >= left; k, v = c.Floor(right) {
		l, r := k.(int), v.(int)
		left, right = min(left, l), max(right, r)
		c.cnt -= r - l + 1
		c.Remove(k)
	}
	c.cnt += right - left + 1
	c.Put(left, right)
}

func (c *CountIntervals) Count() int {
	return c.cnt
}

func main() {
	countIntervals := Constructor()
	countIntervals.Add(2, 3)
	countIntervals.Add(7, 10)
	assert.Equal(&testing.T{}, countIntervals.Count(), 6)
	countIntervals.Add(5, 8)
	assert.Equal(&testing.T{}, countIntervals.Count(), 8)
}
