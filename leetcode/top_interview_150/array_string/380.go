package arraystring

import (
	"math/rand"
)

type RandomizedSet struct {
	// key is the value, value is index of nums slice.
	exist map[int]int
	size  int
	nums  []int
}

func Constructor() RandomizedSet {
	return RandomizedSet{
		exist: map[int]int{},
		size:  0,
		nums:  []int{},
	}
}

func (this *RandomizedSet) Insert(val int) bool {
	_, ok := this.exist[val]
	if ok {
		return false
	}
	this.exist[val] = this.size
	this.size++
	this.nums = append(this.nums, val)
	return true
}

func (this *RandomizedSet) Remove(val int) bool {
	idx, ok := this.exist[val]
	if !ok {
		return false
	}

	// swap the element to be removed with the last element
	this.nums[idx], this.nums[this.size-1] = this.nums[this.size-1], this.nums[idx]

	// update the index
	swapped := this.nums[idx]
	this.exist[swapped] = idx

	// delte the last element
	this.nums = this.nums[:this.size-1]

	this.size--
	delete(this.exist, val)
	return true
}

func (this *RandomizedSet) GetRandom() int {
	idx := rand.Intn(this.size)
	return this.nums[idx]
}
