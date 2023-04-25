<script lang="ts">
import { userJwt } from "../auth";
import { userModules } from "../modules";
import { apiUrl } from "../ApiTypes";

let files: FileList;

const handleUpload = async () => {
  for (const file of files) {
    const result = await fetch(`${apiUrl}/module/deploy`, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${$userJwt}`,
      },
      body: file,
    });

    if (result.status === 201) {
      console.log(`${file.name} uploaded ok`);
    } else {
      console.log(`${file.name} failed to upload`);
    }
  }

  $userModules = [];
};
</script>

<form on:submit|preventDefault="{handleUpload}">
  <label for="new-module" style="font-size: large">Upload a module:</label>
  <input
    accept="application/wasm"
    bind:files="{files}"
    id="new-module"
    name="new-module"
    type="file" />
  <input type="submit" value="Upload" />
</form>
