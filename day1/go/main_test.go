package main

import "testing"

func TestPartOne(t *testing.T) {
	distance := partOne("../input-test.txt")
	if distance != 11 {
		t.Fatalf("Expected: 11, got: %d\n", distance)
	}
}

func TestPartTwo(t *testing.T) {
	similarityScore := partTwo("../input-test.txt")
	if similarityScore != 31 {
		t.Fatalf("Expected: 31, got: %d\n", similarityScore)
	}
}
