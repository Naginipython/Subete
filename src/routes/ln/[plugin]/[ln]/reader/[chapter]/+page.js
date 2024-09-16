export const prerender = false;
export function load({ params }) {
    return {
        id: params.ln,
        ln_index: params.chapter
    }
}
