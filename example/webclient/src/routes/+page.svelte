<script lang="ts">
  import init, { Hello } from "rust-idl";
  import { onMount } from "svelte";

  const WEBSERVER_URL = "http://localhost:8000";

  let loaded = false;
  let payload = new Uint8Array();
  let accept = "application/json";
  let name = "";
  let hello: Hello | null = null;

  const greet = async (name: string): Promise<Hello> => {
    const headers = new Headers();
    headers.set("Accept", accept);
    const result = await fetch(`${WEBSERVER_URL}/hello/${name}`, { headers });
    console.log(`Response status: ${result.status}`);

    payload = new Uint8Array(await result.arrayBuffer());
    console.log("using accept header: ", accept);
    switch (accept) {
      case "application/json":
        return Hello.from_json(payload);
      case "application/msgpack":
        return Hello.from_msgpack(payload);
      default:
        throw new Error(`Unsupported accept header: ${accept}`);
    }
  };

  onMount(async () => {
    await init();
    loaded = true;
  });
</script>

<h1>Ridl Demo</h1>

{#if !loaded}
  <p>Loading...</p>
{:else}
  <div>
    <select bind:value={accept}>
      <option value="application/json">application/json</option>
      <option value="application/msgpack">application/msgpack</option>
    </select>
  </div>

  <div>
    <input type="text" bind:value={name} placeholder="Enter your name" />
    <button on:click={async () => (hello = await greet(name))}>Greet</button>
  </div>

  <div>
    {#if payload.byteLength > 0}
      <p>Raw payload: {payload.join(", ")}</p>
    {/if}
    {#if hello}
      <p>Decoded name = {hello.name}</p>
    {:else}
      <p>Enter your name and click the button above to be greeted</p>
    {/if}
  </div>
{/if}
