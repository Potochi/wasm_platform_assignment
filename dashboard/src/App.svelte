<style>
.topBar {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>

<script lang="ts">
import { userJwt } from "./auth";
import Login from "./lib/Login.svelte";
import ModuleList from "./lib/ModuleList.svelte";
import ModuleUpload from "./lib/ModuleUpload.svelte";
import LogoutButton from "./lib/LogoutButton.svelte";
import jwt_decode from "jwt-decode";

let username = "";
$: if ($userJwt.length > 0) {
  username = jwt_decode($userJwt)["sub"];
}
</script>

<main>
  {#if $userJwt == ""}
    <Login />
  {:else}
    <div class="topBar">
      <h2>Hello {username}.</h2>
      <LogoutButton />
    </div>
    <hr />
    <br />
    <ModuleUpload />
    <br />
    <ModuleList />
  {/if}
</main>
