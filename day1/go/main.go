package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	fmt.Printf("DISTANCE: %d\n", partOne("../input.txt"))
	fmt.Printf("SIMILARITY_SCORE: %d\n", partTwo("../input.txt"))
}

func partOne(path string) int {
	firstList, secondList := parse(path)

	slices.Sort(firstList)
	slices.Sort(secondList)

	distances := []float64{}
	for i, first := range firstList {
		second := secondList[i]
		distances = append(distances, math.Abs(float64(first-second)))
	}

	var result int
	for _, d := range distances {
		result += int(d)
	}

	return result
}

func partTwo(path string) int {
	firstList, secondList := parse(path)

	similarityScore := 0
	secondListMap := make(map[int]int)

	for _, second := range secondList {
		secondListMap[second]++
	}

	for _, first := range firstList {
		similarityScore += first * secondListMap[first]
	}

	return similarityScore
}

func parse(path string) ([]int, []int) {
	file, err := os.Open(path)
	handleErr(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	firstList := make([]int, 0)
	secondList := make([]int, 0)

	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Fields(line)
		first, err := strconv.Atoi(split[0])
		handleErr(err)
		firstList = append(firstList, first)

		second, err := strconv.Atoi(split[1])
		handleErr(err)
		secondList = append(secondList, second)
	}

	return firstList, secondList
}

func handleErr(e error) {
	if e != nil {
		panic(e)
	}
}
