<script>
    import { onMount } from 'svelte';

    onMount(()=>{
        fetch('http://192.168.2.17:5000/');
    })


</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button on:click={()=>fetch('http://192.168.2.17:5000/snake/')}>SNAKE</button>
<button on:click={()=>fetch('http://192.168.2.17:5000/color/')}>COLOR</button>
