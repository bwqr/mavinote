<script lang="ts">
    import Plus from '$components/icons/plus.svelte';
</script>

<div class="d-flex justify-content-center align-items-center flex-column h-100">
    <div id="note-add" class="w-100">
        <Plus class="w-100 mb-5 h-auto text-black-50"/>
    </div>
    <p class="text-black-50 text-center">Start taking notes by first creating a folder, then select the folder you have created</p>
</div>

<style lang="scss">
    #note-add {
        max-width: 50%;
    }

    @media(min-width: 800px) {
        #note-add {
            max-width: 30%;
        }
    }
</style>
