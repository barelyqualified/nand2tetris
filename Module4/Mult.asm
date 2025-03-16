@R0
A=M
D=A
@value1
M=D

@R1
A=M
D=A
@value2
M=D

@0 
D=A
@R2
M=D

@0
D=A
@i
M=D

@0
D=A
@temp_result
M=D

(calc_loop)
@i
D=M
@value2
D=D-M
@store_result
D;JEQ // if i == value2 GOTO store_result


@value1
A=M
D=A
@temp_result
M=D+M


@i
M=M+1 // i = i + 1

@calc_loop
0;JMP

(store_result)
@temp_result
A=M
D=A
@R2
M=D

@END // Prevent any NOP slide attacks.


(END)
@END
0;JMP