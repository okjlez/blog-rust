<script lang="ts">
    import Cookies from 'js-cookie';
    import { redirect } from '@sveltejs/kit';
    import type { PageServerData } from './$types';

    let show = Cookies.get("sid") != null;
    
    /** @type {import('./$types').LayoutServerLoad} */
    export function load({ locals = Cookies.get("sid") != null }) {
        if (locals == false) {
        // Correct!
        throw redirect(307, '/');
        }
    }

</script>

<h1>Thread Creation Page</h1>
<p>Make a Thread</p>

{#if show == true}
<form id="login" action="http://127.0.0.1:8000/api/thread/new" method="POST" >
    <input type="text" id="title" name="title" placeholder="Title"><br>
    <textarea id="body" name="body" rows="4" cols="50" placeholder="Enter content here."></textarea><br>
    <input style="text-center" type="submit" value="submit">
</form> 
{/if}

