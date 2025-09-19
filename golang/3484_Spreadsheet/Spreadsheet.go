// Package main ...
package main

import (
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

type Spreadsheet struct {
	grid [][]int
}

func Constructor(rows int) Spreadsheet {
	grid := make([][]int, rows+1)
	for i := range grid {
		grid[i] = make([]int, 27)
	}
	return Spreadsheet{grid: grid}
}

func (s *Spreadsheet) SetCell(cell string, value int) {
	x, y := s.getPos(cell)
	s.grid[x][y] = value
}

func (s *Spreadsheet) ResetCell(cell string) {
	x, y := s.getPos(cell)
	s.grid[x][y] = 0
}

func (s *Spreadsheet) GetValue(formula string) int {
	i := strings.Index(formula, "+")
	cell1 := formula[1:i]
	cell2 := formula[i+1:]
	return s.getCellVal(cell1) + s.getCellVal(cell2)
}

func (s *Spreadsheet) getPos(cell string) (int, int) {
	x, _ := strconv.Atoi(cell[1:])
	y := int(cell[0] - 'A')
	return x, y
}

func (s *Spreadsheet) getCellVal(cell string) int {
	if cell[0] >= 'A' && cell[0] <= 'Z' {
		x, y := s.getPos(cell)
		return s.grid[x][y]
	}
	val, _ := strconv.Atoi(cell)
	return val
}

func main() {
	spreadsheet := Constructor(3)
	assert.Equal(&testing.T{}, 12, spreadsheet.GetValue("=5+7"))
	spreadsheet.SetCell("A1", 10)
	assert.Equal(&testing.T{}, 16, spreadsheet.GetValue("=A1+6"))
	spreadsheet.SetCell("B2", 15)
	assert.Equal(&testing.T{}, 25, spreadsheet.GetValue("=A1+B2"))
	spreadsheet.ResetCell("A1")
	assert.Equal(&testing.T{}, 15, spreadsheet.GetValue("=A1+B2"))
}
