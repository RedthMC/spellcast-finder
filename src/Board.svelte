<script lang="ts">
    import Cell from "./Cell.svelte";
    import Result from "./Result.svelte";
    import { board } from "./main";

    export let resultBox: Result;

    let cells: HTMLButtonElement[] = [];

</script>

<div>
    {#each $board.letters as letter, index (index)}
        <Cell {resultBox} {cells} {index} {letter}/>
    {/each}
</div>

<style>
    div {
        width: 45rem;
        height: 45rem;
        display: grid;
        gap: 2rem;
        padding: 2rem;
        border-radius: 2rem;
        background-color: #202020;
        grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
        grid-template-rows: 1fr 1fr 1fr 1fr 1fr;
    }
</style>
