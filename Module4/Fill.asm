@24576
D=A
@kbd_arr
M=D

@16384
D=A
@screen_arr
M=D

@i
M=0

@8192
D=A
@n
M=D

(KEYLOOP)
@i
M=0

@8192
D=A
@n
M=D 

@kbd_arr // Retrieve current value from KBD
A=M
D=M
@FILLLOOP
D-1;JGE // if kbd_arr >= 0
@UNFILLLOOP 
D;JEQ // if kbd_arr == 0

@KEYLOOP
0;JMP


(FILLLOOP)
@i
D=M
@n
D=D-M
@KEYLOOP /* Read the value from i and subtract n from it, if it is equal to 0, we fulfilling our branch. */
D;JEQ // if i == n GOTO KEYLOOP

@screen_arr
D=M
@i
A=D+M
M=-1

@i
M=M+1


@kbd_arr
D=M
@KEYLOOP
D;JEQ // if kbd_arr == 0 GOTO KEYLOOP

@FILLLOOP
0;JMP


(UNFILLLOOP)
@i
D=M
@n
D=D-M
@KEYLOOP
D;JEQ // if i == n GOTO KEYLOOP

@screen_arr
D=M
@i
A=D+M
M=0

@i
M=M+1

@kbd_arr
D=M
@KEYLOOP
D;JEQ // if kbd_arr >= 0 GOTO KEYLOOP


@UNFILLLOOP
0;JMP

