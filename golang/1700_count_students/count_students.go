/*
 * @Date: 2022-10-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-19
 * @FilePath: /algorithm/1700_count_students/count_students.go
 */

package main

func countStudents(students, sandwiches []int) int {
	s1 := 0
	for _, v := range students {
		s1 += v
	}
	s0 := len(students) - s1
	for _, x := range sandwiches {
		if x == 0 && s0 > 0 {
			s0--
		} else if x == 1 && s1 > 0 {
			s1--
		} else {
			break
		}
	}
	return s0 + s1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		students := []int{1, 1, 0, 0}
		sandwiches := []int{0, 1, 0, 1}
		ans := 0
		assert(countStudents(students, sandwiches) == ans)
	}

	{
		students := []int{1, 1, 1, 0, 0, 1}
		sandwiches := []int{1, 0, 0, 0, 1, 1}
		ans := 3
		assert(countStudents(students, sandwiches) == ans)
	}
}
