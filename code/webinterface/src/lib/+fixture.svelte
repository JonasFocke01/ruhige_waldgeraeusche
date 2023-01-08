<script lang="ts">
    import Text from '@jonas_focke/svelcon/Wrapper/Text.svelte'
    import Button from '@jonas_focke/svelcon/Input/Button.svelte'
    import Checkbox from '@jonas_focke/svelcon/Input/Checkbox.svelte'
    import type { FixturePositions } from 'src/routes/+page.svelte';

    import P5 from 'p5-svelte';
    export let positions: Array<FixturePositions> = [];
    let editingPositionIndex = -1;
    let createMode = true;
    let editMode = false;
    
    // Todo: remove any
	const sketch = (p5: any) => {
        p5.setup = () => {
			p5.createCanvas(width, height);
		};

		p5.draw = () => {
            p5.background(220);
            p5.stroke(0);
            p5.strokeWeight(0.3);
            for (let i = 0; i < 255; i += 10) {
                p5.line(i, 0, i, 255);
                p5.line(0, i, 255, i);
            }
            p5.strokeWeight(1);
            positions.forEach((position, i) => {
                p5.noStroke();
                p5.fill(scale(i, 0, positions.length, 255, 0), i % 6 === 0 ? 255 : 0, scale(i, 0, positions.length, 0, 255));
                if (editMode && i === editingPositionIndex && Date.now() % 100 > 50) {
                    p5.circle(position.x, position.y, 10);
                } else if (i !== editingPositionIndex) {
                    p5.circle(position.x, position.y, 10);
                }
                if (i > 0) {
                    p5.stroke(0);
                    p5.line(position.x, position.y, positions[i - 1].x, positions[i - 1].y);
                } 
            });
            if (positions.length > 1) {
                p5.line(positions[0].x, positions[0].y, positions[positions.length - 1].x, positions[positions.length - 1].y);
            }
            positions = positions;
		};

        p5.mousePressed = () => {
            if (createMode) {
                if (p5.mouseX > -1 && p5.mouseX < width && p5.mouseY > -1 && p5.mouseY < height) {
                    positions.push({x: Math.floor(p5.mouseX / 10) * 10, y: Math.floor(p5.mouseY / 10) * 10, directionUp: false, directionDown: false, directionIn:false, directionOut:false, brightness: 0})
                }
            }

            if (editMode) {
                positions.forEach((position, i) => {
                    if (p5.dist(p5.mouseX, p5.mouseY, position.x, position.y) < 11) {
                        editingPositionIndex = i;
                    }
                })
            }
        };

        p5.keyPressed = (key: KeyboardEvent) => {
            if (editMode) {
                if (editingPositionIndex > -1) {
                    if (key.key === "ArrowUp" && positions[editingPositionIndex].y > 0) {
                        positions[editingPositionIndex].y -= 10;
                    } else if (key.key === "ArrowDown" && positions[editingPositionIndex].y < 250) {
                        positions[editingPositionIndex].y += 10;
                    } else if (key.key === "ArrowLeft" && positions[editingPositionIndex].x > 0) {
                        positions[editingPositionIndex].x -= 10;
                    } else if (key.key === "ArrowRight" && positions[editingPositionIndex].x < 250) {
                        positions[editingPositionIndex].x += 10;
                    }
                }
                if (key.key === "Enter") {
                    if (editingPositionIndex === positions.length - 1) {
                        editingPositionIndex = 0;    
                    } else {
                        editingPositionIndex += 1;
                    }
                } if (key.key === "Backspace") {
                    if (editingPositionIndex < 1) {
                        editingPositionIndex = positions.length - 1;    
                    } else {
                        editingPositionIndex -= 1;
                    }
                }
            }
        }
	};

    export let name = "";
    export let id: number;
    let moving = false;
    let m = { x: 500, y: 300 };

    let width = 250;
    let height = 250;


    $: {
        name = name.replaceAll(" ", "");
    }

	function handleMousemove(event: MouseEvent) {
        if (moving) {
            m.x += event.movementX;
            m.y += event.movementY;
        }
    }

    function scale (number: number, inMin: number, inMax: number, outMin: number, outMax: number) {
        return (number - inMin) * (outMax - outMin) / (inMax - inMin) + outMin;
    }
</script>

<svelte:window 
    on:mousemove={handleMousemove} 
    on:mouseup={()=> moving = false} 
/>


<div
    class="absolute" 
    style="left: {m.x}px; top: {m.y}px; width: {width}px; height: {height}px;"
>
    <div class="flex flex-row">
        <div class="w-1/3">
            P: {positions.length}
        </div>
        <div class="flex flex-col justify-center cursor-move bg-primary w-1/3 text-lg text-text" on:mousedown={()=> moving = true} >
            <Text text={name} />
            <Text text={"" + (id + 1)} />
        </div>
        <div class="w-1/3">
            <img 
                src="{name + '.jpg'}" 
                alt="{name}" 
            />
        </div>
    </div>
    <div class="cursor-pointer border">
        <P5 {sketch} />
    </div>
    {#key createMode}
        {#if createMode}
            <div class="border m-2">
            
            <Button bgColor="surface" text="remove last" on:click={() => positions.pop()} />
            </div>
            <div class="border m-2">
                
                <Button bgColor="surface" text="clear" on:click={() => {
                    if (confirm('Are you sure you want to clear?')){
                        positions = []
                    }
                }
            }
                />
            </div>
            <div class="border m-2">
                <Button bgColor="surface" text={'🖋'} on:click={() => {
                    editMode = true;
                    createMode = false;
                }} />
            </div>
        {:else if editMode}
            {#if editingPositionIndex > -1}
                <div class="bg-primary px-2 flex flex-col">
                    <Checkbox label="Direction Up" bind:checked={positions[editingPositionIndex].directionUp} />
                    <Checkbox label="Direction Down" bind:checked={positions[editingPositionIndex].directionDown} />
                    <Checkbox label="Direction In" bind:checked={positions[editingPositionIndex].directionIn} />
                    <Checkbox label="Direction Out" bind:checked={positions[editingPositionIndex].directionOut} />
                    <input type="range" min="0" max="1" step="0.01" bind:value={positions[editingPositionIndex].brightness} />
                </div>
            {/if}
            <div class="border m-2">
                <Button bgColor="surface" text={'🔙'} on:click={() => {
                    editMode = false;
                    createMode = true;
                    editingPositionIndex = -1;
                }} />
            </div>
        {/if}
    {/key}
</div>