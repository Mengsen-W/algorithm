// Package main ,..
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for Employee.
type Employee struct {
	Id           int
	Importance   int
	Subordinates []int
}

func getImportance(employees []*Employee, id int) (total int) {
	mp := map[int]*Employee{}
	for _, employee := range employees {
		mp[employee.Id] = employee
	}

	var dfs func(int)
	dfs = func(id int) {
		employee := mp[id]
		total += employee.Importance
		for _, subID := range employee.Subordinates {
			dfs(subID)
		}
	}
	dfs(id)
	return
}

func main() {
	tests := []struct {
		employees []*Employee
		id        int
		total     int
	}{
		{[]*Employee{{1, 5, []int{2, 3}}, {2, 3, []int{}}, {3, 3, []int{}}}, 1, 11},
		{[]*Employee{{1, 2, []int{5}}, {5, -3, []int{}}}, 5, -3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.total, getImportance(test.employees, test.id), index)
	}
}
