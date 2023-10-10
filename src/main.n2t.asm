(RESET)
// Load 5000 into D register
@5000
D=A
// Save that value into a memory address with a symbol
// TARGET = 5000
@TARGET
M=D
// Load constant 0 into D register
@0
D=A
// Set COUNT = 0
@COUNT
M=D

(LOOP_START)
@TARGET
D=M
@COUNT
// D = TARGET - COUNT
D=D-M
@LOOP_START
D; JGT
@1
D=A
@LIGHT_ADDRESS
M=D
@RESET
0; JMP
