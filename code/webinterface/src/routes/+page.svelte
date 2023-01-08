<script lang="ts">
    import Fixture from '$lib/+fixture.svelte';
    import Header from '$lib/+header.svelte';
	import { onMount } from 'svelte';
    import config from '../../../logic/config.json'

    let fixtures: Array<Array<FixturePositions>> = [];
    let mounted = false;
    onMount(()=> {
        mounted = true;
    })

    function exportToFile (e: CustomEvent) {
        if (mounted && fixtures.length > 0) {
            // guards
            if (fixtures.find((e) => e.length !== fixtures[0].length)) {
                alert("All Fixtures need the same amount of animation positions");
                return
            }
            while (!e.detail.animationName) {
                e.detail.animationName = prompt('Please specify an animation name');
            }
            // construct file content
            let content = "";
            for (let i = 0; i < fixtures[0].length; i++) {
                content += "-------------------\n";
                for (let j = 0; j < fixtures.length; j++) {
                    content += 
                    fixtures[j][i].x + ", " +
                    fixtures[j][i].y + ", " +
                    (fixtures[j][i].directionUp ? '1' : 0) + ", " +
                    (fixtures[j][i].directionDown ? '1' : 0) + ", " +
                    (fixtures[j][i].directionIn ? '1' : 0) + ", " +
                    (fixtures[j][i].directionOut ? '1' : 0) + ", " +
                    (fixtures[j][i].brightness === 1 ? '1.0' : fixtures[j][i].brightness === 0 ? '0.0' : fixtures[j][i].brightness) + "\n";
                }
            }

            // write file
            const blob = new Blob([content], {type: 'text/plain'});
            const url = URL.createObjectURL(blob);
            const link = document.createElement('a');
            link.download = e.detail.animationName + '.tpl';
            link.href = url;
            link.click();
            URL.revokeObjectURL(url); // Object URLs should be revoked after use
        }
    }
</script>

<script lang="ts" context="module">
    export interface FixturePositions {
        x: number,
        y: number,
        directionUp: boolean,
        directionDown: boolean,
        directionIn: boolean,
        directionOut: boolean,
        brightness: number
    }
</script>

<Header on:save={(e) => exportToFile(e)} />
{#each config.dmx.fixtures as fixture, i}
    <Fixture name={fixture} id={i} bind:positions={fixtures[i]} />
{/each}