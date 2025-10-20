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
        while (i < posts_json.length) { // Unwrap promise into usable data
            console.log(posts_json[i]);
            posts.push(posts_json[i]);
            i += 1;
        }; // Posts aquired!

        const range = getPostRange(page, page_length, posts.length);

        console.log(posts[0]);

        formatPostsHome(range[0], range[1], posts, posts_window);

        loading.style.display = "none";
    } catch (error) {
        console.error(error.message);
    }
}

function getPostRange(page, page_length, posts_length) { //TODO Test maximum with 11+ posts -> maths doesn't make sense but it is working rn
    let maximum = posts_length - (page-1); // ?? = 10 - (0-1);
    if (maximum > posts_length) { // True
        maximum = posts_length - 1; // = 0;
    }
    let minimum = posts_length - ((page+1) * page_length); // -9 = 1 - 0*10;
    console.log("post length = " + posts_length + " page = " + page + "page_length = " + page_length);
    if (minimum < 0) { // True
        minimum = 0; // = 0;
    };

    return [minimum, maximum];
}

function formatPostsHome(id_minimum, id_maximum, posts) {
    let id = id_minimum;
    const loading = document.getElementById("loading");

    ////console.log("running formatPostsHome");
    ////console.log("minimum = " + id_minimum + " maximum = " + id_maximum)

    while (id <= id_maximum) {
        console.log("Creating post.");
        /*
        Example structure:
        <div id="example_post" class="post">
            <a href="post.html" class="post">
                <div class="post_thumbnail">
                    <img src="default.svg">
                </div>
                <div class="post_text">
                    <div class="post_title">
                        <b>Example post about topic</b>
                    </div>
                    <div class="post_description">
                        Example description of the topic
                    </div>
                </div>
            </a>
        </div>

        Simplified structure:
        Outer wrapper
            Link wrapper
                Image wrapper
                    Image
                Text wrapper
                    Title
                    Description
        */

        const outer_wrapper = document.createElement("div");
        outer_wrapper.id = id;
        outer_wrapper.className = "post";

        const link_wrapper = document.createElement("a");
        link_wrapper.href = "post.html"; //TODO link to a page with the actual post.
        link_wrapper.className = "post";

        const image_wrapper = document.createElement("div");
        image_wrapper.className = "post_thumbnail";

        const image = document.createElement("img");
        image.src = posts[id].thumbnail;

        const text_wrapper = document.createElement("div");
        text_wrapper.className = "post_text";

        const title = document.createElement("div");
        title.className = "post_title";
        
        const title_bold = document.createElement("b");

        const title_text_node = document.createTextNode(posts[id].title);
        title_bold.appendChild(title_text_node);
        title.appendChild(title_bold);

        const description = document.createElement("div");
        description.className = "post_description";

        const description_text_node = document.createTextNode(posts[id].description);
        description.appendChild(description_text_node);

        image_wrapper.appendChild(image);
        
        text_wrapper.appendChild(title);
        text_wrapper.appendChild(description);

        link_wrapper.appendChild(image_wrapper);
        link_wrapper.appendChild(text_wrapper);

        outer_wrapper.appendChild(link_wrapper);

        posts_window.insertBefore(outer_wrapper, loading);

        id = id + 1;
    };
}

// Setup variables
let page = 0;
let page_length = 10;
const posts_window = document.getElementById("posts");
const loading = document.getElementById("loading");

fetchPosts(); // Get posts

////const range = getPostRange(page, page_length, posts.length) // minimum; maximum [this order]

////formatPostsHome(range[0], range[1], posts, posts_window);