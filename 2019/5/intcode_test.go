package main

import (
	"reflect"
	"testing"
)

func Test1(t *testing.T) {
	verify(t, []int{1, 0, 0, 0, 99}, []int{2, 0, 0, 0, 99})
}

func Test2(t *testing.T) {
	verify(t, []int{2, 3, 0, 3, 99}, []int{2, 3, 0, 6, 99})
}

func Test3(t *testing.T) {
	verify(t, []int{2, 4, 4, 5, 99, 0}, []int{2, 4, 4, 5, 99, 9801})
}

func Test4(t *testing.T) {
	verify(t, []int{1, 1, 1, 4, 99, 5, 6, 0, 99}, []int{30, 1, 1, 4, 2, 5, 6, 0, 99})
}

func Test5(t *testing.T) {
	verify(t, []int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, []int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50})
}

func Test6(t *testing.T) {
	verify(t, []int{1002, 4, 3, 4, 33}, []int{1002, 4, 3, 4, 99})
}

func Test7(t *testing.T) {
	verify(t, []int{1101,100,-1,4,0}, []int{1101, 100, -1, 4, 99})
}

func TestEquals8PositionYes(t *testing.T) {
	verifyInputOutput(t, []int{3,9,8,9,10,9,4,9,99,-1,8}, 8, 1)
}

func TestEquals8PositionNo(t *testing.T) {
	verifyInputOutput(t, []int{3,9,8,9,10,9,4,9,99,-1,8}, 7, 0)
}

func TestLessThan8PositionYes(t *testing.T) {
	verifyInputOutput(t, []int{3,9,7,9,10,9,4,9,99,-1,8}, 7, 1)
}

func TestLessThan8PositionNo(t *testing.T) {
	verifyInputOutput(t, []int{3,9,7,9,10,9,4,9,99,-1,8}, 8, 0)
}

func TestEquals8ImmediateYes(t *testing.T) {
	verifyInputOutput(t, []int{3,3,1108,-1,8,3,4,3,99}, 8, 1)
}

func TestEquals8ImmediateNo(t *testing.T) {
	verifyInputOutput(t, []int{3,3,1108,-1,8,3,4,3,99}, 7, 0)
}

func TestLessThan8ImmediateYes(t *testing.T) {
	verifyInputOutput(t, []int{3,3,1107,-1,8,3,4,3,99}, 7, 1)
}

func TestLessThan8ImmediateNo(t *testing.T) {
	verifyInputOutput(t, []int{3,3,1107,-1,8,3,4,3,99}, 8, 0)
}

func verify(t *testing.T, input []int, expected []int) {
	RunProgram(input, TestInputProvider{}, &TestOutputSink{})
	if !reflect.DeepEqual(expected, input) {
		t.Fatalf("Expected: '%s', Actual '%s'", Print(expected), Print(input))
	}
}

func verifyInputOutput(t *testing.T, program []int, input int, output int) {
	testOutputSink := TestOutputSink{}
	var outputSink OutputSink = &testOutputSink
	RunProgram(program, TestInputProvider{Input: input}, outputSink)
	if !testOutputSink.ReceivedValue() {
		t.Fatalf("Expected: '%d', but didn't receive an output", output)
	}
	if output != testOutputSink.Value() {
		t.Fatalf("Expected: '%d', Actual '%d'", output, testOutputSink.Value())
	}
}

type TestInputProvider struct {
	Input int
}

func (tip TestInputProvider) GetInput() int {
	return tip.Input
}

type TestOutputSink struct {
	receivedValue bool
	value int
}

func (tos TestOutputSink) ReceivedValue() bool {
	return tos.receivedValue
}

func (tos TestOutputSink) Value() int {
	return tos.value
}

func (tos *TestOutputSink) OutputValue(value int) {
	tos.receivedValue = true
	tos.value = value
}