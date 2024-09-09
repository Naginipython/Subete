export const prerender = false;
export function load({ params }) {
    return {
        id: params.manga,
        plugin: params.plugin,
        manga_index: params.chapter
    }
}
