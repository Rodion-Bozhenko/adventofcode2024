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
	file, err := os.Open("../input.txt")
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

	fmt.Println(result)
}

func handleErr(e error) {
	panic(e)
}
