package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("SAFE_REPORTS: ", partOne("../input.txt"))
}

func partOne(path string) int {
	reports := parse(path)

	count := 0
	for _, report := range reports {
		if is_report_safe(report) {
			count++
		}
	}

	return count
}

func is_report_safe(report []int) bool {
	asc := report[1] > report[0]

	for i := 0; i < len(report)-1; i++ {
		diff := report[i] - report[i+1]

		if !asc && !slices.Contains([]int{1, 2, 3}, diff) {
			return false
		}
		if asc && !slices.Contains([]int{-3, -2, -1}, diff) {
			return false
		}
	}
	return true
}

func parse(path string) [][]int {
	input_file, err := os.Open(path)
	handleErr(err)
	scanner := bufio.NewScanner(input_file)

	reports := make([][]int, 0)
	for scanner.Scan() {
		reportStr := scanner.Text()
		levels := strings.Split(reportStr, " ")

		report := make([]int, 0)
		for _, level := range levels {
			parsedLevel, err := strconv.Atoi(level)
			handleErr(err)
			report = append(report, parsedLevel)
		}

		reports = append(reports, report)
	}

	return reports
}

func handleErr(e error) {
	if e != nil {
		panic(e)
	}
}
