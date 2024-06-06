<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let folder:string = "C:\\Users\\clccmh";
  let isRegexValid:boolean = true;
  let isFolderValid:boolean = true;

</script>

<div class="h-screen flex flex-col">
  <div class="bg-neutral-800 pb-4 px-3 shadow">
    <div class="w-full px-3 my-3 flex flex-row justify-center items-center">
      <p class="mr-3 text-neutral-100 text-xl">Folder:</p>
      <input class="flex-grow bg-neutral-600 text-neutral-100 px-3 py-2 rounded-lg"
             value="{folder}" />
      <button class="absolute right-20 text-neutral-100 bg-blue-400 px-2 py-0.5 mr-[-0.5em] rounded text-lg font-semibold
                     hover:bg-blue-300 active:bg-blue-500 shadow">
        Change Directory
      </button>
      {#if isFolderValid}
        <p class="bg-green-400 rounded-full text-white px-2 text-xl font-bold absolute right-8">&#x2713;</p>
      {:else}
        <p class="bg-red-400 rounded-full text-white px-2 text-xl font-bold ml-3">&#x2717;</p>
      {/if}
    </div>

    <div class="w-full px-3 flex flex-row justify-center items-center">
      <p class="mr-3 text-neutral-100 text-xl">Regex:</p>
      <input class="flex-grow bg-neutral-600 text-neutral-100 px-3 py-2 rounded-lg"
             placeholder="(.*)/$1" />
      {#if isRegexValid}
        <p class="bg-green-400 rounded-full text-white px-2 text-xl font-bold absolute right-8">&#x2713;</p>
      {:else}
        <p class="bg-red-400 rounded-full text-white px-2 text-xl font-bold ml-3">&#x2717;</p>
      {/if}
    </div>
  </div>

  <div class="flex-grow ml-4 pr-4 py-4">
    <div class="flex flex-col h-full">
      <div class="flex-grow-0 flex-shrink-0 flex flex-row text-center justify-center bg-neutral-800 rounded-t-xl py-2">
        <h1 class="text-2xl font-semibold text-neutral-100 w-1/2">Original</h1>
        <h1 class="text-2xl font-semibold text-neutral-100 w-1/2">Renamed</h1>
      </div>
      <div class="flex-grow flex-shrink rounded-b-xl bg-neutral-800 p-2 pt-0 h-40">
        <div class="h-full overflow-y-scroll scrollbar scrollbar-track-neutral-700 scrollbar-thumb-neutral-600 bg-neutral-800 rounded-b-xl relative top-0 bottom-0">
          <table class="my-1 text-neutral-100 w-full bg-neutral-800 pt-1
                        border-separate border-spacing-2 border-spacing-y-1">
            <tr>
              <td class="px-2 bg-neutral-700 text-sm m-6 rounded w-1/2">One</td>
              <td class="px-2 bg-neutral-700 text-sm m-6 rounded w-1/2">ONE</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
            <tr>
              <td class="px-2 bg-neutral-700 text-sm rounded">Two</td>
              <td class="px-2 bg-neutral-700 text-sm rounded">TWO</td>
            <tr>
          </table>
        </div>
      </div>
    </div>
  </div>

  <div class="w-full py-2 px-2 flex flex-row items-center bg-neutral-800 shadow justify-center">
    <a class="bg-pink-500 rounded shadow px-3 py-2 text-neutral-100 font-bold
              border-none cursor-pointer hover:bg-pink-400 hover:shadow-lg
              active:bg-pink-600 active:shadow-none"
              href="#">
              Rename!</a>
  </div>

</div>

<style>

</style>
