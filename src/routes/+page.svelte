<script lang="ts">
	import { greet, add } from 'rust';

	let name = '';
</script>

<input bind:value={name} placeholder="Enter your name" />
<h1>{greet(name || 'SvelteKit')}</h1>
<h2>1 + 1 = {add(1, 1)}</h2>
