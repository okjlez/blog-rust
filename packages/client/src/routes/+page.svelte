<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<form action="http://127.0.0.1:8000/api/account/login" method="POST">
    <label for="fname">Email</label>
    <input type="text" id="email" name="email"><br><br>
    <label for="lname">Password:</label>
    <input type="text" id="password" name="password"><br><br>
    <input type="submit" value="Submit">
</form> 