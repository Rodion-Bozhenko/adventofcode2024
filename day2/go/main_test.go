package main

import "testing"

func TestPartOne(t *testing.T) {
	safeReportsCount := partOne("../input-test.txt")
	if safeReportsCount != 2 {
		t.Fatalf("Expected: 2, got: %d\n", safeReportsCount)
	}
}

func TestPartTwo(t *testing.T) {
	safeReportsCount := partTwo("../input-test.txt")
	if safeReportsCount != 4 {
		t.Fatalf("Expected: 4, got: %d\n", safeReportsCount)
	}
}
