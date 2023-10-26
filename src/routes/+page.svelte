<script lang="ts">
	import { greet } from 'rust';

	let name = '';
</script>

<input bind:value={name} placeholder="Enter your name" />
<h1>{greet(name || 'SvelteKit')}</h1>
