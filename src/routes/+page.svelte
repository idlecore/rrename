<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";

  let folder: string = "";
  let files: string[] = [];
  let renamedFiles: string[] = [];
  let isRegexValid: boolean = true;
  let isRenameRegexValid: boolean = true;
  let isFolderValid: boolean = true;
  let fileRegex: string = "^.*$";
  let replacementRegex: string = "(.*)";
  let replacementString: string = "$1";
  let isReportOpen: boolean = ""
  let report: string = ""

  let scrollbar = "scrollbar scrollbar-track-neutral-700 scrollbar-thumb-neutral-600";

  let clear_all = function() {
    fileRegex = "^.*$";
    replacementRegex = "(.*)";
    replacementString = "$1";
  };


  invoke("get_cwd").then((wd) => {
      folder = (wd as string);
      update_files();
  });


  let update_regex = function() {
      invoke("test_file_regex", {re: fileRegex})
        .then((res) => {
            isRegexValid = (res as boolean);
            if (isRegexValid) {
                update_files();
            }
        });
  };

  let update_files = function() {
      invoke("get_files", {dir: folder, re: fileRegex})
          .then((res) => {
              files = (res as string[]);
              isFolderValid = true;
              process_replacements();
          })
          .catch((_) => {
              isFolderValid = false;
              files = [];
              renamedFiles = [];
          });
  };


  let process_replacements = function() {
    invoke("process_replacements", {files: files, re: replacementRegex, out: replacementString})
        .then((res) => {
            renamedFiles = (res as string[]);
            isRenameRegexValid = true;
        })
        .catch((_) => {
            renamedFiles = []
            isRenameRegexValid = false;
        });
  }

  let dir_select = async function() {
    const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: folder,
    });
    if (selected != null) {
        folder = (selected as string);
    }
    clear_all();
    update_files();
  }

  let rename = function() {
    invoke("rename", {dir: folder, original: files, renamed: renamedFiles})
    .then((res) => {
        (res as string[]).forEach((line: string) => {
            report += line;
            report += "\n";
        });
        replacementRegex = "(.*)";
        replacementString = "$1";
        isReportOpen = true;
        update_files();
    });
  }


</script>

<div class="h-screen flex flex-col">
  <div class="bg-neutral-800 pb-4 px-3 shadow">
    <div class="w-full px-3 my-3 flex flex-row justify-center items-center">
      <p class="mr-3 text-neutral-100 text-lg">Folder:</p>
      <input class="flex-grow bg-neutral-600 text-neutral-100 px-2 py-1 rounded-lg"
             bind:value="{folder}"
             on:input={update_files}/>
      <button class="absolute right-20 text-neutral-100 bg-blue-400 px-2 py-0.5 mr-[-0.5em] rounded font-semibold
                     hover:bg-blue-300 active:bg-blue-500 shadow"
                     on:click={dir_select} >
        Change Directory
      </button>
      {#if isFolderValid}
        <p class="bg-green-400 rounded-full text-white px-2 font-bold absolute right-8">&#x2713;</p>
      {:else}
        <p class="bg-red-400 rounded-full text-white px-2 font-bold ml-3">&#x2717;</p>
      {/if}
    </div>

    <div class="w-full px-3 flex flex-row justify-center items-center pb-4">
      <p class="mr-3 text-neutral-100 text-lg">File Regex:</p>
      <input class="flex-grow bg-neutral-600 text-neutral-100 px-2 py-1 rounded-lg"
             bind:value={fileRegex} 
             on:input={update_regex} />
      {#if isRegexValid}
        <p class="bg-green-400 rounded-full text-white px-2 font-bold absolute right-8">&#x2713;</p>
      {:else}
        <p class="bg-red-400 rounded-full text-white px-2 font-bold ml-3">&#x2717;</p>
      {/if}
    </div>
    <div class="w-full px-3 flex flex-row justify-center items-center">
      <p class="mr-3 text-neutral-100 text-lg">Rename Regex:</p>
      <input class="flex-grow bg-neutral-600 text-neutral-100 px-2 mr-1 py-1 rounded-lg"
             bind:value={replacementRegex}
             on:input={process_replacements}/>
      <input class="flex-grow bg-neutral-600 text-neutral-100 px-2 ml-1 py-1 rounded-lg"
             bind:value={replacementString}
             on:input={process_replacements}/>
      {#if isRenameRegexValid}
        <p class="bg-green-400 rounded-full text-white px-2 font-bold absolute right-8">&#x2713;</p>
      {:else}
        <p class="bg-red-400 rounded-full text-white px-2 font-bold ml-3">&#x2717;</p>
      {/if}
    </div>
  </div>

  <div class="flex-grow ml-4 pr-4 py-4">
    <div class="flex flex-col h-full">
      <div class="flex-grow-0 flex-shrink-0 flex flex-row text-center justify-center bg-neutral-800 rounded-t-xl py-1">
        <h1 class="text-xl font-semibold text-neutral-100 w-1/2">Original</h1>
        <h1 class="text-xl font-semibold text-neutral-100 w-1/2">Renamed</h1>
      </div>
      <div class="flex-grow flex-shrink rounded-b-xl bg-neutral-800 p-2 pt-0 h-40">
        <div class="h-full overflow-y-scroll {scrollbar} bg-neutral-800 rounded-b-xl relative top-0 bottom-0">
          <table class="my-1 text-neutral-100 w-full bg-neutral-800 pt-1
                        border-separate border-spacing-2 border-spacing-y-1">
            {#each files.map(function(e, i) {return [e, renamedFiles[i]]}) as file}
                <tr>
                  <td class="px-2 bg-neutral-700 text-sm m-6 rounded w-1/2">{file[0]}</td>
                  <td class="px-2 bg-neutral-700 text-sm m-6 rounded w-1/2">{file[1]}</td>
                <tr>
            {/each}
          </table>
        </div>
      </div>
    </div>
  </div>

  <div class="w-full py-2 px-2 flex flex-row items-center bg-neutral-800 shadow justify-center">
    <button class="bg-pink-500 rounded shadow px-3 py-2 text-neutral-100 font-bold
              border-none cursor-pointer hover:bg-pink-400 hover:shadow-lg
              active:bg-pink-600 active:shadow-none"
              on:click={rename}>
              Rename!</button>
  </div>

<!-- Report Screen -->

    {#if isReportOpen}
    <div class="absolute w-full h-full bg-neutral-800">
        <h1 class="text-2xl text-center font-bold my-3 text-neutral-100">Rename Complete!</h1>
        <p class="rounded-lg shadow-md bg-neutral-700 mx-4 px-3 py-3 overflow-y-scroll {scrollbar} text-neutral-200 max-h-[75%]">
            {report}
        </p>
        <div class="text-center my-3">
            <button class="bg-blue-400 rounded shadow px-3 py-2 text-neutral-100 font-bold"
                on:click={() => isReportOpen = false}
            >
                Close
            </button>
        </div>
    </div>
    {/if}

</div>

<style>

</style>
