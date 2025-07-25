import { posts } from "./data.js";

export function load() {
    return {
        summaries: posts.map((post) => ({
            title: post.title,
            slug: post.slug
        }))
    };
}