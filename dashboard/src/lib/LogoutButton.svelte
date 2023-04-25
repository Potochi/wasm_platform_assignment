<style>
.logoutButton {
  background-color: rgb(172, 158, 70);
}
</style>

<script lang="ts">
import { userJwt } from "../auth";

const handleClick = async () => {
  $userJwt = "";
};
</script>

<button class="logoutButton" on:click="{handleClick}">Logout</button>
