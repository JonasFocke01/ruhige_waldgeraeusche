File format description:
The file is only parsed as expected IF INSTRUCTIONS ARE FOLLOWED CLOSELY.
Whitespaces are removed while parsing so be verbose where you need.
Every line represents a scanner exept lines consisting of a number and the divider symbol.
Animations are structured in blocks where each block represents a keyframe of the animation.
Every Keyframe block consists of instructions for every scanner.
Values are comma-separated.
Example:




-------------------------1-------------------------
 50, 50,  0,  0,  1,  1
 50, 50,  0,  0,  1,  1
 50, 50,  0,  0,  1,  1
 50, 50,  0,  0,  1,  1
 50, 50,  0,  0,  1,  1
-------------------------2-------------------------
200, 50,  1,  0,  0,  0
200, 50,  1,  0,  0,  0
200, 50,  1,  0,  0,  0
200, 50,  1,  0,  0,  0
200, 50,  1,  0,  0,  0
-------------------------3-------------------------
200,200,  0,  0,  1,  1
200,200,  0,  0,  1,  1
200,200,  0,  0,  1,  1
200,200,  0,  0,  1,  1
200,200,  0,  0,  1,  1
-------------------------4-------------------------
 50,200,  0,  1,  0,  0
 50,200,  0,  1,  0,  0
 50,200,  0,  1,  0,  0
 50,200,  0,  1,  0,  0
 50,200,  0,  1,  0,  0




 Here all scanners are doing the same thing.
 As you can see, every key frame is started by the separator followed by five scanner lines.
 The scanner lines contain two values for X and Y, followed by four instructions that indicate
 if this keyframe moves the scanner UP, DOWN, IN or OUT:
 X-Position, Y-Position, UP, DOWN, IN, OUT

 Note: Each keyframe itteration is endless.
 In this Example, the Keyframe after 4 is 1, so the scanner moves from (50, 200) to (50, 50).
 That means it is traveling sideways, resulting in (UP, DOWN, IN, OUT) of (0, 0, 1, 1)