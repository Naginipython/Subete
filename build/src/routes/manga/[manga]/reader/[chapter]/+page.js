export function load({ params }) {
    return {
        id: params.manga,
        manga_index: params.chapter
    }
}
