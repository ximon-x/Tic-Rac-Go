// Test the Checkwin Function

package pkg

import (
	"testing"
)

func TestCheckWinEmpty(t *testing.T) {
	board := [][]int{
		{0, 0, 0},
		{0, 0, 0},
		{0, 0, 0},
	}

	if checkWin(board) {
		t.Errorf("Expected false, got true for 3x3")
	}
}

func TestCheckWinHorizontal(t *testing.T) {
	board := [][]int{
		{1, 1, 1},
		{0, 0, 0},
		{0, 0, 0},
	}

	if !checkWin(board) {
		t.Errorf("Expected true, got false for 3x3")
	}

	board = [][]int{
		{1, 1, 2, 1},
		{0, 0, 0, 0},
		{0, 0, 0, 0},
		{2, 2, 2, 2},
	}

	if !checkWin(board) {
		t.Errorf("Expected true, got false for 4x4")
	}
}

func TestCheckWinVertical(t *testing.T) {
	board := [][]int{
		{1, 0, 0},
		{1, 0, 0},
		{1, 0, 0},
	}

	if !checkWin(board) {
		t.Errorf("Expected true, got false for 3x3")
	}

	board = [][]int{
		{1, 0, 0, 2},
		{2, 1, 0, 2},
		{1, 0, 2, 2},
		{1, 0, 0, 2},
	}

	if !checkWin(board) {
		t.Errorf("Expected true, got false for 4x4")
	}
}

func TestCheckWinDiagonal(t *testing.T) {
	board := [][]int{
		{1, 0, 1},
		{0, 0, 0},
		{0, 0, 0},
	}

	if checkWin(board) {
		t.Errorf("Expected false, got true for 3x3")
	}

	board = [][]int{
		{1, 0, 0},
		{0, 1, 0},
		{0, 0, 1},
	}

	if !checkWin(board) {
		t.Errorf("Expected true, got false for 3x3")
	}

	board = [][]int{
		{1, 0, 0, 2},
		{2, 1, 0, 2},
		{1, 0, 1, 2},
		{1, 0, 0, 1},
	}

	if !checkWin(board) {
		t.Errorf("Expected true, got false for 4x4")
	}
}
