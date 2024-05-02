<script lang="ts">
    import { onMount } from 'svelte';
    import { EditorView, keymap, lineNumbers } from '@codemirror/view';
    import { EditorState } from '@codemirror/state';
    import { defaultKeymap } from '@codemirror/commands';
    import { javascript } from '@codemirror/lang-javascript';

    let editor: EditorView;

    onMount(() => {
        const state = EditorState.create({
            doc: 'console.log("Hello, World!");',
            extensions: [
                keymap.of(defaultKeymap),
                lineNumbers(),
                javascript(),
            ],
        });

        editor = new EditorView({
            state,
            parent: document.getElementById('editor'),
        });
    });
</script>

<div id="editor"></div>

<style>
    #editor {
        height: 400px;
    }
</style>