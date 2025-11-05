// Package main ...
package main

import (
	"math"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

// Element 存储元素值和对应的出现次数
type Element struct {
	val int // 元素数值
	cnt int // 出现次数
}

func findXSum(nums []int, k int, x int) []int {
	n := len(nums)
	ansLen := n - k + 1
	ans := make([]int, ansLen)

	// 1. 初始化三个核心数据结构（必须全程同步）
	// freq: 元素值→当前频率（O(1)获取频率）
	freq := make(map[int]int)
	// 统计第一个窗口（0~k-1）的频率
	for i := 0; i < k; i++ {
		freq[nums[i]]++
	}
	// sortedElements: 按「频率降序→数值降序」排序的元素切片（核心有序结构）
	sortedElements := initSortedElements(freq)
	// valToIdx: 元素值→sortedElements中的索引（O(1)查找元素位置）
	valToIdx := buildValToIdxMap(sortedElements)

	// 2. 计算第一个窗口的x-sum
	ans[0] = calculateXSum(sortedElements, x)

	// 3. 滑动窗口处理剩余子数组（左删右加，同步更新三个数据结构）
	for i := 1; i < ansLen; i++ {
		leftVal := nums[i-1]    // 离开窗口的左侧元素
		rightVal := nums[i+k-1] // 进入窗口的右侧元素

		// 处理左删：元素离开窗口，更新频率→有序切片→索引映射
		sortedElements, valToIdx, freq = handleLeftElement(sortedElements, valToIdx, freq, leftVal)
		// 处理右加：元素进入窗口，更新频率→有序切片→索引映射
		sortedElements, valToIdx, freq = handleRightElement(sortedElements, valToIdx, freq, rightVal)

		// 计算当前窗口的x-sum
		ans[i] = calculateXSum(sortedElements, x)
	}

	return ans
}

// initSortedElements 基于频率字典初始化有序切片（快排，O(m log m)，m为不同元素个数）
func initSortedElements(freq map[int]int) []Element {
	var elements []Element
	for val, cnt := range freq {
		elements = append(elements, Element{val: val, cnt: cnt})
	}

	// 排序规则：频率降序 → 数值降序（核心规则，所有调整都要遵循）
	sort.Slice(elements, func(i, j int) bool {
		if elements[i].cnt != elements[j].cnt {
			return elements[i].cnt > elements[j].cnt
		}
		return elements[i].val > elements[j].val
	})
	return elements
}

// buildValToIdxMap 构建「元素值→切片索引」的映射（O(m)）
func buildValToIdxMap(elements []Element) map[int]int {
	valToIdx := make(map[int]int, len(elements))
	for idx, elem := range elements {
		valToIdx[elem.val] = idx
	}
	return valToIdx
}

// calculateXSum 基于有序切片计算x-sum（O(min(x,m))）
func calculateXSum(sortedElements []Element, x int) int {
	take := int(math.Min(float64(x), float64(len(sortedElements))))
	sum := 0
	for i := 0; i < take; i++ {
		sum += sortedElements[i].val * sortedElements[i].cnt
	}
	return sum
}

// handleLeftElement 处理离开窗口的左侧元素（左删）
// 返回更新后的sortedElements、valToIdx、freq（确保三者同步）
func handleLeftElement(
	sortedElements []Element,
	valToIdx map[int]int,
	freq map[int]int,
	leftVal int,
) ([]Element, map[int]int, map[int]int) {
	// O(1) 查找元素在切片中的索引（核心优势）
	idx, exists := valToIdx[leftVal]
	if !exists {
		return sortedElements, valToIdx, freq // 理论上不会出现，防御性处理
	}

	// 1. 更新频率（先更新freq，再更新切片元素的cnt）
	freq[leftVal]--
	sortedElements[idx].cnt--

	// 2. 频率为0：从三个数据结构中彻底删除该元素
	if sortedElements[idx].cnt == 0 {
		delete(freq, leftVal)
		delete(valToIdx, leftVal)
		// 从切片中删除元素（切片长度减1）
		sortedElements = append(sortedElements[:idx], sortedElements[idx+1:]...)
		// 关键：更新删除位置后所有元素的索引映射（因为它们的位置都左移了1）
		for i := idx; i < len(sortedElements); i++ {
			valToIdx[sortedElements[i].val] = i
		}
		return sortedElements, valToIdx, freq
	}

	// 3. 频率>0：元素需要下沉（保持切片有序），同步更新映射
	sortedElements, valToIdx = bubbleDown(sortedElements, valToIdx, idx)
	return sortedElements, valToIdx, freq
}

// handleRightElement 处理进入窗口的右侧元素（右加）
// 返回更新后的sortedElements、valToIdx、freq（确保三者同步）
func handleRightElement(
	sortedElements []Element,
	valToIdx map[int]int,
	freq map[int]int,
	rightVal int,
) ([]Element, map[int]int, map[int]int) {
	// O(1) 查找元素是否已存在
	idx, exists := valToIdx[rightVal]
	if exists {
		// 1. 元素已存在：更新频率，元素需要上浮（保持切片有序）
		freq[rightVal]++
		sortedElements[idx].cnt++
		sortedElements, valToIdx = bubbleUp(sortedElements, valToIdx, idx)
		return sortedElements, valToIdx, freq
	}

	// 2. 元素不存在：新增元素到三个数据结构
	newElem := Element{val: rightVal, cnt: 1}
	freq[rightVal] = 1
	// 插入到切片的合适位置（保持有序），并同步更新映射
	sortedElements, valToIdx = insertNewElement(sortedElements, valToIdx, newElem)
	return sortedElements, valToIdx, freq
}

// compare 比较两个元素的优先级：a是否应该排在b前面（遵循核心排序规则）
func compare(a, b Element) bool {
	if a.cnt != b.cnt {
		return a.cnt > b.cnt // 频率高的优先
	}
	return a.val > b.val // 频率相同，数值大的优先
}

// bubbleUp 元素频率增加后上浮（向前移动到合适位置），同步更新valToIdx
func bubbleUp(elements []Element, valToIdx map[int]int, idx int) ([]Element, map[int]int) {
	// 从当前位置向前比较，直到优先级不再更高
	for idx > 0 {
		prevIdx := idx - 1 // 前一个元素的索引
		// 当前元素优先级高于前一个，交换位置
		if compare(elements[idx], elements[prevIdx]) {
			// 交换切片中的元素
			elements[idx], elements[prevIdx] = elements[prevIdx], elements[idx]
			// 关键：同步更新valToIdx中两个交换元素的索引
			valToIdx[elements[idx].val] = idx
			valToIdx[elements[prevIdx].val] = prevIdx
			// 继续向前比较
			idx = prevIdx
		} else {
			break // 优先级不再更高，停止上浮
		}
	}
	return elements, valToIdx
}

// bubbleDown 元素频率减少后下沉（向后移动到合适位置），同步更新valToIdx
func bubbleDown(elements []Element, valToIdx map[int]int, idx int) ([]Element, map[int]int) {
	n := len(elements)
	// 从当前位置向后比较，直到优先级不再更低
	for idx < n-1 {
		nextIdx := idx + 1 // 后一个元素的索引
		// 当前元素优先级低于后一个，交换位置
		if compare(elements[nextIdx], elements[idx]) {
			// 交换切片中的元素
			elements[idx], elements[nextIdx] = elements[nextIdx], elements[idx]
			// 关键：同步更新valToIdx中两个交换元素的索引
			valToIdx[elements[idx].val] = idx
			valToIdx[elements[nextIdx].val] = nextIdx
			// 继续向后比较
			idx = nextIdx
		} else {
			break // 优先级不再更低，停止下沉
		}
	}
	return elements, valToIdx
}

// insertNewElement 插入新元素到有序切片的合适位置，同步更新valToIdx
func insertNewElement(elements []Element, valToIdx map[int]int, newElem Element) ([]Element, map[int]int) {
	n := len(elements)
	insertIdx := n // 默认插入到末尾（优先级最低）
	// 找到第一个优先级低于新元素的位置，插入到其前面
	for i := 0; i < n; i++ {
		if compare(newElem, elements[i]) {
			insertIdx = i
			break
		}
	}

	// 1. 插入新元素到切片
	elements = append(elements[:insertIdx], append([]Element{newElem}, elements[insertIdx:]...)...)
	// 2. 同步更新valToIdx：
	// a. 新元素的索引
	valToIdx[newElem.val] = insertIdx
	// b. 插入位置后所有元素的索引（因为它们的位置都右移了1）
	for i := insertIdx + 1; i < len(elements); i++ {
		valToIdx[elements[i].val] = i
	}

	return elements, valToIdx
}

func main() {
	tests := []struct {
		nums     []int
		k        int
		x        int
		expected []int
	}{
		{[]int{1, 1, 2, 2, 3, 4, 2, 3}, 6, 2, []int{6, 10, 12}},
		{[]int{3, 8, 7, 8, 7, 5}, 2, 2, []int{11, 15, 15, 15, 12}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, findXSum(test.nums, test.k, test.x), index)
	}
}
