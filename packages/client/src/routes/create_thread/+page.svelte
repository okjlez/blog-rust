<script lang="ts">
    import Cookies from 'js-cookie';

    let show = Cookies.get("sid") != null;

    //prerender before page...
</script>

<h1>Thread Creation Page</h1>
<p>Make a Thread</p>


{#if show == true}
<form id="login" action="http://127.0.0.1:8000/api/thread/new/" method="POST" >
    <input type="text" id="title" name="title" placeholder="Title"><br><br>
    <textarea id="w3review" name="w3review" rows="4" cols="50"></textarea>
    <input type="submit" value="Submit">
</form> 
{/if}

