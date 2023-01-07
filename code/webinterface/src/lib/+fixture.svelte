<script lang="ts">
    import Textinput from '@jonas_focke/svelcon/Input/Textfield.svelte'
    import Text from '@jonas_focke/svelcon/Wrapper/Text.svelte'
    import Checkbox from '@jonas_focke/svelcon/Input/Checkbox.svelte'
    export let name = "";
    let moving = false;
    let m = { x: 0, y: 0 };

    let directionX = "0";
    let directionY = "0";
    let directionUP = false;
    let directionDOWN = false;
    let directionIN = false;
    let directionOUT = false;
    let fixtureDimm = "0";

    $: {
        name = name.replaceAll(" ", "");
    }

	function handleMousemove(event: MouseEvent) {
        if (moving) {
            m.x += event.movementX;
            m.y += event.movementY;
            console.log(Math.floor(scale(m.x, 0, 1920, 255, 0)))
            console.log(Math.floor(scale(m.y, 0, 1920, 255, 0)))
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
    class="w-20 h-20 cursor-move absolute" 
    style="left: {m.x}px; top: {m.y}px;"
    on:mousedown={()=> moving = true} 
>
    <img 
        src="{name + '.jpg'}" 
        alt="{name}" 
    />

    <div class="flex flex-row">
<div>

    <Text text="X: " />
    
    <Text text="Y: " />
    
    <Text text="UP: " />
    
    <Text text="DOWN: " />
    
    <Text text="IN: " />
    
    <Text text="OUT: " />
    
    <Text text="DIMM: " />
</div>
    <div>

        <Textinput bind:value={directionX} />
        <Textinput bind:value={directionY} />
        <Checkbox label="" bind:checked={directionUP} />
        <Checkbox label="" bind:checked={directionDOWN} />
        <Checkbox label="" bind:checked={directionIN} />
        <Checkbox label="" bind:checked={directionOUT} />
        <Textinput bind:value={fixtureDimm} />
    </div>    
    </div>
        
    
    
    
    
    
    
    
    
    
    
    
    
    
    

</div>