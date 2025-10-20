async function fetchPosts() {
    const url = "http://127.0.0.1:5500/posts/posts.json";

    try {
        const response = await fetch(url)
        
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        const posts_json = await response.json(); // Parsing JSON

        console.log(posts_json.length);

        let i = 0;
        let posts = []
        while (i < posts_json.length) {
            console.log(posts_json[i]);
            posts.push(posts_json[i]);
            i += 1;
        }

        return posts;
    } catch (error) {
        console.error(error.message);
        return false;
    }
}

let posts = fetchPosts(); // Get posts, then display posts
console.log(posts);