File format description:
The file is only parsed as expected IF INSTRUCTIONS ARE FOLLOWED CLOSELY.
Whitespaces are removed while parsing so be verbose where you need.
Every line represents a fixture exept lines consisting of a number and the divider symbol.
Animations are structured in blocks where each block represents a keyframe of the animation (usualy 1/10 of the beat).
Every Keyframe block consists of instructions for every fixture.
Values are comma-separated.
Example:




-------------------------1-------------------------
 50, 50,  0,  0,  100,  100
 50, 50,  0,  0,  100,  100
 50, 50,  0,  0,  100,  100
 50, 50,  0,  0,  100,  100
 50, 50,  0,  0,  100,  100
-------------------------2-------------------------
200, 50,  100,  0,  0,  0
200, 50,  100,  0,  0,  0
200, 50,  100,  0,  0,  0
200, 50,  100,  0,  0,  0
200, 50,  100,  0,  0,  0
-------------------------3-------------------------
200,200,  0,  0,  100,  100
200,200,  0,  0,  100,  100
200,200,  0,  0,  100,  100
200,200,  0,  0,  100,  100
200,200,  0,  0,  100,  100
-------------------------4-------------------------
 50,200,  0,  100,  0,  0
 50,200,  0,  100,  0,  0
 50,200,  0,  100,  0,  0
 50,200,  0,  100,  0,  0
 50,200,  0,  100,  0,  0




 Here all fixtures are doing the same thing.
 As you can see, every key frame is started by the separator followed by five fixture lines.
 The fixture lines contain two values for X and Y, followed by four instructions that indicate
 how bright the fixture should be while moving UP, DOWN, IN or OUT:
 X-Position, Y-Position, UP, DOWN, IN, OUT

 Note: Each keyframe itteration is endless.
 In this Example, the Keyframe after 4 is 1, so the fixture moves from (50, 200) to (50, 50).
 That means it is traveling sideways, resulting in (UP, DOWN, IN, OUT) of (0, 0, 100, 100)