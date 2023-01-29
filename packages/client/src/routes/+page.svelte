<script lang="ts">
    import Cookies from 'js-cookie';

    let show = Cookies.get("sid") != null;

    //prerender before page...
</script>

<h1>Simple login</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>


{#if show == true}
<h2>You're logged in welcome!</h2>
<ul style="width:120px;">
    <a href="http://127.0.0.1:5173/create_thread">create thread</a><br>
    <a href="http://127.0.0.1:8000/api/account/logout">logout</a>
</ul>
{/if}

{#if show == false}

<form id="login" action="http://127.0.0.1:8000/api/account/login" method="POST" >
    <input type="text" id="email" name="email" placeholder="Email">
    <input type="password" id="password" name="password" placeholder="Password"><br>
    <input type="submit" value="Submit">
</form> 
{/if}
