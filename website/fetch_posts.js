async function fetchPosts() {
    const url = "http://127.0.0.1:5500/posts/posts.json";

    try {
        const response = await fetch(url)
        
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        const posts_json = await response.json(); // Parsing JSON
        return await posts_json;
    } catch (error) {
        console.error(error.message);
        return false;
    }
}

function getPostsForRange(max, min) {
    console.log(max);
    console.log(min);
}

posts_json = fetchPosts();

console.log(posts_json);

console.log(posts_json.message)