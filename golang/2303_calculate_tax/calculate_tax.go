/*
 * @Date: 2023-01-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-23
 * @FilePath: /algorithm/golang/2303_calculate_tax/calculate_tax.go
 */

package main

func calculateTax(brackets [][]int, income int) float64 {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	totalTax := 0
	lower := 0
	for _, bracket := range brackets {
		upper, percent := bracket[0], bracket[1]
		tax := (min(income, upper) - lower) * percent
		totalTax += tax
		if income <= upper {
			break
		}
		lower = upper
	}
	return float64(totalTax) / 100
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		brackets := [][]int{{3, 50}, {7, 10}, {12, 25}}
		income := 10
		ans := 2.65000000
		assert(calculateTax(brackets, income) == ans)
	}

	{
		brackets := [][]int{{1, 0}, {4, 25}, {5, 50}}
		income := 2
		ans := 0.25000000
		assert(calculateTax(brackets, income) == ans)
	}

	{
		brackets := [][]int{{2, 50}}
		income := 0
		ans := 0.000000
		assert(calculateTax(brackets, income) == ans)
	}
}
