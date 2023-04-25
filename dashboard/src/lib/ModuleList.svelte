<style>
.moduleList {
  background-color: rgb(51, 49, 62);
  margin-top: 1rem;
  border-radius: 1rem;
}
</style>

<script lang="ts">
import { onMount } from "svelte";
import { ApiResponse, type ApiModuleType, apiUrl } from "../ApiTypes";
import { userJwt } from "../auth";
import Module from "./Module.svelte";
import { userModules } from "../modules";

const fetchModules = async () => {
  const result = await fetch(`${apiUrl}/user/modules`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${$userJwt}`,
    },
  });

  const data = await result.json();
  $userModules = ApiResponse.parse(data).modules;
};

$: if ($userModules.length == 0) {
  fetchModules().catch((e) => console.log(e));
}
</script>

<div class="moduleList">
  {#each $userModules as module}
    <Module
      name="{module.module_hash}"
      functions="{module.functions}"
      moduleId="{module.id}" />
  {/each}
</div>
