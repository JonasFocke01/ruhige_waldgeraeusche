<script lang="ts">
    import Button from '@jonas_focke/svelcon/Input/Button.svelte';
    import Textfield from '@jonas_focke/svelcon/Input/Textfield.svelte';
    import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

    let animationName = "";

    function exportToFile() {
        dispatch('save', { animationName });
    }
</script>
<div class="w-full flex flex-row bg-surface h-14">
    <div class="w-1/4"></div>
    <div class="w-1/4 p-2 flex flex-col justify-center">
        <Textfield bind:value={animationName} placeholder="Animation name" />
    </div>
    <div class="w-1/4 p-2 flex flex-col justify-center">
        <Button text="speichern" bgColor="primary" on:click={() => exportToFile()} />
    </div>
    <div class="w-1/4"></div>
</div>