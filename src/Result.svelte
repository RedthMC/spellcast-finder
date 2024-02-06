<script lang="ts">
    import { findWord, findWordCanSwap, preventEnter, result } from "./main";

    let findButton: HTMLButtonElement;
    let style = "";
    let style2 = "";

    export function click() {
        style = "";
        setTimeout(() => style = "clicked");
        findWord();
    }
    
    export function clickCanSwap() {
        style2 = "";
        setTimeout(() => style2 = "clicked");
        findWordCanSwap();
    }

    export function focus() {
        findButton.focus();
    }
</script>

<div>
    <h2>Result</h2>
    <p>
        {#if $result}
            {$result.word}
            <br />
            {$result.score} Points
        {:else}
            No Result
        {/if}
    </p>
    <button bind:this={findButton} on:click={click} on:keydown={preventEnter} class={style}>Find</button>
    <button on:click={clickCanSwap} on:keydown={preventEnter} class={style2}>Can Swap</button>
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
    button.clicked::after {
        animation-duration: 500ms;
        animation-name: explode;
    }
    @keyframes explode {
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
