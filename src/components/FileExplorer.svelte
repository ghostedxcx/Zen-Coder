<script lang="ts">
  import { onMount } from 'svelte';
  import { open, save } from '@tauri-apps/api/dialog';
  import { readTextFile, writeTextFile, readDir, FileEntry } from '@tauri-apps/api/fs';
  import { EditorView, keymap, lineNumbers } from '@codemirror/view';
  import { EditorState } from '@codemirror/state';
  import { defaultKeymap, indentWithTab } from '@codemirror/commands';
  import { javascript } from '@codemirror/lang-javascript';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { languages } from '@codemirror/language-data';
  import { indentService } from '@codemirror/language';

  let currentFile: string | null = null;
  let editor: EditorView;
  let editorContainer: HTMLDivElement;
  let sidebarOpen = true;
  let files: FileEntry[] = [];
  let expandedFolders: Record<string, boolean> = {};

  onMount(async () => {
    editor = new EditorView({
      state: EditorState.create({
        doc: '',
        extensions: [
          keymap.of([...defaultKeymap, indentWithTab]),
          lineNumbers(),
          javascript(),
          oneDark,
          indentService.of((context) => {
            const { state, pos } = context;
            const language = state.languageDataAt(pos);
            if (language.name === 'javascript') {
              return javascript().indentation(state, pos);
            }
            return null;
          }),
          EditorState.tabSize.of(2),
          EditorView.lineWrapping,
        ],
      }),
      parent: editorContainer,
    });

    await loadFiles('.');
  });

  async function loadFiles(path: string) {
    try {
      const entries = await readDir(path);
      files = entries;
    } catch (error) {
      console.error('Failed to load files:', error);
    }
  }

  async function openFile(file: FileEntry) {
    if (file.children) {
      expandedFolders[file.path] = !expandedFolders[file.path];
      if (expandedFolders[file.path]) {
        await loadFiles(file.path);
      }
    } else {
      try {
        currentFile = file.path;
        const content = await readTextFile(file.path);
        editor.dispatch({
          changes: {
            from: 0,
            to: editor.state.doc.length,
            insert: content,
          },
        });
      } catch (error) {
        console.error('Failed to open file:', error);
      }
    }
  }

  async function saveFile() {
    if (currentFile) {
      await writeTextFile(currentFile, editor.state.doc.toString());
      console.log('File saved successfully.');
    } else {
      const filePath = await save();
      if (filePath) {
        currentFile = filePath;
        await writeTextFile(filePath, editor.state.doc.toString());
        console.log('File saved successfully.');
      }
    }
  }

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }

  function renderFileTree(entries: FileEntry[], depth: number = 0) {
    return entries.map((entry) => {
      if (entry.children) {
        return `
          <li>
            <div class="cursor-pointer hover:bg-gray-700 py-1 px-2" style="padding-left: ${depth * 20}px" on:click={() => openFile(entry)}>
              ${expandedFolders[entry.path] ? '▼' : '▶'} ${entry.name}
            </div>
            ${expandedFolders[entry.path] ? `<ul>${renderFileTree(entry.children, depth + 1)}</ul>` : ''}
          </li>
        `;
      } else {
        return `
          <li>
            <div class="cursor-pointer hover:bg-gray-700 py-1 px-2" style="padding-left: ${depth * 20}px" on:click={() => openFile(entry)}>
              ${entry.name}
            </div>
          </li>
        `;
      }
    }).join('');
  }
</script>

<div class="flex min-h-screen bg-gray-900 text-white">
  <!-- Sidebar -->
  <div class="w-64 bg-gray-800 p-4 transition-all duration-300 {sidebarOpen ? 'ml-0' : '-ml-64'}">
    <h2 class="text-xl font-bold mb-4">Files</h2>
    <ul>
      {@html renderFileTree(files)}
    </ul>
  </div>

  <div class="flex-1 flex flex-col">
    <!-- Toolbar -->
    <div class="flex items-center justify-between px-4 py-2 bg-gray-800">
      <div class="flex items-center">
        <button class="p-2 rounded hover:bg-gray-700 focus:outline-none" on:click={toggleSidebar}>
          <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
        </button>
        <button class="p-2 rounded hover:bg-gray-700 focus:outline-none" on:click={saveFile} disabled={!currentFile}>
          <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
          </svg>
        </button>
      </div>
      <div>
        <!-- Dropdown menu or additional options -->
      </div>
    </div>

    <!-- Editor area -->
    <div class="flex-1 overflow-hidden">
      <div class="h-full" bind:this={editorContainer}></div>
    </div>

    <!-- Status bar -->
    <div class="px-4 py-1 bg-gray-800 text-sm">
      <span>{currentFile}</span>
    </div>
  </div>
</div>

<style global>
  .cm-content {
    text-align: left;
  }

  .cm-scroller {
    height: 100%;
  }
</style>
