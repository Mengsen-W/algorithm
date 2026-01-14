// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

type SegmentTree struct {
	count   []int
	covered []int
	xs      []int
	n       int
}

func NewSegmentTree(xs []int) *SegmentTree {
	n := len(xs) - 1
	return &SegmentTree{
		count:   make([]int, 4*n),
		covered: make([]int, 4*n),
		xs:      xs,
		n:       n,
	}
}

func (st *SegmentTree) modify(qleft, qright, qval, left, right, pos int) {
	if st.xs[right+1] <= qleft || st.xs[left] >= qright {
		return
	}
	if qleft <= st.xs[left] && st.xs[right+1] <= qright {
		st.count[pos] += qval
	} else {
		mid := (left + right) / 2
		st.modify(qleft, qright, qval, left, mid, pos*2+1)
		st.modify(qleft, qright, qval, mid+1, right, pos*2+2)
	}

	if st.count[pos] > 0 {
		st.covered[pos] = st.xs[right+1] - st.xs[left]
	} else {
		if left == right {
			st.covered[pos] = 0
		} else {
			st.covered[pos] = st.covered[pos*2+1] + st.covered[pos*2+2]
		}
	}
}

func (st *SegmentTree) Update(qleft, qright, qval int) {
	st.modify(qleft, qright, qval, 0, st.n-1, 0)
}

func (st *SegmentTree) Query() int {
	return st.covered[0]
}

func separateSquares(squares [][]int) float64 {
	// 存储事件: (y坐标, 类型, 左边界, 右边界)
	type Event struct {
		y, delta, xl, xr int
	}
	events := []Event{}
	xsSet := make(map[int]bool)

	for _, sq := range squares {
		x, y, l := sq[0], sq[1], sq[2]
		xr := x + l
		events = append(events, Event{y, 1, x, xr})
		events = append(events, Event{y + l, -1, x, xr})
		xsSet[x] = true
		xsSet[xr] = true
	}

	// 按y坐标排序事件
	sort.Slice(events, func(i, j int) bool {
		return events[i].y < events[j].y
	})

	// 离散化坐标
	xs := make([]int, 0, len(xsSet))
	for x := range xsSet {
		xs = append(xs, x)
	}
	sort.Ints(xs)

	// 初始化线段树
	segTree := NewSegmentTree(xs)

	psum := []float64{}
	widths := []int{}
	totalArea := 0.0
	prev := events[0].y

	// 扫描：计算总面积和记录中间状态
	for _, event := range events {
		y, delta, xl, xr := event.y, event.delta, event.xl, event.xr
		length := segTree.Query()
		totalArea += float64(length) * float64(y-prev)
		segTree.Update(xl, xr, delta)
		// 记录前缀和和宽度
		psum = append(psum, totalArea)
		widths = append(widths, segTree.Query())
		prev = y
	}

	// 计算目标面积（向上取整的一半）
	target := int64(totalArea+1) / 2
	// 二分查找第一个大于等于target的位置
	i := sort.Search(len(psum), func(i int) bool {
		return psum[i] >= float64(target)
	})
	i--

	// 获取对应的面积、宽度和高度
	area := psum[i]
	width := widths[i]
	height := events[i].y

	return float64(height) + (totalArea-area*2)/(float64(width)*2.0)
}

func main() {
	tests := []struct {
		squares [][]int
		want    float64
	}{
		{[][]int{{0, 0, 1}, {2, 2, 1}}, 1.00},
		{[][]int{{0, 0, 2}, {1, 1, 1}}, 1.00},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.want, separateSquares(test.squares), index)
	}
}
