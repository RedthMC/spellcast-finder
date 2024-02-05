<script lang="ts">
    import Result from "./Result.svelte";
    import { board, path } from "./stores";

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
</script>

<button bind:this={cells[index]} class={"cell" + ($path.includes(index) ? " path" : "")} on:keydown={event => onKeyDown(event, index)}>
    <button
        class={"x2" + ($board.double_score === index ? " selected" : "")}
        on:click={() => setX2(index)}
        on:keydown={event => {
            if (event.key === "Enter") event.preventDefault();
        }}
    >
        2x
    </button>
    {letter}
</button>

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
    }
    .cell:hover {
        background-color: #797979;
    }
    button:focus {
        outline: 0.2rem solid aqua;
    }
    .cell.path {
        color: #efefef;
        background-color: #00690e;
    }
    .cell.path:hover {
        background-color: #1ca800;
    }

    .x2 {
        position: absolute;
        color: #efefef;
        font-family: Poppins, "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
        top: -10%;
        right: -10%;
        width: 40%;
        height: 40%;
        border-radius: 50%;
        z-index: 1;
        background: rgb(189, 0, 189);
        opacity: 0;
        align-items: center;
        font-size: 1.5rem;
        display: grid;
        border: none;
        transition: 100ms;
    }
    .x2:active {
        transform: translateY(10%);
    }
    .cell:hover .x2:not(.selected) {
        opacity: 0.4;
    }
    .x2.selected {
        opacity: 1;
    }
</style>
