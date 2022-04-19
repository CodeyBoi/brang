# brang
A programming language compiling down to Brainfuck!

## Examples

### Compiling code
Study the following program in file ´print.b´, which takes two character inputs and prints out which is bigger
```
print "input: ";

getchar a;
getchar b;

if a < b {
    print "a<b\n";
} else if a > b {
    print "a>b\n";
} else {
    print "a=b\n";
}
```
This can be compiled using the command
```
/path/to/binary compile print.b
```
This prints the following to the file `b.bf`
```
[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++.+++++.++.+++++.-.----------------------------------
------------------------.--------------------------.[-],>[-],>>[-]+>>>[-]<<[-]<<
<<[->>>>+>>+<<<<<<][-]>>>>>>[-<<<<<<+>>>>>>][-]<[-]<<<<[->>>>+>+<<<<<][-]>>>>>[-
<<<<<+>>>>>]>[-]<[-]<[->+>+<<][-]>>[-<<+>>][-]>[-]+>[-]>>>>[-]<<<[-]<<<<<<[->>>>
>>+>>>+<<<<<<<<<][-]>>>>>>>>>[-<<<<<<<<<+>>>>>>>>>][-]<<[-]<<<<<[->>>>>+>>+<<<<<
<<][-]>>>>>>>[-<<<<<<<+>>>>>>>]<[-]<<+>+<[->-[>]<<]<[<+>>]<<<[-]>[-<+>]<<<[-]>>[
-<<+>>]>[-]<<<[[-]>>>-<<<]>>>+[-<<<+>>>]<<<<<[-]>>[-<<+>>]<<[[-]>-<>>[-]++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++++++++.-------------------------------------.++++++++++++++++++++++++++++++++
++++++.-------------------------------------------------------------------------
---------------.<<]>[->>[-]+>>>>[-]<<[-]<<<<<<<[->>>>>>>+>>+<<<<<<<<<][-]>>>>>>>
>>[-<<<<<<<<<+>>>>>>>>>][-]<[-]<<<<<<<[->>>>>>>+>+<<<<<<<<][-]>>>>>>>>[-<<<<<<<<
+>>>>>>>>][-]>[-]+>[-]>>>>[-]<<<[-]<<<<[->>>>+>>>+<<<<<<<][-]>>>>>>>[-<<<<<<<+>>
>>>>>][-]<<[-]<<<<<<[->>>>>>+>>+<<<<<<<<][-]>>>>>>>>[-<<<<<<<<+>>>>>>>>]<[-]<<+>
+<[->-[>]<<]<[<+>>]<<<<[-]>>[-<<+>>][-]<<[[-]>>-<<]>>+[-<<+>>]<<<<<[-]>>>[-<<<+>
>>]<<<[[-]>-<>>>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++.-----------------------------------.+++++++
+++++++++++++++++++++++++++++.--------------------------------------------------
--------------------------------------.<<<]>[->>[-]+++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.-----------
-------------------------.+++++++++++++++++++++++++++++++++++++.----------------
------------------------------------------------------------------------.<<]<<]
```
which is a valid Brainfuck program that operates as you would expect! Magic!

### Running a Brainfuck program
Now when you have a .bf file you can also run it using a runtime visualizer! Simply type
```
/path/to/binary run b.bf
```
You will see some output of the form
```
Memory:
 97  114  0 [ 1 ] 4   0   4 

Instructions:
[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++.++.+++++.-.----------------------
                                                                                                           v
------------------------------------.--------------------------.[-],>[-],>>[-]+>>>[-]<<[-]<<<<[->>>>+>>+<<<<<<][-]>>>>>>[-<<<<<<+>>>>>>][-]<[-]<<<<[
->>>>+>+<<<<<][-]>>>>>[-<<<<<+>>>>>]>[-]<[-]<[->+>+<<][-]>>[-<<+>>][-]>[-]+>[-]>>>>[-]<<<[-]<<<<<<[->>>>>>+>>>+<<<<<<<<<][-]>>>>>>>>>[-<<<<<<<<<+>>>
>>>>>>][-]<<[-]<<<<<[->>>>>+>>+<<<<<<<][-]>>>>>>>[-<<<<<<<+>>>>>>>]<[-]<<+>+<[->-[>]<<]<[<+>>]<<<[-]>[-<+>]<<<[-]>>[-<<+>>]>[-]<<<[[-]>>>-<<<]>>>+[-
<<<+>>>]<<<<<[-]>>[-<<+>>]<<[[-]>-<>>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.----------
---------------------------.++++++++++++++++++++++++++++++++++++++.---------------------------------------------------------------------------------
-------.<<]>[->>[-]+>>>>[-]<<[-]<<<<<<<[->>>>>>>+>>+<<<<<<<<<][-]>>>>>>>>>[-<<<<<<<<<+>>>>>>>>>][-]<[-]<<<<<<<[->>>>>>>+>+<<<<<<<<][-]>>>>>>>>[-<<<<
<<<<+>>>>>>>>][-]>[-]+>[-]>>>>[-]<<<[-]<<<<[->>>>+>>>+<<<<<<<][-]>>>>>>>[-<<<<<<<+>>>>>>>][-]<<[-]<<<<<<[->>>>>>+>>+<<<<<<<<][-]>>>>>>>>[-<<<<<<<<+>
>>>>>>>]<[-]<<+>+<[->-[>]<<]<[<+>>]<<<<[-]>>[-<<+>>][-]<<[[-]>>-<<]>>+[-<<+>>]<<<<<[-]>>>[-<<<+>>>]<<<[[-]>-<>>>[-]+++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.-----------------------------------.++++++++++++++++++++++++++++++++++++.----------
------------------------------------------------------------------------------.<<<]>[->>[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++.------------------------------------.+++++++++++++++++++++++++++++++++++++.--------------------------------
--------------------------------------------------------.<<]<<]

Output:
input: 
```
Here you can see the memory of the process, the instruction table and the process' output in realtime!
