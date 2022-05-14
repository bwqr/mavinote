<script lang="ts">
    import init, { greet } from 'wasm';

    init().then(res => console.log(greet())).catch(e => console.error('error', e));
</script>

<div>
    <h1>Hello World!</h1>
</div>
