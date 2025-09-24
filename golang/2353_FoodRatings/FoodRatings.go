// Package main ...
package main

import (
	"container/heap"
	"testing"

	"github.com/stretchr/testify/assert"
)

type FoodRatings struct {
	foodMap map[string]struct {
		rating  int
		cuisine string
	}
	ratingMap map[string]*PriorityQueue
	n         int
}

func Constructor(foods []string, cuisines []string, ratings []int) FoodRatings {
	n := len(foods)
	foodMap := make(map[string]struct {
		rating  int
		cuisine string
	})
	ratingMap := make(map[string]*PriorityQueue)

	for i := 0; i < n; i++ {
		food := foods[i]
		cuisine := cuisines[i]
		rating := ratings[i]
		foodMap[food] = struct {
			rating  int
			cuisine string
		}{rating, cuisine}
		if ratingMap[cuisine] == nil {
			ratingMap[cuisine] = &PriorityQueue{}
		}
		heap.Push(ratingMap[cuisine], Pair{n - rating, food})
	}

	return FoodRatings{
		foodMap:   foodMap,
		ratingMap: ratingMap,
		n:         n,
	}
}

func (this *FoodRatings) ChangeRating(food string, newRating int) {
	entry := this.foodMap[food]
	cuisine := entry.cuisine
	heap.Push(this.ratingMap[cuisine], Pair{this.n - newRating, food})
	entry.rating = newRating
	this.foodMap[food] = entry
}

func (this *FoodRatings) HighestRated(cuisine string) string {
	pq := this.ratingMap[cuisine]
	for pq.Len() > 0 {
		top := (*pq)[0]
		if this.n-top.rating == this.foodMap[top.food].rating {
			return top.food
		}
		heap.Pop(pq)
	}
	return ""
}

type Pair struct {
	rating int
	food   string
}

type PriorityQueue []Pair

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	if pq[i].rating == pq[j].rating {
		return pq[i].food < pq[j].food
	}
	return pq[i].rating < pq[j].rating
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(Pair))
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

func main() {
	foods := []string{"kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"}
	cuisines := []string{"korean", "japanese", "japanese", "greek", "japanese", "korean"}
	ratings := []int{9, 12, 8, 15, 14, 7}

	foodRatings := Constructor(foods, cuisines, ratings)
	assert.Equal(&testing.T{}, "kimchi", foodRatings.HighestRated("korean"))
	assert.Equal(&testing.T{}, foodRatings.HighestRated("japanese"), "ramen")
	foodRatings.ChangeRating("sushi", 16)
	assert.Equal(&testing.T{}, foodRatings.HighestRated("japanese"), "sushi")
	foodRatings.ChangeRating("ramen", 16)
	assert.Equal(&testing.T{}, foodRatings.HighestRated("japanese"), "ramen")
}
