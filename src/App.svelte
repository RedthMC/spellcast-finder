<script lang="ts">
    import Board from "./Board.svelte";
    import Result from "./Result.svelte";

    let resultBox: Result;

    document.onkeydown = event => {
        if (event.key === "Enter") resultBox.click();
    };
</script>

<header>
    <h1>SpellCast Finder</h1>
</header>
<hr />
<main>
    <Board {resultBox} />
    <Result bind:this={resultBox} />
</main>

<style>
    h1 {
        font-size: 3rem;
        margin: 0.5rem;
    }

    hr {
        width: 50rem;
        display: block;
    }

    main {
        margin: 1rem auto;
        display: grid;
        grid-template-columns: auto auto;
        grid-template-rows: auto;
        gap: 1rem;
    }
</style>
