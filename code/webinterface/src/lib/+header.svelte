<script lang="ts">
    import Button from '@jonas_focke/svelcon/Input/Button.svelte';
    import Textfield from '@jonas_focke/svelcon/Input/Textfield.svelte';
    import FileDrop from 'filedrop-svelte';
    import type { Files } from "filedrop-svelte";
    
	let files: Files;
    import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher<{save: { animationName: string }, fileDroped: { content: string }}>();

    let animationName = "";

    function exportToFile() {
        dispatch('save', { animationName });
    }

</script>
<div class="w-full flex flex-row bg-surface h-14">
    <div class="w-1/4 p-2 flex flex-col justify-center">
        <FileDrop on:filedrop={(e) => { 
            files = e.detail.files;
            files["accepted"][0].text().then((filecontent) => dispatch('fileDroped', {content: filecontent}))
            }}>
            Upload animationfiles (.tpl)
        </FileDrop>
    </div>
    <div class="w-1/4 p-2 flex flex-col justify-center">
        <Textfield bind:value={animationName} placeholder="Animation name" />
    </div>
    <div class="w-1/4 p-2 flex flex-col justify-center">
        <Button text="speichern" bgColor="primary" on:click={() => exportToFile()} />
    </div>
    <div class="w-1/4"></div>
</div>