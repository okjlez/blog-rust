<script lang="ts">
    import Cookies from 'js-cookie';
    import { redirect } from '@sveltejs/kit';
    import type { PageServerData } from './$types';

    let show = Cookies.get("sid") != null;
    /*
    <form id="login" action="http://127.0.0.1:8000/api/account/login" method="POST" >
    <input type="text" id="email" name="email" placeholder="Email"><br><br>
    <input type="password" id="password" name="password" placeholder="Password"><br><br>
    <input type="submit" value="Submit">
</form> 
    */
</script>

<h1>Thread Creation Page</h1>
<p>Make a Thread</p>

{#if show == true}
<form id="thread" action="http://127.0.0.1:8000/api/thread/new" method="POST" >
    <input type="text" id="title" name="title" placeholder="title"><br><br>
    <input type="body" id="body" name="body" placeholder="body"><br><br>
    <input type="submit" value="Submit">
</form> 
    */
{/if}

