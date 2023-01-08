<script lang="ts">
    import Fixture from '$lib/+fixture.svelte';
    import Header from '$lib/+header.svelte';
	import { onMount } from 'svelte';
    import config from '../../../logic/config.json'

    let fixtures: Array<Array<{x: number, y: number}>> = [];
    let mounted = false;
    onMount(()=> {
        mounted = true;
        config.dmx.fixtures.forEach(() => {
            fixtures.push([]);
        })
    })

    // export to file

    function exportToFile (e: DispatchEvent) {
        if (mounted && e.detail.animationName !== "") {

            // construct file content

            // write file
            const blob = new Blob(["Bla\nMoin"], {type: 'text/plain'});
            const url = URL.createObjectURL(blob);
            const link = document.createElement('a');
            link.download = e.detail.animationName + '.tpl';
            link.href = url;
            link.click();
            URL.revokeObjectURL(url); // Object URLs should be revoked after use
        }
    }
</script>

<Header on:save={(e) => exportToFile(e)} />
{#each config.dmx.fixtures as fixture, i}
    <Fixture name={fixture} id={i} bind:positions={fixtures[i]} />
{/each}