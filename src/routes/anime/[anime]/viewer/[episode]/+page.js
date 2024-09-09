export const prerender = false;
export function load({ params }) {
    return {
        id: params.anime,
        anime_index: params.episode
    }
}
