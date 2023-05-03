<style>
.msgBox {
  background-color: rgb(93, 93, 44);
  text-align: center;
  padding: 1rem 1rem 1rem 1rem;
  border-radius: 1rem;
}
</style>

<script lang="ts">
import { userJwt } from "../auth";
import { userModules } from "../modules";
import { LoginReponse, apiUrl } from "../ApiTypes";
import { fly } from "svelte/transition";

let username = "";
let password = "";

let error = "";

const handleSubmit = async () => {
  const result = await fetch(`${apiUrl}/auth/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ username: username, password: password }),
  });

  const data = await result.json();

  if (result.status == 200) {
    const jwt = LoginReponse.parse(data);
    $userJwt = jwt.jwt;
    return;
  }

  $userModules = [];

  error = "Login failed";
};

const handleRegister = async () => {
  const result = await fetch(`${apiUrl}/auth/register`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ username: username, password: password }),
  });

  if (result.status != 200) {
    error = "Registration failed";
  } else {
    await handleSubmit();
  }
};

let errorTimeout: number;

$: if (!!error) {
  clearTimeout(errorTimeout);
  errorTimeout = setTimeout(() => {
    error = "";
  }, 5000);
}
</script>

<div>
  {#if error.length > 0}
    <div style="position: absolute; left: 50%; top: 0;">
      <div
        class="msgBox"
        style="position: relative; left: -50%;"
        transition:fly="{{ y: -100 }}">
        {error}
      </div>
    </div>
  {/if}
  <h1>Authenticate</h1>
  <form on:submit|preventDefault="{handleSubmit}">
    <label>
      Username:
      <input type="text" bind:value="{username}" placeholder="..." />
    </label>

    <label>
      Password:
      <input type="password" bind:value="{password}" placeholder="..." />
    </label>

    <button type="submit"> Log in </button>
    <button type="button" on:click="{handleRegister}"> Register </button>
  </form>
</div>
