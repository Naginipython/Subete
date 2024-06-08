<script>
    import { goto } from "$app/navigation";
    import { faClockRotateLeft, faSun, faMoon, faGear } from '@fortawesome/free-solid-svg-icons'
    import Fa from 'svelte-fa'

    let special = false;

</script>

more screen

<hr>

<div>
    <div class="box">
        <div class="more-links">
            <div class="link-icon">
                {#if special}<Fa icon={faMoon} />
                {:else}<Fa icon={faSun} />
                {/if}
            </div>
            <p>Toggle Special (WIP)</p>
        </div>
        <div class="right-float">
            <label class="switch">
                <input type="checkbox" bind:checked={special}>
                <span class="slider"></span>
            </label>
        </div>
    </div>
</div>

<hr>

<div class="box more-options">
    <button class="more-links" on:click={() => goto('/')}>
        <div class="link-icon"><Fa icon={faClockRotateLeft} /></div>
        <p>History (WIP)</p>
    </button>
</div>
<div class="box more-options">
    <button class="more-links" on:click={() => goto('/settings')}>
        <div class="link-icon"><Fa icon={faGear} /></div>
        <p>Settings</p>
    </button>
</div>

<style>
    hr {
        border-color: grey;
    }
    .box {
        width: 100vw;
        overflow: auto;
        display: inline-flex;
        background-color: var(--secondary-color);
    }
    .more-options:hover {
        background-color: var(--selection-color)
    }
    .more-links {
        text-align: left;
        cursor: pointer;
        display: inline-flex;
        justify-content: left;
        align-items: center;
        border: 0;
        background-color: transparent;
        color: inherit;
        width: 100%;
        float: left;
        padding: 10px 0;
    }
    .more-links p {
        justify-content: left;
        width: inherit;
        padding: 0;
        margin: 0;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        font-size: large;
    }
    .link-icon {
        padding: 0 10px;
        font-size: large;
    }
    .switch {
        position: relative;
        width: 60px;
        height: 34px;
    }
    .switch input {
        opacity: 0;
    }
    .slider {
        position: absolute;
        cursor: pointer;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: #ccc;
        transition: .4s;
        border-radius: 34px;
    }
    .slider:before {
        position: absolute;
        content: "";
        height: 26px;
        width: 26px;
        left: 4px;
        bottom: 4px;
        background-color: white;
        transition: .4s;
        border-radius: 50%;
    }
    input:checked + .slider {
        background-color: var(--selection-color);
    }
    input:checked + .slider:before {
        transform: translateX(26px);
    }
    .right-float {
        float: right;
        display: flex;
        align-items: center;
        padding-right: 10px;
    }
</style>