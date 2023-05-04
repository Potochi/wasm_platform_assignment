<style>
.resultText {
  flex-basis: 100%;
  text-align: center;
}
.functionContainer {
  display: grid;
  grid-template-columns: 33% 33% 33%;
  align-items: center;
  column-gap: 1px;
  row-gap: 1px;
}

.callButton {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  flex-direction: row;
}

.paramInput {
  margin: 0;
  padding: 0;
  resize: none;
  text-align: right;
  max-width: 12rem;
}

.inlineDesc {
  white-space: nowrap;
  display: flex;
  overflow: auto;
  margin-top: 0.2rem;
  margin-bottom: 0.2rem;
  justify-content: space-between;
}

.paramLabel {
  margin-right: 0.5rem;
  max-width: fit-content;
  color: rgb(158, 158, 158);
}

.actualButton {
  background-color: rgb(49, 114, 49);
}
</style>

<script lang="ts">
import { apiUrl, FunctionResult } from "../ApiTypes";
import { userJwt } from "../auth";
import { fly } from "svelte/transition";

export let name: string;
export let signature: string;
export let modId: number;

let parameters = signature.split("->")[0].split(",");

if (parameters[0] == "") {
  parameters = [];
}

let timeout: number;
let return_value = "";

const values = Array<number>(parameters.length).fill(0);
let visible = false;

const callFunction = async () => {
  try {
    const result = await fetch(
      `${apiUrl}/function/call/${encodeURIComponent(
        modId
      )}/${encodeURIComponent(name)}`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${$userJwt}`,
        },
        body: JSON.stringify({ params: values }),
      }
    );

    const data = await result.json();

    if (result.status == 200) {
      return_value = JSON.stringify(FunctionResult.parse(data).return_value);
    } else {
      return_value = JSON.stringify(data);
    }

    visible = true;

    clearTimeout(timeout);
    timeout = setTimeout(() => {
      visible = false;
    }, 2000);
  } catch {
    console.log("error in function");
  }
};
</script>

<div class="functionContainer">
  <h4>{name}</h4>
  <div>
    {#each parameters as param, idx}
      <div class="inlineDesc">
        <label class="paramLabel" for="{idx.toString()}"
          >Param #{idx} ({param})</label>
        <input
          bind:value="{values[idx]}"
          class="paramInput"
          type="number"
          id="{idx.toString()}" />
      </div>
      <hl></hl>
    {/each}
  </div>
  <div class="callButton">
    {#if visible}
      <h5 class="resultText" transition:fly="{{ y: 20 }}">
        {return_value}
      </h5>
    {/if}
    <button on:click="{callFunction}" class="actualButton">Call</button>
  </div>
</div>
