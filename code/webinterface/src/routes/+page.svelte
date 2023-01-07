<script lang="ts">
    import Fixture from '$lib/+fixture.svelte';
    import Textinput from '@jonas_focke/svelcon/Input/Textfield.svelte'
	import { onMount } from 'svelte';
    import config from '../../../logic/config.json'

    onMount(()=> {
        fetch('http://localhost:3000/cars')
        // {
        //     method: 'POST',
        //     body: JSON.stringify({
        //         'my_object': 'MOIN'
        //     }),
        //     headers: {
        //         'Content-Type': 'application/json'
        //     }
        // })
        .then(response => console.log(response.json()))
    })
</script>

{#each config.dmx.fixtures as fixture}
    <Fixture name={fixture} />
{/each}
<Textinput />