<script lang="ts">
    import Board from "./Board.svelte";
    import Result from "./Result.svelte";
    import icon from "./spellcast-finder.svg";
    let resultBox: Result;

    document.onkeydown = event => {
        if (event.key === "Enter") {
            if (event.shiftKey) resultBox.clickCanSwap();
            else resultBox.click();
        }
    };
</script>

<header>
    <h1>SpellCast Finder</h1>
    <img src={icon} alt="Icon" />
    <a href="https://github.com/RedthMC/spellcast-finder">by Redth</a>
</header>
<hr />
<main>
    <Board {resultBox} />
    <Result bind:this={resultBox} />
</main>

<style>
    header {
        padding: 0.5rem;
        display: grid;
        align-items: center;
        gap: 1rem;
        grid-template-columns: auto auto auto;
        grid-template-rows: auto;
    }

    h1 {
        font-size: 2.5rem;
        margin: 0.5rem;
        display: inline;
    }

    img {
        display: inline;
        width: 5rem;
        height: 5rem;
    }

    a {
        font-size: 1rem;
        color: #efefef;
    }

    hr {
        width: 50rem;
        display: block;
    }

    main {
        margin: 1rem auto;
        display: grid;
        grid-template-columns: auto;
        grid-template-rows: auto auto;
        gap: 1rem;
    }
</style>
