package main

import "testing"

func TestPartOne(t *testing.T) {
	safe_reports_count := partOne("../input-test.txt")
	if safe_reports_count != 2 {
		t.Fatalf("Expected: 2, got: %d\n", safe_reports_count)
	}
}
