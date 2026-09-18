// Package main ...
package main

import "fmt"

func checkOverlap(radius int, xCenter int, yCenter int, x1 int, y1 int, x2 int, y2 int) bool {
	min := func(a int, b int) int {
		if a < b {
			return a
		}
		return b
	}
	dist := 0
	if xCenter < x1 || xCenter > x2 {
		dist += min((x1-xCenter)*(x1-xCenter), (x2-xCenter)*(x2-xCenter))
	}
	if yCenter < y1 || yCenter > y2 {
		dist += min((y1-yCenter)*(y1-yCenter), (y2-yCenter)*(y2-yCenter))
	}
	return dist <= radius*radius
}

func main() {
	tests := []struct {
		radius  int
		xCenter int
		yCenter int
		x1      int
		y1      int
		x2      int
		y2      int
		ans     bool
	}{
		{1, 0, 0, 1, -1, 3, 1, true},
		{1, 1, 1, 1, -3, 2, -1, false},
		{1, 0, 0, -1, 0, 0, 1, true},
	}

	for index, test := range tests {
		if checkOverlap(test.radius, test.xCenter, test.yCenter, test.x1, test.y1, test.x2, test.y2) != test.ans {
			fmt.Println(index)
		}
	}
}
