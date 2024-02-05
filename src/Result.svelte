<script lang="ts">
    import { find, type JsSearchResult } from "./wasm/spellcast_finder.js";
    import { board, path } from "./stores.js";

    let clicked: boolean | undefined;
    let result: JsSearchResult | undefined;
    let findButton: HTMLButtonElement;

    export function click() {
        clicked = !clicked;
        result = find($board.letters.join("").toLowerCase(), $board);
        path.set(result?.path || Int32Array.of());
    }

    export function focus() {
        findButton.focus();
    }
</script>

<div>
    <h2>Result</h2>
    <p>
        {#if result}
            {result.word}
            <br />
            {result.score} Points
        {:else}
            No Result
        {/if}
    </p>
    <button bind:this={findButton} on:click={click} on:keydown|preventDefault class={clicked === undefined ? "" : clicked ? "clicked1" : "clicked2"}>Find</button>
</div>

<style>
    div {
        display: grid;
        gap: 1rem;
        padding: 1rem;
        background-color: #202020;
        border-radius: 1rem;
        align-items: center;
        margin: auto 0;
    }

    h2 {
        font-size: 2rem;
        margin: 0rem;
    }

    p {
        font-size: 1.4rem;
        margin: 0;
    }

    button {
        font-family: Poppins, "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
        background-color: #3a3a3a;
        border: none;
        outline: aqua solid 0rem;
        color: #efefef;
        border-radius: 0.5rem;
        font-size: 1.4rem;
        transition: 100ms;
        cursor: pointer;
        position: relative;
    }
    button:focus {
        outline: 0.2rem solid aqua;
        transition: 100ms;
    }
    button:hover {
        background-color: #797979;
        transition: 100ms;
    }
    button:active {
        transform: translateY(10%);
        transition: 100ms;
    }
    button::after {
        content: "";
        display: block;
        position: absolute;
        border-radius: 0.5rem;
        left: 0;
        top: 0;
        width: 100%;
        height: 100%;
    }
    button.clicked1::after {
        animation-duration: 500ms;
        animation-name: explode1;
    }
    button.clicked2::after {
        animation-duration: 500ms;
        animation-name: explode2;
    }
    @keyframes explode1 {
        from {
            box-shadow: 0 0 0 0 white;
            opacity: 1;
        }
        to {
            box-shadow: 0 0 10px 40px white;
            opacity: 0;
        }
    }
    @keyframes explode2 {
        from {
            box-shadow: 0 0 0 0 white;
            opacity: 1;
        }
        to {
            box-shadow: 0 0 10px 40px white;
            opacity: 0;
        }
    }
</style>
