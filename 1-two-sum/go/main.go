package main

import "fmt"

// O(n^2) Solution
func twoSum(nums []int, target int) []int {
    if len(nums) < 2 {
        return []int{-1, -1}
    }

    for i := 0; i < len(nums) - 1; i++ {
        for j := i + 1; j < len(nums); j++ {
            if nums[i] + nums[j] == target {
                return []int{i, j}
            }
        }
    }

    return []int{-1,-1}
}

func main() {
	// fmt.Println("Hello, world! ~ GO")

    nums := []int{2,7,11,15}
    target := 9
    fmt.Println("Result = ", twoSum(nums, target))
}
