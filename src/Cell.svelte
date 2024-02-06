<script lang="ts">
    import Result from "./Result.svelte";
    import { board, result, preventEnter } from "./main";
    import type { JsSearchResult } from "./wasm/spellcast_finder";

    export let resultBox: Result;
    export let cells: HTMLButtonElement[];
    export let index: number;
    export let letter: string;

    function onKeyDown(event: KeyboardEvent, index: number) {
        let key = event.key;
        switch (key) {
            case "ArrowUp":
                cells[index - 5]?.focus();
                return;
            case "ArrowDown":
                cells[index + 5]?.focus();
                return;
            case "Backspace":
            case "ArrowLeft":
                cells[index - 1]?.focus();
                return;
            case "ArrowRight":
                cells[index + 1]?.focus();
                return;
        }
        if (key.length !== 1) return;
        if (!key.match(/[A-Z]|[a-z]/g)) return;
        key = key.toUpperCase();
        $board.letters[index] = key;
        (cells[index + 1] || resultBox).focus();
    }

    function setX2(index: number) {
        if ($board.double_score !== index) {
            $board.double_score = index;
        } else {
            $board.double_score = -1;
        }
    }

    function setDL(index: number) {
        if ($board.double_letter !== index) {
            $board.double_letter = index;
            $board.triple_letter = -1;
        } else {
            $board.double_letter = -1;
        }
    }

    function setTL(index: number) {
        if ($board.triple_letter !== index) {
            $board.triple_letter = index;
            $board.double_letter = -1;
        } else {
            $board.triple_letter = -1;
        }
    }

    function getPathStyle(path: Int32Array| undefined) {
        if (!path?.includes(index)) return "";
        const pathOrder = path.indexOf(index);
        if (pathOrder === 0) return "path start";
        const lastCell = path[pathOrder - 1];
        const offset = index - lastCell;
        return `path path${offset}`;
    }

    function getSwap(result: JsSearchResult | undefined, index: number) {
        if (result?.swapped_index === undefined) return;
        const swappedPos = result.path[result.swapped_index]
        switch (swappedPos) {
            case index: 
                return result.word[result.swapped_index].toUpperCase();
            default:
                return;
        }
    }

</script>

<div class={getPathStyle($result?.path)}>
    <button bind:this={cells[index]} class="cell" on:keydown={event => onKeyDown(event, index)}>
        <button class={"x2" + ($board.double_score === index ? " selected" : "")} on:click={() => setX2(index)} on:keydown={preventEnter}>2x</button>
        <button class={"dl" + ($board.double_letter === index ? " selected" : "")} on:click={() => setDL(index)} on:keydown={preventEnter}>DL</button>
        <button class={"tl" + ($board.triple_letter === index ? " selected" : "")} on:click={() => setTL(index)} on:keydown={preventEnter}>TL</button>
        {#if getSwap($result, index)}
            <div class="swap">{getSwap($result, index)}</div>
        {/if}
        {letter}
    </button>
</div>

<style>
    .cell {
        background-color: #3a3a3a;
        font-size: 3rem;
        padding: 0;
        color: #efefef;
        font-family: Poppins, "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
        border-radius: 1rem;
        transition: 100ms;
        outline: 0rem solid aqua;
        border: none;
        position: relative;
        width: 100%;
        height: 100%;
        z-index: 1;
    }
    .cell:hover {
        background-color: #797979;
    }
    button:focus {
        outline: 0.2rem solid aqua;
    }
    .path .cell {
        color: #efefef;
        background-color: #00690e;
    }
    .path .cell:hover {
        background-color: #1ca800;
    }
    .path.start .cell {
        background-color: #006960;
    }
    .path.start .cell:hover {
        background-color: #00a892;
    }

    .path {
        position: relative;
    }
    .path::before {
        position: absolute;
        content: "";
        background-color: #1ca800;
        width: 100%;
        height: 20%;
    }
    .start::before {
        display: none;
    }
    .path-6::before {
        bottom: -10%;
        right: -50%;
        rotate: 45deg;
    }
    .path-5::before {
        bottom: 0%;
        left: 0%;
        rotate: 90deg;
    }
    .path-4::before {
        bottom: -10%;
        left: -50%;
        rotate: -45deg;
    }
    .path-1::before {
        top: 40%;
        left: 50%;
    }
    .path1::before {
        top: 40%;
        left: -50%;
    }
    .path4::before {
        top: -10%;
        right: -50%;
        rotate: -45deg
    }
    .path5::before {
        top: 0%;
        left: 0%;
        rotate: 90deg;
    }
    .path6::before {
        top: -10%;
        left: -50%;
        rotate: 45deg;
    }
    
    .cell button {
        position: absolute;
        color: #efefef;
        font-family: Poppins, "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
        width: 40%;
        height: 40%;
        border-radius: 50%;
        opacity: 0;
        align-items: center;
        font-size: 1.5rem;
        display: grid;
        border: none;
        transition: 100ms;
    }
    
    .swap {
        position: absolute;
        color: #efefef;
        font-family: Poppins, "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
        width: 40%;
        height: 40%;
        border-radius: 50%;
        opacity: 1;
        align-items: center;
        font-size: 1.5rem;
        display: grid;
        border: none;
        transition: 100ms;
        bottom: -10%;
        right: -10%;
        background: rgb(189, 13, 0);
    }

    .x2 {
        top: -10%;
        right: -10%;
        background: rgb(189, 0, 189);
    }

    .dl {
        top: -10%;
        left: -10%;
        background: rgb(189, 154, 0);
    }

    .tl {
        bottom: -10%;
        left: -10%;
        background: rgb(189, 154, 0);
    }

    .cell button:active {
        transform: translateY(10%);
    }
    .cell:hover button:not(.selected) {
        opacity: 0.4;
    }
    .cell .selected {
        opacity: 1;
    }
</style>
