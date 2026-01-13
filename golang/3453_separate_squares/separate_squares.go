// Package main ...
package main

import (
	"math"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func separateSquares(squares [][]int) float64 {
	var totalArea int64 = 0
	type Event struct {
		y     int
		l     int
		delta int
	}
	events := make([]Event, 0, len(squares)*2)

	for _, sq := range squares {
		y, l := sq[1], sq[2]
		totalArea += int64(l) * int64(l)
		events = append(events, Event{y, l, 1})
		events = append(events, Event{y + l, l, -1})
	}

	// 按y坐标排序
	sort.Slice(events, func(i, j int) bool {
		return events[i].y < events[j].y
	})

	coveredWidth := 0.0 // 当前扫描线下所有底边之和
	currArea := 0.0     // 当前累计面积
	prevHeight := 0.0   // 前一个扫描线的高度

	for _, event := range events {
		y, l, delta := event.y, event.l, event.delta
		diff := float64(y) - prevHeight
		// 两条扫描线之间新增的面积
		area := coveredWidth * diff
		// 如果加上这部分面积超过总面积的一半
		if 2.0*(currArea+area) >= float64(totalArea) {
			return prevHeight + (float64(totalArea)-2.0*currArea)/(2.0*coveredWidth)
		}
		// 更新宽度：开始事件加宽度，结束事件减宽度
		coveredWidth += float64(delta * l)
		currArea += area
		prevHeight = float64(y)
	}

	return 0.0
}

func main() {
	tests := []struct {
		squares [][]int
		ans     float64
	}{
		{[][]int{{0, 0, 1}, {2, 2, 1}}, 1.0},
		{[][]int{{0, 0, 2}, {1, 1, 1}}, 1.16667},
	}

	for index, test := range tests {
		assert.Truef(&testing.T{}, math.Abs(test.ans-separateSquares(test.squares)) < 1e-5, "test %d failed", index)
	}
}
