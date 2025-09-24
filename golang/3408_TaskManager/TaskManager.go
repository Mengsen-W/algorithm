// Package main ...
package main

import (
	"container/heap"
	"testing"

	"github.com/stretchr/testify/assert"
)

type TaskManager struct {
	taskInfo map[int][2]int
	heap     *PriorityQueue
}

func Constructor(tasks [][]int) TaskManager {
	tm := TaskManager{
		taskInfo: make(map[int][2]int),
		heap:     &PriorityQueue{},
	}
	heap.Init(tm.heap)

	for _, task := range tasks {
		userID, taskID, priority := task[0], task[1], task[2]
		tm.taskInfo[taskID] = [2]int{priority, userID}
		heap.Push(tm.heap, Task{priority: priority, taskID: taskID})
	}
	return tm
}

func (t *TaskManager) Add(userID int, taskID int, priority int) {
	t.taskInfo[taskID] = [2]int{priority, userID}
	heap.Push(t.heap, Task{priority: priority, taskID: taskID})
}

func (t *TaskManager) Edit(taskID int, newPriority int) {
	if info, exists := t.taskInfo[taskID]; exists {
		info[0] = newPriority
		t.taskInfo[taskID] = info
		heap.Push(t.heap, Task{priority: newPriority, taskID: taskID})
	}
}

func (t *TaskManager) Rmv(taskID int) {
	delete(t.taskInfo, taskID)
}

func (t *TaskManager) ExecTop() int {
	for t.heap.Len() > 0 {
		task := heap.Pop(t.heap).(Task)
		priority, taskID := task.priority, task.taskID
		if info, exists := t.taskInfo[taskID]; exists && info[0] == priority {
			userID := info[1]
			delete(t.taskInfo, taskID)
			return userID
		}
	}
	return -1
}

type Task struct {
	priority int
	taskID   int
}

type PriorityQueue []Task

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	if pq[i].priority != pq[j].priority {
		return pq[i].priority > pq[j].priority
	}
	return pq[i].taskID > pq[j].taskID
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x any) {
	*pq = append(*pq, x.(Task))
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

func main() {
	taskManager := Constructor([][]int{{1, 101, 10}, {2, 102, 20}, {3, 103, 15}})
	taskManager.Add(4, 104, 5)
	taskManager.Edit(102, 8)
	assert.Equal(&testing.T{}, taskManager.ExecTop(), 3)
	taskManager.Rmv(101)
	taskManager.Add(5, 105, 15)
	assert.Equal(&testing.T{}, taskManager.ExecTop(), 5)
}
