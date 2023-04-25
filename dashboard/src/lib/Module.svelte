<style>
.moduleContainer {
  display: grid;
  grid-template-columns: auto;
  grid-auto-rows: auto;
  padding: 0;
  margin: 1rem;
}

.fullRow {
  grid-column: 1;
  width: 100%;
}

ul.functionList {
  list-style-type: none;
  margin: 0;
  padding: 0;
}

.moduleHeader {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  justify-content: space-between;
  align-items: center;
}

.deleteButton {
  background-color: rgb(125, 31, 31);
}

.moduleName {
}
</style>

<script lang="ts">
import Function from "./Function.svelte";
import { apiUrl, type ApiFunctionType } from "../ApiTypes";
import { userJwt } from "../auth";
import { userModules } from "../modules";

export let name: string;
export let moduleId: number;
export let functions: ApiFunctionType[] = [];

const handleRemove = async () => {
  const result = await fetch(`${apiUrl}/module/delete/${moduleId}`, {
    method: "DELETE",
    headers: {
      Authorization: `Bearer ${$userJwt}`,
    },
  });

  console.log(result);

  $userModules = [];
};
</script>

<div class="moduleContainer">
  <div class="fullRow moduleHeader">
    <h3 class="moduleName">{name}</h3>
    <button class="deleteButton" on:click="{handleRemove}">Remove</button>
  </div>
  <hr class="fullRow" />
  <ul class="functionList">
    {#each functions as func, idx}
      <li>
        <Function
          name="{func.function}"
          signature="{func.signature}"
          modId="{moduleId}" />
        {#if idx < functions.length - 1}
          <hr class="fullRow" />
        {/if}
      </li>
    {/each}
  </ul>
</div>
